import re
import urllib.request
from pathlib import Path
import inspect


if __name__ == "__main__":
    cache_path = Path("./java_opcodes.html")
    opcodes_rs_file = Path("./src/opcodes.rs")
    lib_rs_file = Path("./src/lib.rs")
    opcodes_rs_mod = "pub mod opcodes;\n"

    if not cache_path.exists():
        req = urllib.request.urlopen(
            url="https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-6.html",
            timeout=60,
        )
        html = req.read().decode("utf-8")
        with cache_path.open(mode="w") as f:
            f.write(html)
    else:
        with cache_path.open(mode="r") as f:
            html = f.read()

    pattern = re.compile(r"([0-9a-z_]*?)(</em></span>)? = \d+ \((0x[0-9a-f]+)\)")
    opcodes_raw = ((opname, int(opcode[2:], 16)) for (opname, _, opcode) in pattern.findall(html))
    tab = "    "
    opcodes = dict([(opname, opcode) for (opname, opcode) in sorted(opcodes_raw, key=lambda x: x[1])])
    opcodes_rs_code = """
#[non_exhaustive]
pub struct Opcodes;

pub struct Opcode {
    pub opcode: u8,
    pub opname: &'static str,
}

#[allow(unused)]
impl Opcodes {
"""
    for opname, opcode in opcodes.items():
        opcodes_rs_code += f"{tab}pub const {opname.upper()}: Opcode = Opcode {{ opcode: 0x{opcode:02x}, opname: \"{opname}\" }};\n"
    opcodes_rs_code += "}"
    opcodes_rs_code = inspect.cleandoc(opcodes_rs_code)
    with opcodes_rs_file.open(mode="w") as f:
        f.write(opcodes_rs_code)

    with lib_rs_file.open(mode="r") as f:
        lib_rs = f.readlines()

    if not opcodes_rs_mod in lib_rs:
        lib_rs.append(opcodes_rs_mod)

    with lib_rs_file.open(mode="w") as f:
        for line in lib_rs:
            f.write(line)



