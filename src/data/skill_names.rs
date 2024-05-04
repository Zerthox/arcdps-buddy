use phf::phf_map;

/// Skill name overrides.
pub static SKILL_OVERRIDES: phf::Map<u32, &'static str> = phf_map! {
    12815u32 => "Lightning Leap Combo",
    22492u32 => "Basilisk Venom",
    31749u32 => "Blood Moon",
    32410u32 => "Hunter's Verdict",
};
