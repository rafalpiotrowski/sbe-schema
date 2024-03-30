use crate::evolution::Optional;
use crate::types::{Composite, EnumType, Presence, Ref, SetType, Type};

impl Optional for Type {
    fn is_optional(&self) -> bool {
        match self.presence.as_ref() {
            Some(presence) => *presence == Presence::Optional,
            None => false,
        }
    }
}

impl Optional for Ref {
    fn is_optional(&self) -> bool {
        match self.presence.as_ref() {
            Some(presence) => *presence == Presence::Optional,
            None => false,
        }
    }
}

impl<'a> Optional for &'a Composite {
    fn is_optional(&self) -> bool {
        false
    }
}

impl<'a> Optional for &'a EnumType {
    fn is_optional(&self) -> bool {
        false
    }
}

impl<'a> Optional for &'a SetType {
    fn is_optional(&self) -> bool {
        false
    }
}