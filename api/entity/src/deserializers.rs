use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};
use serde_aux::prelude::deserialize_option_number_from_string;

pub fn deserialize_option_option_number_from_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    Ok(Some(deserialize_option_number_from_string(deserializer)?))
}
