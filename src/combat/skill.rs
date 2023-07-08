use phf::phf_map;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub name: String,
}

impl Skill {
    pub fn new(id: u32, name: Option<impl Into<String>>) -> Self {
        Self {
            id,
            name: match OVERRIDES.get(&id) {
                Some(name) => name.to_string(),
                None => match name {
                    Some(name) => name.into(),
                    None => id.to_string(),
                },
            },
        }
    }
}

static OVERRIDES: phf::Map<u32, &'static str> = phf_map! {
    22492u32 => "Basilisk Venom",
};
