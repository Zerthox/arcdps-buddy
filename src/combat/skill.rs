use phf::phf_map;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub name: String,
}

impl Skill {
    pub fn new(id: u32, name: Option<&str>) -> Self {
        Self {
            id,
            name: match OVERRIDES.get(&id) {
                Some(name) => name.to_string(),
                None => match name {
                    Some(name) if !name.is_empty() => name.into(),
                    _ => id.to_string(),
                },
            },
        }
    }
}

static OVERRIDES: phf::Map<u32, &'static str> = phf_map! {
    22492u32 => "Basilisk Venom",
};
