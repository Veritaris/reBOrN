#![allow(non_upper_case_globals)]

use std::collections::HashMap;

#[allow(unused)]
const ru_RU: &[u8] = include_bytes!("../../../resources/localisation/ru_RU.lang");
#[allow(unused)]
const en_EU: &[u8] = include_bytes!("../../../resources/localisation/en_EU.lang");

#[allow(unused)]
type LocalesHashMap = HashMap<&'static str, HashMap<&'static str, &'static str>>;

// pub fn get_locales() -> Box<&'static LocalesHashMap> {
// let mut locales: &'static LocalesHashMap = HashMap::<&'static str, HashMap<&'static str, &'static str>>::new();
// let mut locales: &'static LocalesHashMap = &LocalesHashMap::new();
// let mut ru_locale
// Box::new(locales)
// }
pub fn localize(key: &str) -> &str {
    key
}
