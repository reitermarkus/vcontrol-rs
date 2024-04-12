use std::num::NonZeroU32;

use nom::{Err, IResult, ToUsize};

use crate::{
  data_type::Int32,
  error::{error_position, Error},
};

pub fn into_failure<E>(err: Err<E>) -> nom::Err<E> {
  match err {
    Err::Error(e) => Err::Failure(e),
    err => err,
  }
}

pub fn library_id(input: &[u8]) -> IResult<&[u8], NonZeroU32, Error<'_>> {
  let err_input = input;

  match Int32::parse(input) {
    Ok((input, object_id)) => {
      if let Some(object_id) = u32::try_from(i32::from(object_id)).ok().and_then(NonZeroU32::new) {
        Ok((input, object_id))
      } else {
        Err(nom::Err::Failure(error_position!(err_input, InvalidLibraryId)))
      }
    },
    Err(err) => Err(err),
  }
}

pub fn object_id(input: &[u8]) -> IResult<&[u8], NonZeroU32, Error<'_>> {
  let err_input = input;

  match Int32::parse(input) {
    Ok((input, object_id)) => {
      if let Some(object_id) = u32::try_from(i32::from(object_id)).ok().and_then(NonZeroU32::new) {
        Ok((input, object_id))
      } else {
        Err(nom::Err::Failure(error_position!(err_input, InvalidObjectId)))
      }
    },
    Err(err) => Err(err),
  }
}

pub fn length(input: &[u8]) -> IResult<&[u8], usize, Error<'_>> {
  let err_input = input;

  match Int32::parse(input) {
    Ok((input, length)) => {
      if let Ok(length) = u32::try_from(i32::from(length)) {
        Ok((input, length.to_usize()))
      } else {
        Err(nom::Err::Failure(error_position!(err_input, InvalidLength)))
      }
    },
    Err(err) => Err(err),
  }
}
