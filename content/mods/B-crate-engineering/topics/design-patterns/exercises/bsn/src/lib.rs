use std::fmt::Display;

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
/// Error creating BSN
// TODO: update the enum to make it more descriptive
// as there can be several reasons for a BSN to not be valid
pub enum Error {
    /// The BSN was invalid
    InvalidBsn,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidBsn => write!(f, "Invalid BSN number"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    /// Try to create a new BSN. Returns `Err` if the passed string
    /// does not represent a valid BSN
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        if !Self::check_bsn_str(bsn.to_string().as_str()) {
            return Err(Error::InvalidBsn);
        }

        Ok(Bsn {
            inner: bsn.to_string(),
        })
    }

    /// Check whether the passed string represents a valid BSN.
    //  Returns `Err` if the passed string does not represent a valid BSN
    pub fn validate(bsn: &str) -> Result<(), Error> {
        if !Self::check_bsn_str(bsn) {
            return Err(Error::InvalidBsn);
        }

        Ok(())
    }

    pub fn check_bsn_str(bsn: &str) -> bool {
        let array = [9, 8, 7, 6, 5, 4, 3, 2, -1];
        bsn.len() == 9
            && bsn.chars().all(|c| c.is_ascii_digit())
            && bsn
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .zip(array.iter())
                .map(|(x, &y)| x as i64 * y as i64)
                .sum::<i64>()
                % 11
                == 0
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A visitor for deserializing strings into `Bns`
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            // TODO: Override the correct `Visitor::visit_*` to validate the input and output a new `BSN`
            // if the input represents a valid BSN. Note that we do not need to override all default methods
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if !Bsn::check_bsn_str(v) {
                    return Err(E::custom(Error::InvalidBsn));
                }

                Ok(Bsn {
                    inner: v.to_string(),
                })
            }
        }

        deserializer.deserialize_str(BsnVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_validation() {
        let bsns = include_str!("../valid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_ok(),
                "BSN {bsn} is valid, but did not pass validation"
            )
        });

        let bsns = include_str!("../invalid_bsns.in").lines();
        bsns.for_each(|bsn| {
            assert!(
                Bsn::validate(bsn).is_err(),
                "BSN {bsn} invalid, but passed validation"
            )
        });
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}
