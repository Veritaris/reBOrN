use std::fmt::{Debug, Display, Formatter};
use std::ops;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(unused)]
pub enum AccessFlagContext {
    Class,
    Method,
    Field,
    Module,
}

#[non_exhaustive]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccessFlags {
    pub value: u16,
    pub context: AccessFlagContext,
}

impl AccessFlags {
    pub const PUBLIC: u16 = 1;
    pub const PRIVATE: u16 = 1 << 1;
    pub const PROTECTED: u16 = 1 << 2;
    pub const STATIC: u16 = 1 << 3;
    pub const FINAL: u16 = 1 << 4;
    pub const SUPER: u16 = 1 << 5;
    pub const SYNCHRONIZED: u16 = 1 << 5;
    pub const BRIDGE: u16 = 1 << 6;
    pub const VOLATILE: u16 = 1 << 6;
    pub const VARARGS: u16 = 1 << 7;
    pub const TRANSIENT: u16 = 1 << 7;
    pub const NATIVE: u16 = 1 << 8;
    pub const INTERFACE: u16 = 1 << 9;
    pub const ABSTRACT: u16 = 1 << 10;
    pub const STRICT: u16 = 1 << 11;
    pub const SYNTHETIC: u16 = 1 << 12;
    pub const ANNOTATION: u16 = 1 << 13;
    pub const ENUM: u16 = 1 << 14;
    pub const MANDATED: u16 = 1 << 15;
    pub const MODULE: u16 = 1 << 15;
}

impl AccessFlags {
    pub fn as_string(&self) -> String {
        <AccessFlags as Into<String>>::into(*self)
    }
}

impl ops::BitOr<AccessFlags> for AccessFlags {
    type Output = AccessFlags;

    fn bitor(self, rhs: AccessFlags) -> Self::Output {
        AccessFlags {
            value: self.value | rhs.value,
            context: self.context,
        }
    }
}

impl Display for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().clone().as_str())
    }
}

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:#06x} {}",
            self.value,
            self.as_string().clone().as_str()
        ))
    }
}

impl From<(AccessFlagContext, u16)> for AccessFlags {
    fn from(value: (AccessFlagContext, u16)) -> Self {
        AccessFlags {
            value: value.1,
            context: value.0,
        }
    }
}

impl Into<u16> for AccessFlags {
    fn into(self) -> u16 {
        self.value
    }
}

impl Into<String> for AccessFlags {
    #[allow(unreachable_patterns)]
    fn into(self) -> String {
        let mut access_specifier = String::new();
        for i in 0..16 {
            access_specifier += match self.value & (1 << i) {
                0 => "",
                AccessFlags::PUBLIC => "public ",
                AccessFlags::PRIVATE => "private ",
                AccessFlags::PROTECTED => "protected ",
                AccessFlags::STATIC => "static ",
                AccessFlags::FINAL => "final ",

                AccessFlags::SUPER | AccessFlags::SYNCHRONIZED => match self.context {
                    AccessFlagContext::Class => "super ",
                    AccessFlagContext::Module => "synchronized ",
                    AccessFlagContext::Method | AccessFlagContext::Field => "",
                },

                AccessFlags::VOLATILE | AccessFlags::BRIDGE => match self.context {
                    AccessFlagContext::Field => "volatile ",
                    AccessFlagContext::Method => "bridge ",
                    AccessFlagContext::Module | AccessFlagContext::Class => "",
                },

                AccessFlags::TRANSIENT | AccessFlags::VARARGS => match self.context {
                    AccessFlagContext::Field => "transient ",
                    AccessFlagContext::Method => "varargs ",
                    AccessFlagContext::Module | AccessFlagContext::Class => "",
                },

                AccessFlags::NATIVE => "native ",
                AccessFlags::INTERFACE => "interface ",
                AccessFlags::ABSTRACT => "abstract ",
                AccessFlags::STRICT => "strict ",
                AccessFlags::SYNTHETIC => "synthetic ",
                AccessFlags::ANNOTATION => "annotation ",
                AccessFlags::ENUM => "enum ",

                AccessFlags::MANDATED | AccessFlags::MODULE => match self.context {
                    AccessFlagContext::Module => "mandated ",
                    AccessFlagContext::Class => "module ",
                    AccessFlagContext::Method | AccessFlagContext::Field => "",
                },
                _ => "unknown access flag",
            }
        }
        String::from(access_specifier.trim())
    }
}
