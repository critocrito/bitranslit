use lazy_static::lazy_static;
use transliteratsiya_derive::language_pack;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 47] = include!("../standards/hy/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 28] =
        include!("../standards/hy/pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 2] =
        include!("../standards/hy/reverse_specific_mapping.in");
    static ref REVERSE_SPECIFIC_PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 2] =
        include!("../standards/hy/reverse_specific_pre_processor_mapping.in");
}

language_pack!(Armenian {
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
    reverse_specific_pre_processor_mapping: REVERSE_SPECIFIC_PRE_PROCESSOR_MAPPING,
});
