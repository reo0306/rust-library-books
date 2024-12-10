use strum::{AsRefStr, EnumIter, EnumString};

#[dervie(Debug, EnumString, AsRefStr, EnumIter, Default, PartialEq, Eq)]
pub enum Role {
    Admin,
    #[default]
    User,
}