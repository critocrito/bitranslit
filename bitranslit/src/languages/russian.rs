use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 50] = include!("../standards/ru/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 14] =
        include!("../standards/ru/pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 8] =
        include!("../standards/ru/reverse_specific_mapping.in");
}

language_pack!(Russian {
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
});
