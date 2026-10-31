use crate::{
	evolution::Optional,
	types::{Composite, EnumType, Presence, Ref, SetType, Type},
};

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

impl Optional for &Composite {
	fn is_optional(&self) -> bool {
		false
	}
}

impl Optional for &EnumType {
	fn is_optional(&self) -> bool {
		false
	}
}

impl Optional for &SetType {
	fn is_optional(&self) -> bool {
		false
	}
}
