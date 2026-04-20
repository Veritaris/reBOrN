import dataclasses
import inspect
import re
import urllib.request
from pathlib import Path

from bs4 import BeautifulSoup

cache_path = Path("./java_opcodes.html")
opcodes_rs_file = Path("./src/opcodes.rs")
lib_rs_file = Path("./src/lib.rs")
opcodes_rs_mod = "pub mod opcodes;\n"
opcode_spec_pattern = re.compile(r"([0-9a-z_]*?)(</em></span>)? = \d+ \((0x[0-9a-f]+)\)")
opcode_string_pattern = re.compile(r"(?P<opname>\w+) = \d+ \((?P<opcode>0x[0-9a-f]+)\)")
tab = "    "
opcodes_enum_struct_rs = """
#[non_exhaustive]
pub struct Opcodes;
"""
opcode_struct_rs = """
pub struct Opcode {
    pub opcode: u8,
    pub opname: &'static str,
    pub oplen: u8,
}
"""
opcodes_enum_struct_impl_head_rs = """
#[allow(unused)]
impl Opcodes {
"""
opcodes_enum_struct_impl_tail_rs = """
}
"""
opcodes_byte_map_head_rs = """
pub const OPCODES_MAP: [Option<Opcode>; 255] = [
"""
opcodes_byte_map_tail_rs = """
];
"""


@dataclasses.dataclass(kw_only=True)
class Opcode:
    opname: str
    opcode: int
    oplen: int
    desc: str
    stack_pop: int
    stack_push: int

    def into_rs(self) -> str:
        return f"Opcode {{ opcode: 0x{self.opcode:02x}, opname: \"{self.opname}\", oplen: {self.oplen} }}"


def download_jvm_opcodes_spec(jvm_version: int) -> str:
    req = urllib.request.urlopen(
        url=f"https://docs.oracle.com/javase/specs/jvms/se{jvm_version}/html/jvms-6.html",
        timeout=60,
    )
    return req.read().decode("utf-8")


def fetch_jvm_opcodes_spec(jvm_version: int = 23) -> str:
    if not cache_path.exists():
        print(f"Fetching opcodes for java se{jvm_version}")
        spec = download_jvm_opcodes_spec(jvm_version)
        with cache_path.open(mode="w") as f:
            f.write(spec)
    else:
        with cache_path.open(mode="r") as f:
            spec = f.read()
    return spec


def gen_opcodes_structs(opcodes: list[Opcode]) -> str:
    opcodes_rs_code = opcodes_enum_struct_rs + opcode_struct_rs + opcodes_enum_struct_impl_head_rs
    for opcode in opcodes:
        opcodes_rs_code += f"{tab}pub const {opcode.opname.upper()}: Opcode = {opcode.into_rs()};\n"

    opcodes_rs_code += opcodes_enum_struct_impl_tail_rs
    opcodes_rs_code = inspect.cleandoc(opcodes_rs_code)
    return opcodes_rs_code


def gen_opcodes_byte_map(opcodes: list[Opcode]) -> str:
    opcodes_byte_mapping: list[Opcode | None] = [None for _ in range(255)]

    for opcode in opcodes:
        idx = opcode.opcode
        opcodes_byte_mapping[idx] = opcode

    opcodes_byte_map = ""
    opcodes_byte_map += opcodes_byte_map_head_rs

    opcodes_list = []

    for opcode in opcodes_byte_mapping:
        if opcode is None:
            opcodes_list.append(f"{tab}None,")
            continue
        opcodes_list.append(f"{tab}Some(Opcodes::{opcode.opname.upper()}),")

    opcodes_byte_map += "\n".join(opcodes_list)
    opcodes_byte_map += opcodes_byte_map_tail_rs
    return opcodes_byte_map


if __name__ == "__main__":
    opcodes_spec = fetch_jvm_opcodes_spec()
    soup = BeautifulSoup(opcodes_spec, "lxml")
    opcodes_pars = soup.find_all("div", class_="section-execution")
    opcodes: list[Opcode] = []

    for par in opcodes_pars[1:]:
        (_, op_par, op_operands_par, op_forms_par, op_stack_par, *rem) = par.find_all("div", recursive=False)

        opcode_operands = op_operands_par.find_all("span", class_="emphasis")
        opcode_forms = op_forms_par.find_all("p", class_="norm")

        opcode_len = 0
        for opcode_operand in opcode_operands:
            opcode_len += 1

        # skip first example
        for opcode_form in opcode_forms:
            row = opcode_form.text.strip()
            opcode_string_parsed = opcode_string_pattern.match(row).groupdict()
            opcode_name = opcode_string_parsed["opname"]
            opcode_code = int(opcode_string_parsed["opcode"][2:], 16)
            opcode = Opcode(
                opname=opcode_name,
                opcode=opcode_code,
                oplen=opcode_len,
                desc=op_par.text.strip(),
                stack_pop=0,
                stack_push=0,
            )
            opcodes.append(opcode)

    opcodes = list(sorted(opcodes, key=lambda op: op.opname))

    opcodes_rs_code = ""
    opcodes_rs_code += gen_opcodes_structs(opcodes)
    opcodes_rs_code += gen_opcodes_byte_map(opcodes)

    with opcodes_rs_file.open(mode="w") as f:
        f.write(opcodes_rs_code)

    with lib_rs_file.open(mode="r") as f:
        lib_rs = f.readlines()

    if not opcodes_rs_mod in lib_rs:
        lib_rs.append(opcodes_rs_mod)

    with lib_rs_file.open(mode="w") as f:
        for line in lib_rs:
            f.write(line)
