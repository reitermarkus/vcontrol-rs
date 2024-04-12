use std::num::NonZeroU32;

use nom::IResult;

use crate::{
  data_type::Int32,
  error::{error_position, Error},
  record::RecordType,
};

/// 2.6.1 `SerializationHeaderRecord`
#[derive(Debug, Clone, PartialEq)]
pub struct SerializationHeader {
  pub root_id: Option<NonZeroU32>,
  pub header_id: Int32,
  pub major_version: Int32,
  pub minor_version: Int32,
}

impl SerializationHeader {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let (input, _) = RecordType::SerializedStreamHeader.parse(input)?;

    let err_input = input;
    let (input, root_id) = match Int32::parse(input) {
      Ok((input, object_id)) => {
        if let Ok(object_id) = u32::try_from(i32::from(object_id)) {
          Ok((input, NonZeroU32::new(object_id)))
        } else {
          Err(nom::Err::Failure(error_position!(err_input, InvalidRootId)))
        }
      },
      Err(err) => Err(err),
    }?;

    let (input, header_id) = Int32::parse(input)?;

    let err_input = input;
    let (input, major_version) = match Int32::parse(input) {
      Ok((input, major_version)) => {
        if major_version.0 == 1 {
          Ok((input, major_version))
        } else {
          Err(nom::Err::Failure(error_position!(err_input, InvalidMajorVersion)))
        }
      },
      Err(err) => Err(err),
    }?;

    let err_input = input;
    let (input, minor_version) = match Int32::parse(input) {
      Ok((input, minor_version)) => {
        if minor_version.0 == 0 {
          Ok((input, minor_version))
        } else {
          Err(nom::Err::Failure(error_position!(err_input, InvalidMinorVersion)))
        }
      },
      Err(err) => Err(err),
    }?;

    Ok((input, Self { root_id, header_id, major_version, minor_version }))
  }
}
