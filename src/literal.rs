use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::sync::OnceLock;

use crate::{XsdError, XsdFromStr, XsdSizedValue, XsdValueDomain};

/// XSD value with its lexical form.
///
/// Useful when the same value can have multiple lexical forms, and you want to
/// remember a specific one.
pub struct XsdLiteral<T: XsdValueDomain> {
	value: T,
	lexical: OnceLock<<T::LexicalForm as ToOwned>::Owned>,
}

impl<T> XsdLiteral<T>
where
	T: XsdValueDomain,
{
	pub fn new(value: T) -> Self {
		Self {
			value,
			lexical: OnceLock::new(),
		}
	}

	pub fn as_value(&self) -> &T {
		&self.value
	}

	pub fn as_value_mut(&mut self) -> &mut T {
		self.lexical = OnceLock::new();
		&mut self.value
	}

	pub fn into_value(self) -> T {
		self.value
	}

	pub fn as_lexical_form(&self) -> &T::LexicalForm {
		self.lexical
			.get_or_init(|| self.value.to_lexical_form())
			.borrow()
	}

	pub fn into_lexical_form(self) -> <T::LexicalForm as ToOwned>::Owned {
		self.lexical
			.into_inner()
			.unwrap_or_else(|| self.value.to_lexical_form())
	}

	pub fn as_str(&self) -> &str {
		self.as_lexical_form().borrow()
	}
}

impl<T: XsdSizedValue> XsdLiteral<T> {
	pub fn from_string(value: String) -> Result<Self, XsdError> {
		Self::try_from_lexical_form(TryFrom::try_from(value).map_err(XsdError::lexical_form)?)
	}

	pub fn try_from_lexical_form(
		lexical_form: <T::LexicalForm as ToOwned>::Owned,
	) -> Result<Self, XsdError> {
		let value = lexical_form.borrow().try_into().map_err(XsdError::value)?;

		Ok(Self {
			value,
			lexical: OnceLock::from(lexical_form),
		})
	}

	pub fn from_lexical_form(lexical_form: <T::LexicalForm as ToOwned>::Owned) -> Self
	where
		for<'a> &'a T::LexicalForm: Into<T>,
	{
		let value = lexical_form.borrow().into();

		Self {
			value,
			lexical: OnceLock::from(lexical_form),
		}
	}
}

impl<T: XsdValueDomain> Deref for XsdLiteral<T> {
	type Target = T;

	fn deref(&self) -> &T {
		self.as_value()
	}
}

impl<T: XsdValueDomain> DerefMut for XsdLiteral<T> {
	fn deref_mut(&mut self) -> &mut T {
		self.as_value_mut()
	}
}

impl<T: XsdValueDomain> Borrow<T> for XsdLiteral<T> {
	fn borrow(&self) -> &T {
		self.as_value()
	}
}

impl<T: XsdValueDomain> BorrowMut<T> for XsdLiteral<T> {
	fn borrow_mut(&mut self) -> &mut T {
		self.as_value_mut()
	}
}

impl<T: XsdValueDomain> AsRef<T> for XsdLiteral<T> {
	fn as_ref(&self) -> &T {
		self.as_value()
	}
}

impl<T: XsdValueDomain> AsMut<T> for XsdLiteral<T> {
	fn as_mut(&mut self) -> &mut T {
		self.as_value_mut()
	}
}

impl<T: XsdValueDomain> From<T> for XsdLiteral<T> {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

/// Clones the value along with its lexical form, if it has one.
impl<T: XsdValueDomain + Clone> Clone for XsdLiteral<T> {
	fn clone(&self) -> Self {
		// `ToOwned::Owned` is not required to be `Clone`, so the lexical form
		// is copied through `ToOwned::to_owned` instead.
		let lexical = match self.lexical.get() {
			Some(owned) => {
				let lexical_form: &T::LexicalForm = owned.borrow();
				OnceLock::from(lexical_form.to_owned())
			}
			None => OnceLock::new(),
		};

		Self {
			value: self.value.clone(),
			lexical,
		}
	}
}

impl<T: XsdValueDomain + Default> Default for XsdLiteral<T> {
	fn default() -> Self {
		Self::new(T::default())
	}
}

/// Compares the values only, ignoring the lexical forms.
impl<T: XsdValueDomain + PartialEq> PartialEq for XsdLiteral<T> {
	fn eq(&self, other: &Self) -> bool {
		self.value == other.value
	}
}

impl<T: XsdValueDomain + Eq> Eq for XsdLiteral<T> {}

/// Compares the values only, ignoring the lexical forms.
impl<T: XsdValueDomain + PartialOrd> PartialOrd for XsdLiteral<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.value.partial_cmp(&other.value)
	}
}

