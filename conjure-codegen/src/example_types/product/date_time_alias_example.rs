use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash)]
pub struct DateTimeAliasExample(pub conjure_object::DateTime<conjure_object::Utc>);
impl std::fmt::Display for DateTimeAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for DateTimeAliasExample {
    type Target = conjure_object::DateTime<conjure_object::Utc>;
    #[inline]
    fn deref(&self) -> &conjure_object::DateTime<conjure_object::Utc> {
        &self.0
    }
}
impl std::ops::DerefMut for DateTimeAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::DateTime<conjure_object::Utc> {
        &mut self.0
    }
}
impl ser::Serialize for DateTimeAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for DateTimeAliasExample {
    fn deserialize<D>(d: D) -> Result<DateTimeAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(DateTimeAliasExample)
    }
}