#![deny(clippy::all)]

use std::str::FromStr;

use napi::{Error, Status};

#[macro_use]
extern crate napi_derive;

use rusty_ulid::{DecodingError, Ulid};

const TIME_LEN: usize = 10;
const RANDOM_LEN: usize = 16;
const ENCODING: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";

#[napi]
pub fn is_valid(id: String) -> bool {
  if id.len() != TIME_LEN + RANDOM_LEN {
    return false;
  }

  id.chars().all(|c| ENCODING.contains(c))
}

#[napi]
pub fn decode_time(id: String) -> Result<i64, Error> {
  if !is_valid(id.clone()) {
    return Err(Error::new(Status::InvalidArg, "Invalid ID".to_string()));
  }

  let ulid: Ulid = Ulid::from_str(&id)
    .map_err(|e: DecodingError| Error::new(Status::InvalidArg, e.to_string()))?;
  let timestamp = ulid.timestamp();
  Ok(timestamp as i64)
}
