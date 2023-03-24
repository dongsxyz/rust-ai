pub trait SSML: Into<String> {}

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Into<String> for Gender {
    fn into(self) -> String {
        (match self {
            Self::Male => "Male",
            Self::Female => "Female",
        })
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum ResponseType {
    Bytes(Vec<u8>),
    Text(String),
}


#[derive(Debug, Clone)]
pub enum ResponseExpectation {
    Bytes,
    Text,
}

