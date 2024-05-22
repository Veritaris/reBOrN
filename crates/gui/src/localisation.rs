#![allow(non_upper_case_globals)]

use std::collections::HashMap;

const ru_RU: &[u8] = include_bytes!("../../../localisation/ru_RU.lang");
const en_EU: &[u8] = include_bytes!("../../../localisation/en_EU.lang");

type LocalesHashMap = HashMap<&'static str, HashMap<&'static str, &'static str>>;

// pub fn get_locales() -> Box<&'static LocalesHashMap> {
//     let mut locales: &'static LocalesHashMap = HashMap::<&'static str, HashMap<&'static str, &'static str>>::new();
//     // let mut ru_locale
//     Box::new(locales)
// }
pub fn localize(key: &str) -> &str {
    key
}