use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 42] = include!("../standards/bg/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 16] =
        include!("../standards/bg/pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 3] =
        include!("../standards/bg/reverse_specific_mapping.in");
    static ref CODE: &'static str = "bg";
}

language_pack!(Bulgarian {
    code: CODE,
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
});
