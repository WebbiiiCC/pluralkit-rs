use std::fmt::Display;

#[derive(Eq, PartialEq, Hash)]
pub enum RateLimitScope {
    GenericGet,
    GenericUpdate,
    Message,
}

impl Display for RateLimitScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RateLimitScope::GenericGet => "generic_get",
            RateLimitScope::GenericUpdate => "generic_update",
            RateLimitScope::Message => "message",
        };
        write!(f, "{}", str)
    }
}

impl<'a> TryFrom<&'a str> for RateLimitScope {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "generic_get" => Ok(RateLimitScope::GenericGet),
            "generic_update" => Ok(RateLimitScope::GenericUpdate),
            "message" => Ok(RateLimitScope::Message),
            _ => Err(value),
        }
    }
}