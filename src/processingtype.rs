pub enum T0103 {
    D,
    P,
    T
}

pub enum T0207 {
    A,
    I,
    R,
    T
}

pub type ProcessingMode = Option<T0207>;


impl T0103 {
    pub fn from_str(s: &str) -> Option<Self>{
        match s {
            "D" => Some(T0103::D),
            "P" => Some(T0103::P),
            "T" => Some(T0103::T),
            _ => None
        }
    }
}

impl T0207 {
    pub fn from_str(s: &str) -> Option<Self>{
        match s {
            "A" => Some(T0207::A),
            "I" => Some(T0207::I),
            "R" => Some(T0207::R),
            "T" => Some(T0207::T),
            _ => None
        }
    }
}


pub struct ProcessingType {
    pub processingid: T0103,
    pub processingmode: Option<T0207>,
}

impl ProcessingType {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('^').collect();

        let id = T0103::from_str(parts.get(0)?);

        let mode = parts.get(1).and_then(|v| T0207::from_str(v));

        Some(ProcessingType {
            processingid: id?,
            processingmode: mode,
        })
    }
}




