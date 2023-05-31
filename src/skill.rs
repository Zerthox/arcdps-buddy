#[derive(Debug, Clone)]
pub struct Skill {
    pub id: u32,
    pub name: String,
}

impl Skill {
    pub fn new(id: u32, name: Option<impl Into<String>>) -> Self {
        Self {
            id,
            name: match name {
                Some(name) => name.into(),
                None => id.to_string(),
            },
        }
    }
}