/// Compares the values only, ignoring the lexical forms.
impl<T: XsdValueDomain + Ord> Ord for XsdLiteral<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value.cmp(&other.value)
	}
}

/// Hashes the value only, ignoring the lexical form.
impl<T: XsdValueDomain + Hash> Hash for XsdLiteral<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state)
	}
}

impl<T: XsdSizedValue> FromStr for XsdLiteral<T> {
	type Err = XsdError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::from_string(s.to_owned())
	}
}

impl<'a, T: XsdSizedValue> XsdFromStr<'a> for XsdLiteral<T> {
	fn xsd_from_str(lexical_form: &'a str) -> Result<Self, XsdError> {
		Self::from_str(lexical_form)
	}
}

impl<T: XsdValueDomain> fmt::Display for XsdLiteral<T>
where
	T::LexicalForm: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self.as_lexical_form(), f)
	}
}

impl<T: XsdValueDomain> fmt::Debug for XsdLiteral<T>
where
	T::LexicalForm: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self.as_lexical_form(), f)
	}
}

#[cfg(feature = "serde")]
impl<T: XsdValueDomain> serde::Serialize for XsdLiteral<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.as_lexical_form().borrow().serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for XsdLiteral<T>
where
	T: XsdSizedValue,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		XsdLiteral::from_string(s).map_err(serde::de::Error::custom)
	}
}

#[cfg(test)]
mod tests {
	use std::collections::hash_map::DefaultHasher;

	use super::*;
	use crate::{XsdDateTime, lexical};

	/// Lexical form of a `dateTime` that is valid but not canonical: the
	/// canonical form drops the trailing zero of the fractional seconds.
	const NON_CANONICAL: &str = "2002-10-10T12:00:00.50-05:30";
	const CANONICAL: &str = "2002-10-10T12:00:00.5-05:30";

	fn literal(lexical_form: &str) -> XsdLiteral<XsdDateTime> {
		XsdLiteral::from_string(lexical_form.to_owned()).unwrap()
	}

