use lazy_static::lazy_static;
use transliteratsiya_derive::language_pack;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 52] = include!("../standards/l1/mapping.in");
    static ref REVERSE_SPECIFIC_PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 18] =
        include!("../standards/l1/reverse_specific_pre_processor_mapping.in");
    static ref REVERSE_SPECIFIC_MAPPING: [(&'static str, &'static str); 46] =
        include!("../standards/l1/reverse_specific_mapping.in");
}

language_pack!(Latin1 {
    mapping: MAPPING,
    reverse_specific_pre_processor_mapping: REVERSE_SPECIFIC_PRE_PROCESSOR_MAPPING,
    reverse_specific_mapping: REVERSE_SPECIFIC_MAPPING,
});
