use crate::evolution::Optional;
use crate::types::{Presence, Ref, Type};

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
