use strum::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter)]
pub enum Accounts {
    GitHub,
    LinkedIn,
}

impl Accounts {
    pub const fn as_url(&self) -> &'static str {
        match self {
            Self::GitHub => "https://github.com/splurf",
            Self::LinkedIn => "https://www.linkedin.com/in/eschwart",
        }
    }
}
