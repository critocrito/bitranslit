use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 54] = include!("../standards/sr/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 6] =
        include!("../standards/sr/pre_processor_mapping.in");
    static ref CODE: &'static str = "sr";
}

language_pack!(Serbian {
    code: CODE,
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
});
