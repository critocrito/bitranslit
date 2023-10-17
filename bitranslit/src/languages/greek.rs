use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 38] = include!("../standards/el/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 41] =
        include!("../standards/el/pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 18] =
        include!("../standards/el/reverse_specific_mapping.in");
    static ref CODE: &'static str = "el";
}

language_pack!(Greek {
    code: CODE,
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
});
