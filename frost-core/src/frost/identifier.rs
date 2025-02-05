//! FROST participant identifiers

use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

use crate::{Ciphersuite, Error, Field, FieldError, Group, Scalar};

/// A FROST participant identifier.
///
/// The identifier is a field element in the scalar field that the secret polynomial is defined
/// over, corresponding to some x-coordinate for a polynomial f(x) = y.  MUST NOT be zero in the
/// field, as f(0) = the shared secret.
#[derive(Copy, Clone, PartialEq)]
pub struct Identifier<C: Ciphersuite>(Scalar<C>);

impl<C> Identifier<C>
where
    C: Ciphersuite,
{
    /// Serialize the identifier using the ciphersuite encoding.
    pub fn serialize(&self) -> <<C::Group as Group>::Field as Field>::Serialization {
        <<C::Group as Group>::Field>::serialize(&self.0)
    }

    /// Deserialize an Identifier from a serialized buffer.
    /// Returns an error if it attempts to deserialize zero.
    pub fn deserialize(
        buf: &<<C::Group as Group>::Field as Field>::Serialization,
    ) -> Result<Self, Error<C>> {
        let scalar = <<C::Group as Group>::Field>::deserialize(buf)?;
        if scalar == <<C::Group as Group>::Field>::zero() {
            Err(FieldError::InvalidZeroScalar.into())
        } else {
            Ok(Self(scalar))
        }
    }
}

impl<C> Eq for Identifier<C> where C: Ciphersuite {}

impl<C> Debug for Identifier<C>
where
    C: Ciphersuite,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Identifier")
            .field(&<<C::Group as Group>::Field>::serialize(&self.0).as_ref())
            .finish()
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl<C> Hash for Identifier<C>
where
    C: Ciphersuite,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        <<C::Group as Group>::Field>::serialize(&self.0)
            .as_ref()
            .hash(state)
    }
}

impl<C> Ord for Identifier<C>
where
    C: Ciphersuite,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let serialized_self = <<C::Group as Group>::Field>::little_endian_serialize(&self.0);
        let serialized_other = <<C::Group as Group>::Field>::little_endian_serialize(&other.0);
        serialized_self.as_ref().cmp(serialized_other.as_ref())
    }
}

impl<C> PartialOrd for Identifier<C>
where
    C: Ciphersuite,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let serialized_self = <<C::Group as Group>::Field>::little_endian_serialize(&self.0);
        let serialized_other = <<C::Group as Group>::Field>::little_endian_serialize(&other.0);
        serialized_self
            .as_ref()
            .partial_cmp(serialized_other.as_ref())
    }
}

impl<C> std::ops::Mul<Scalar<C>> for Identifier<C>
where
    C: Ciphersuite,
{
    type Output = Scalar<C>;

    fn mul(self, scalar: Scalar<C>) -> Scalar<C> {
        self.0 * scalar
    }
}

impl<C> std::ops::MulAssign<Identifier<C>> for Scalar<C>
where
    C: Ciphersuite,
{
    fn mul_assign(&mut self, identifier: Identifier<C>) {
        *self = *self * identifier.0
    }
}

impl<C> std::ops::Sub for Identifier<C>
where
    C: Ciphersuite,
{
    type Output = Self;

    fn sub(self, rhs: Identifier<C>) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<C> TryFrom<u16> for Identifier<C>
where
    C: Ciphersuite,
{
    type Error = Error<C>;

    fn try_from(n: u16) -> Result<Identifier<C>, Self::Error> {
        if n == 0 {
            Err(FieldError::InvalidZeroScalar.into())
        } else {
            // Classic left-to-right double-and-add algorithm that skips the first bit 1 (since
            // identifiers are never zero, there is always a bit 1), thus `sum` starts with 1 too.
            let one = <<C::Group as Group>::Field>::one();
            let mut sum = <<C::Group as Group>::Field>::one();

            let bits = (n.to_be_bytes().len() as u32) * 8;
            for i in (0..(bits - n.leading_zeros() - 1)).rev() {
                sum = sum + sum;
                if n & (1 << i) != 0 {
                    sum = sum + one;
                }
            }
            Ok(Self(sum))
        }
    }
}
