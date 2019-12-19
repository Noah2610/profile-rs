pub mod prelude {
    pub use super::Profile;
    pub use super::Profiles;
}

#[derive(Clone, Debug)]
pub struct Profiles(Vec<Profile>);

impl<V> From<V> for Profiles
where
    V: Into<Vec<Profile>>,
{
    fn from(v: V) -> Self {
        Self(v.into())
    }
}

#[derive(Clone, Debug)]
pub struct Profile {
    name: String,
}

impl<S> From<S> for Profile
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self { name: s.into() }
    }
}
