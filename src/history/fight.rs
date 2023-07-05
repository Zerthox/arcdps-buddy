use arcdps::Agent;

#[derive(Debug, Clone)]
pub struct Fight<T> {
    pub id: Option<u32>,
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub data: T,
}

impl<T> Fight<T> {
    pub fn new(start: u64, data: T) -> Self
    where
        T: Default,
    {
        Self {
            id: None,
            name: Self::DEFAULT_NAME.into(),
            start,
            end: 0,
            data,
        }
    }

    pub fn with_default(start: u64) -> Self
    where
        T: Default,
    {
        Self::new(start, T::default())
    }

    const DEFAULT_NAME: &str = "Unknown";

    pub fn update_target(&mut self, species: u32, target: Option<&Agent>) {
        if species > 2 {
            self.id = Some(species);
            self.name = match target {
                Some(Agent {
                    name: Some(name), ..
                }) if !name.is_empty() => name.to_string(),
                _ => Self::DEFAULT_NAME.into(),
            };
        } else {
            self.id = None;
            self.name = Self::DEFAULT_NAME.into();
        }
    }

    pub fn duration(&self) -> Option<u64> {
        if self.end != 0 {
            Some(self.end - self.start)
        } else {
            None
        }
    }

    pub fn end(&mut self, time: u64) -> u64 {
        self.end = time;
        self.end - self.start
    }
}
