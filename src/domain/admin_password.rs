use secrecy::{ExposeSecret, SecretString};

#[derive(Debug)]
pub struct AdminPassword(SecretString);

impl AdminPassword {
    pub fn parse<S>(password: S) -> Result<Self, String>
    where
        S: AsRef<str>,
    {
        let password_len = password.as_ref().chars().count();
        if !(12..=128).contains(&password_len) {
            return Err("String length is not acceptable".into());
        }

        Ok(Self(SecretString::from(password.as_ref())))
    }
}

impl<'a> TryFrom<&'a str> for AdminPassword {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for AdminPassword {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<SecretString> for AdminPassword {
    type Error = String;

    fn try_from(value: SecretString) -> Result<Self, Self::Error> {
        Self::parse(value.expose_secret())
    }
}

impl TryFrom<&SecretString> for AdminPassword {
    type Error = String;

    fn try_from(value: &SecretString) -> Result<Self, Self::Error> {
        Self::parse(value.expose_secret())
    }
}

impl AsRef<str> for AdminPassword {
    fn as_ref(&self) -> &str {
        self.0.expose_secret()
    }
}

impl From<AdminPassword> for SecretString {
    fn from(value: AdminPassword) -> Self {
        value.0
    }
}
