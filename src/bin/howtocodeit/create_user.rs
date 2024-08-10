use std::fmt::Display;

use anyhow::{anyhow, Error};
use regex::Regex;
use thiserror::Error;

fn email_regex() -> Regex {
    Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid username")]
pub struct UsernameError(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, UsernameError> {
        if username.len() >= 3 {
            Ok(Self(username.to_string()))
        } else {
            Err(UsernameError(username.to_string()))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PasswordError {
    #[error("Minimum {0} character required")]
    MinLen(u8),

    #[error("Special character required")]
    SpecialCharacterRequired,
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Return encrypted or hash of password.
        write!(f, "#HIDDEN")
    }
}

impl Password {
    pub fn new(password: &str) -> Result<Self, PasswordError> {
        if password.trim().len() < 8 {
            return Err(PasswordError::MinLen(8));
        }

        if !password.contains('!') {
            return Err(PasswordError::SpecialCharacterRequired);
        }

        Ok(Self(password.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid email address")]
pub struct EmailAddressError(String);

impl EmailAddress {
    pub fn new(raw_email: &str) -> Result<Self, EmailAddressError> {
        if email_regex().is_match(raw_email) {
            Ok(Self(raw_email.into()))
        } else {
            Err(EmailAddressError(raw_email.to_string()))
        }
    }
}

#[derive(Error, Clone, Debug, PartialEq)]
#[error("a user with email address {0} already exists")]
pub struct UserAlreadyExistsError(EmailAddress);

pub struct User {
    id: String,
    username: Username,
    email: EmailAddress,
    password: Password,
}

impl User {
    pub fn get_username(&self) -> &Username {
        &self.username
    }
}

pub fn get_longer<'a>(s1: &'a Username, s2: &'a Username) -> &'a Username {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn get_longer_example() -> anyhow::Result<()> {
    let user1 = Username::new("john").map_err(|_| anyhow!("Wrong username"))?;
    let user2 = Username::new("joe").map_err(|_| anyhow!("Wrong username"))?;

    let longer_str = get_longer(&user1, &user2);

    println!("Longer name: {}", longer_str);

    Ok(())
}

pub fn create_user(
    username: Username,
    email: EmailAddress,
    password: Password,
) -> Result<User, UserAlreadyExistsError> {
    let _ = get_longer_example();

    Ok(User {
        id: "".to_string(),
        username,
        email,
        password,
    })
}
