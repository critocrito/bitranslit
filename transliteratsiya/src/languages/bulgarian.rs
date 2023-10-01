use lazy_static::lazy_static;
use transliteratsiya_derive::language_pack;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 61] = include!("../standards/bg.in");
}

language_pack!(Bulgarian { mapping: MAPPING });