	fn hash_of<T: Hash>(value: &T) -> u64 {
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);
		hasher.finish()
	}

	#[test]
	fn new_computes_canonical_lexical_form() {
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		let literal = XsdLiteral::new(value);

		assert_eq!(literal.as_lexical_form().as_str(), CANONICAL);
		assert_eq!(literal.to_string(), CANONICAL);
	}

	#[test]
	fn from_string_preserves_lexical_form() {
		let literal = literal(NON_CANONICAL);

		assert_eq!(literal.as_lexical_form().as_str(), NON_CANONICAL);
		assert_eq!(literal.to_string(), NON_CANONICAL);

		// The value itself is the same as the one parsed directly, and its own
		// canonical lexical form is different from the one preserved.
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		assert_eq!(*literal.as_value(), value);
		assert_eq!(literal.as_value().to_lexical_form().as_str(), CANONICAL);
	}

	#[test]
	fn try_from_lexical_form_preserves_lexical_form() {
		let lexical_form = lexical::XsdDateTimeString::new(NON_CANONICAL.to_owned()).unwrap();
		let literal = XsdLiteral::<XsdDateTime>::try_from_lexical_form(lexical_form).unwrap();

		assert_eq!(literal.as_lexical_form().as_str(), NON_CANONICAL);
	}

	#[test]
	fn from_string_rejects_invalid_lexical_form() {
		for input in ["", "not a date", "2002-10-10", "2002-10-10T12:00"] {
			assert!(
				XsdLiteral::<XsdDateTime>::from_string(input.to_owned()).is_err(),
				"{input:?}"
			);
		}
	}

	#[test]
	fn from_string_rejects_invalid_value() {
		// Syntactically valid, but there is no 30th of February.
		let result = XsdLiteral::<XsdDateTime>::from_string("2002-02-30T12:00:00".to_owned());
		assert!(result.is_err());
	}

	#[test]
	fn into_lexical_form_returns_preserved_form() {
		let lexical_form = literal(NON_CANONICAL).into_lexical_form();
		assert_eq!(lexical_form.as_str(), NON_CANONICAL);
	}

	#[test]
	fn into_lexical_form_computes_canonical_form_if_missing() {
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		let lexical_form = XsdLiteral::new(value).into_lexical_form();
		assert_eq!(lexical_form.as_str(), CANONICAL);
	}

	#[test]
	fn into_value_discards_lexical_form() {
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		assert_eq!(literal(NON_CANONICAL).into_value(), value);
	}

	#[test]
	fn mutating_value_invalidates_lexical_form() {
		let mut literal = literal(NON_CANONICAL);
		assert_eq!(literal.as_lexical_form().as_str(), NON_CANONICAL);

		let other: XsdDateTime = "1999-12-31T23:59:59Z".parse().unwrap();
		*literal.as_value_mut() = other;

		assert_eq!(*literal.as_value(), other);
		assert_eq!(literal.as_lexical_form().as_str(), "1999-12-31T23:59:59Z");
	}

	#[test]
	fn deref_mut_invalidates_lexical_form() {
		let mut literal = literal(NON_CANONICAL);
		assert_eq!(literal.as_lexical_form().as_str(), NON_CANONICAL);

		*literal = "1999-12-31T23:59:59Z".parse().unwrap();

		assert_eq!(literal.to_string(), "1999-12-31T23:59:59Z");
	}

	#[test]
	fn clone_preserves_lexical_form() {
		let original = literal(NON_CANONICAL);
		let clone = original.clone();

		assert_eq!(original.to_string(), NON_CANONICAL);
		assert_eq!(clone.to_string(), NON_CANONICAL);
		assert_eq!(original, clone);
	}

	#[test]
	fn clone_is_independent_of_original() {
		let original = literal(NON_CANONICAL);
		let mut clone = original.clone();

		*clone = "1999-12-31T23:59:59Z".parse().unwrap();

		// Mutating the clone resets its lexical form only.
		assert_eq!(clone.to_string(), "1999-12-31T23:59:59Z");
		assert_eq!(original.to_string(), NON_CANONICAL);
	}

	#[test]
	fn clone_without_lexical_form_stays_lazy() {
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		let clone = XsdLiteral::new(value).clone();

		assert_eq!(clone.to_string(), CANONICAL);
	}

	#[test]
	fn equality_and_hash_ignore_lexical_form() {
		let a = literal(NON_CANONICAL);
		let b = literal(CANONICAL);

		assert_ne!(a.to_string(), b.to_string());
		assert_eq!(a, b);
		assert_eq!(hash_of(&a), hash_of(&b));
	}

	#[test]
	fn equality_compares_values() {
		assert_ne!(
			literal("2002-10-10T12:00:00Z"),
			literal("2002-10-10T12:00:01Z")
		);
	}

	#[test]
	fn ordering_compares_values() {
		let earlier = literal("2002-10-10T12:00:00.50Z");
		let later = literal("2002-10-11T12:00:00Z");

		assert_eq!(earlier.partial_cmp(&later), Some(Ordering::Less));
		assert_eq!(later.partial_cmp(&earlier), Some(Ordering::Greater));
		assert_eq!(
			earlier.partial_cmp(&literal("2002-10-10T12:00:00.5Z")),
			Some(Ordering::Equal)
		);
	}

	#[test]
	fn from_value_and_borrowing() {
		let value: XsdDateTime = NON_CANONICAL.parse().unwrap();
		let literal = XsdLiteral::from(value);

		assert_eq!(*literal, value);
		assert_eq!(AsRef::<XsdDateTime>::as_ref(&literal), &value);
		assert_eq!(Borrow::<XsdDateTime>::borrow(&literal), &value);
	}

	#[test]
	fn display_and_debug_use_lexical_form() {
		let literal = literal(NON_CANONICAL);

		assert_eq!(format!("{literal}"), NON_CANONICAL);
		assert_eq!(
			format!("{literal:?}"),
			format!("{:?}", literal.as_lexical_form())
		);
	}
}
