use bitranslit_derive::language_pack;
use lazy_static::lazy_static;

lazy_static! {
    static ref MAPPING: [(&'static str, &'static str); 44] = include!("../standards/mk/mapping.in");
    static ref PRE_PROCESSOR_MAPPING: [(&'static str, &'static str); 18] =
        include!("../standards/mk/pre_processor_mapping.in");
}

language_pack!(Makedonian {
    mapping: MAPPING,
    pre_processor_mapping: PRE_PROCESSOR_MAPPING,
});
