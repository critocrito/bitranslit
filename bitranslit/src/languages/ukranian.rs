use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 46] = include!("../standards/ua/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 20] =
        include!("../standards/ua/pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 2] =
        include!("../standards/ua/reverse_specific_mapping.in");
}

language_pack!(Ukranian {
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
});
