#![deny(clippy::all)]

use napi::{Error, Status};

#[macro_use]
extern crate napi_derive;

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
  let valid = is_valid(id.clone());
  if valid == false {
    return Err(Error::new(Status::InvalidArg, "Invalid ID".to_string()));
  }

  let time_str = &id[..TIME_LEN];
  let time = u64::from_str_radix(time_str, 32).map_err(|e| {
    Error::new(Status::InvalidArg, format!("Failed to parse time: {}", e))
  })?;

  Ok(time as i64)
}
