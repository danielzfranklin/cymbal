use std::borrow::Cow;

use addr2line::demangle_auto;

#[derive(Debug)]
pub struct Function {
    pub link_name: String,
    pub name: String,
    pub low_pc: u64,
}

impl Function {
    pub fn new(link_name: String, low_pc: u64) -> Self {
        let name = demangle_auto(Cow::Borrowed(&link_name), Some(gimli::DW_LANG_Rust)).into();

        Self {
            link_name,
            name,
            low_pc,
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.link_name == other.link_name
    }
}

impl Eq for Function {}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Function {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.link_name.cmp(&other.link_name)
    }
}
