mod any_uri;
mod base64_binary;
mod boolean;
mod date_time;
mod decimal;
pub mod double;
pub mod float;
mod hex_binary;

pub use base64_binary::*;
pub use boolean::*;
pub use date_time::*;
pub use decimal::*;
pub use double::{Double, DoubleBuf, InvalidDouble};
pub use float::{Float, FloatBuf, InvalidFloat};
pub use hex_binary::*;

/// Lexical type.
pub trait Lexical {
	type Error;

	fn parse(value: &str) -> Result<&Self, Self::Error>;
}

impl Lexical for str {
	type Error = std::convert::Infallible;

	fn parse(value: &str) -> Result<&Self, Self::Error> {
		Ok(value)
	}
}

pub trait LexicalFormOf<V>: Lexical {
	type ValueError;

	fn try_as_value(&self) -> Result<V, Self::ValueError>;
}

macro_rules! lexical_form {
	{
		$(#[$ty_meta:meta])*
		ty: $ty:ident,

		$(#[$buffer_ty_meta:meta])*
		buffer: $buffer_ty:ident,

		$(#[$new_meta:meta])*
		new,

		$(#[$new_unchecked_meta:meta])*
		new_unchecked,

		value: $value_ty:ty,
		error: $error_ty:ident,
		as_ref: $as_ref:ident,
		parent_forms: { $( $as_parent_form:ident: $parent_form:ty, $parent_buf_form:ty ),* }
	} => {
		#[derive(Debug)]
		pub struct $error_ty;

		$(#[$ty_meta])*
		pub struct $ty([u8]);

		$(#[$buffer_ty_meta])*
		#[derive(Clone)]
		pub struct $buffer_ty(Vec<u8>);

		impl $ty {
			$(#[$new_meta])*
			#[inline(always)]
			pub fn new<S: ?Sized + AsRef<[u8]>>(s: &S) -> Result<&Self, $error_ty> {
				if check_bytes(s.as_ref()) {
					Ok(unsafe { Self::new_unchecked(s) })
				} else {
					Err($error_ty)
				}
			}

			$(#[$new_unchecked_meta])*
			#[inline(always)]
			pub unsafe fn new_unchecked<S: ?Sized + AsRef<[u8]>>(s: &S) -> &Self {
				std::mem::transmute(s.as_ref())
			}

			$(#[$new_unchecked_meta])*
			#[inline(always)]
			pub const unsafe fn new_unchecked_from_slice(s: &[u8]) -> &Self {
				std::mem::transmute(s)
			}

			#[inline(always)]
			pub fn as_str(&self) -> &str {
				unsafe { core::str::from_utf8_unchecked(&self.0) }
			}

			#[inline(always)]
			pub fn as_bytes(&self) -> &[u8] {
				&self.0
			}
		}

		impl std::fmt::Debug for $ty {
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.as_str().fmt(f)
			}
		}

		impl crate::Lexical for $ty {
			type Error = $error_ty;

			#[inline(always)]
			fn parse(value: &str) -> Result<&Self, Self::Error> {
				Self::new(value)
			}
		}

		impl std::ops::Deref for $ty {
			type Target = str;

			#[inline(always)]
			fn deref(&self) -> &str {
				self.as_str()
			}
		}

		impl AsRef<[u8]> for $ty {
			fn as_ref(&self) -> &[u8] {
				&self.0
			}
		}

		impl AsRef<str> for $ty {
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}

		impl std::borrow::ToOwned for $ty {
			type Owned = $buffer_ty;

			#[inline(always)]
			fn to_owned(&self) -> $buffer_ty {
				unsafe { <$buffer_ty>::new_unchecked(self.as_str().to_owned()) }
			}
		}

		impl std::fmt::Display for $ty {
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.as_str().fmt(f)
			}
		}

		impl<'a> From<&'a $buffer_ty> for &'a $ty {
			#[inline(always)]
			fn from(b: &'a $buffer_ty) -> Self {
				b.as_ref()
			}
		}

		impl $buffer_ty {
			$(#[$new_meta])*
			#[inline(always)]
			pub fn new<S: AsRef<[u8]> + Into<Vec<u8>>>(
				s: S,
			) -> Result<Self, ($error_ty, S)> {
				if check_bytes(s.as_ref()) {
					Ok(unsafe { Self::new_unchecked(s) })
				} else {
					Err(($error_ty, s))
				}
			}

			$(#[$new_unchecked_meta])*
			#[inline(always)]
			pub unsafe fn new_unchecked(s: impl Into<Vec<u8>>) -> Self {
				std::mem::transmute(s.into())
			}

			#[inline(always)]
			pub fn $as_ref(&self) -> &$ty {
				unsafe { <$ty>::new_unchecked(&self.0) }
			}

			#[inline(always)]
			pub fn into_bytes(self) -> Vec<u8> {
				self.0
			}

			#[inline(always)]
			pub fn into_string(mut self) -> String {
				let buf = self.0.as_mut_ptr();
				let len = self.0.len();
				let capacity = self.0.capacity();
				core::mem::forget(self);
				unsafe { String::from_raw_parts(buf, len, capacity) }
			}

			#[inline(always)]
			pub fn into_value(self) -> $value_ty {
				self.value()
			}
		}

		impl PartialEq for $buffer_ty {
			fn eq(&self, other: &Self) -> bool {
				self.$as_ref().eq(other.$as_ref())
			}
		}

		impl Eq for $buffer_ty {}

		impl std::str::FromStr for $buffer_ty {
			type Err = $error_ty;

			fn from_str(s: &str) -> Result<Self, $error_ty> {
				Self::new(s.to_owned()).map_err(|(e, _)| e)
			}
		}

		impl AsRef<[u8]> for $buffer_ty {
			#[inline(always)]
			fn as_ref(&self) -> &[u8] {
				self.as_bytes()
			}
		}

		impl AsRef<str> for $buffer_ty {
			#[inline(always)]
			fn as_ref(&self) -> &str {
				self.as_str()
			}
		}

		impl std::ops::Deref for $buffer_ty {
			type Target = $ty;

			#[inline(always)]
			fn deref(&self) -> &$ty {
				self.$as_ref()
			}
		}

		impl AsRef<$ty> for $buffer_ty {
			#[inline(always)]
			fn as_ref(&self) -> &$ty {
				self.$as_ref()
			}
		}

		impl Borrow<$ty> for $buffer_ty {
			#[inline(always)]
			fn borrow(&self) -> &$ty {
				self.$as_ref()
			}
		}

		impl fmt::Display for $buffer_ty {
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.as_str().fmt(f)
			}
		}

		impl std::fmt::Debug for $buffer_ty {
			#[inline(always)]
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.0.fmt(f)
			}
		}

		impl PartialEq<$ty> for $buffer_ty {
			#[inline(always)]
			fn eq(&self, other: &$ty) -> bool {
				self == other
			}
		}

		impl<'a> PartialEq<&'a $ty> for $buffer_ty {
			#[inline(always)]
			fn eq(&self, other: &&'a $ty) -> bool {
				self.$as_ref() == *other
			}
		}

		impl PartialEq<$buffer_ty> for $ty {
			#[inline(always)]
			fn eq(&self, other: &$buffer_ty) -> bool {
				self == other
			}
		}

		$(
			impl $ty {
				#[inline(always)]
				pub fn $as_parent_form(&self) -> &$parent_form {
					self.into()
				}
			}

			impl<'a> From<&'a $ty> for &'a $parent_form {
				#[inline(always)]
				fn from(value: &'a $ty) -> Self {
					unsafe { <$parent_form>::new_unchecked(value) }
				}
			}

			impl<'a> TryFrom<&'a $parent_form> for &'a $ty {
				type Error = $error_ty;

				#[inline(always)]
				fn try_from(i: &'a $parent_form) -> Result<Self, Self::Error> {
					<$ty>::new(i)
				}
			}

			impl TryFrom<$parent_buf_form> for $buffer_ty {
				type Error = ($error_ty, $parent_buf_form);

				#[inline(always)]
				fn try_from(i: $parent_buf_form) -> Result<Self, Self::Error> {
					match Self::new(i.into_string()) {
						Ok(d) => Ok(d),
						Err((e, s)) => Err((e, unsafe { <$parent_buf_form>::new_unchecked(s) })),
					}
				}
			}

			impl AsRef<$parent_form> for $ty {
				#[inline(always)]
				fn as_ref(&self) -> &$parent_form {
					self.into()
				}
			}

			impl AsRef<$parent_form> for $buffer_ty {
				#[inline(always)]
				fn as_ref(&self) -> &$parent_form {
					self.$as_ref().into()
				}
			}

			impl PartialEq<$parent_form> for $ty {
				#[inline(always)]
				fn eq(&self, other: &$parent_form) -> bool {
					self.as_str() == other.as_str()
				}
			}
		)*
	};
}

pub(crate) use lexical_form;
