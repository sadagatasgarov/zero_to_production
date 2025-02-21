use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(pub String);

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let empty_or_whitspace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contain_forbidden_characters = s.chars().any(|x| forbidden_characters.contains(&x));

        if !(empty_or_whitspace || is_too_long || contain_forbidden_characters) {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Self(s)
        }
    }
}



impl SubscriberName {
    pub fn inner_ref(&self) -> &str {
    // The caller gets a shared reference to the inner string.
    // This gives the caller **read-only** access,
    // they have no way to compromise our invariants!
    &self.0
    }
    }