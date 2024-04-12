use bitflags::bitflags;
use nom::{combinator::map, IResult};

use crate::{
  data_type::Int32,
  error::{error_position, Error},
};

bitflags! {
  /// 2.2.1.1 `MessageFlags`
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub struct MessageFlags: i32 {
    /// The record contains no arguments.
    /// It is in the Arg category.
    const NO_ARGS                   = 0x00000001;
    /// The Arguments Array is in the Args field of the Method record.
    /// It is in the Arg category.
    const ARGS_INLINE               = 0x00000002;
    /// Each argument is an item in a separate Call Array record.
    /// It is in the Arg category.
    const ARGS_IS_ARRAY             = 0x00000004;
    /// The Arguments Array is an item in a separate Call Array record.
    /// It is in the Arg category.
    const ARGS_IN_ARRAY             = 0x00000008;

    /// The record does not contain a Call Context value.
    /// It is in the Context category.
    const NO_CONTEXT                = 0x00000010;
    /// Call Context contains only a Logical Call ID value and is in
    /// the CallContext field of the Method record.
    /// It is in the Context category.
    const CONTEXT_INLINE            = 0x00000020;
    /// CallContext values are contained in an array that is contained in the Call Array record.
    /// It is in the Context category.
    const CONTEXT_IN_ARRAY          = 0x00000040;

    /// The Method Signature is contained in the Call Array record.
    /// It is in the Signature category.
    const METHOD_SIGNATURE_IN_ARRAY = 0x00000080;

    /// Message Properties is contained in the Call Array record.
    /// It is in the Property category.
    const PROPERTIES_IN_ARRAY       = 0x00000100;

    /// The Return Value is a Null object.
    /// It is in the Return category.
    const NO_RETURN_VALUE           = 0x00000200;
    /// The method has no Return Value.
    /// It is in the Return category.
    const RETURN_VALUE_VOID         = 0x00000400;
    /// The Return Value is in the ReturnValue field of the MethodReturnCallArray record.
    /// It is in the Return category.
    const RETURN_VALUE_INLINE       = 0x00000800;
    /// The Return Value is contained in the MethodReturnCallArray record.
    /// It is in the Return category.
    const RETURN_VALUE_IN_ARRAY     = 0x00001000;

    /// An Exception is contained in the MethodReturnCallArray record.
    /// It is in the Exception category.
    const EXCEPTION_IN_ARRAY        = 0x00002000;

    /// The Remote Method is generic and the actual Remoting Types
    /// for the Generic Arguments are contained in the Call Array.
    /// It is in the Generic category.
    const GENERIC_METHOD            = 0x00008000;
  }
}

impl MessageFlags {
  pub fn parse(input: &[u8]) -> IResult<&[u8], Self, Error<'_>> {
    let err_input = input;

    let (input, flags) = map(Int32::parse, |n| Self::from_bits_retain(n.0))(input)
      .map_err(|err| err.map(|err| error_position!(err.input, ExpectedMessageFlags)))?;

    let args_flags =
      flags.intersection(Self::NO_ARGS.union(Self::ARGS_INLINE).union(Self::ARGS_IS_ARRAY).union(Self::ARGS_IN_ARRAY));

    let context_flags = flags.intersection(Self::NO_CONTEXT.union(Self::CONTEXT_INLINE).union(Self::CONTEXT_IN_ARRAY));
    let return_flags = flags.intersection(
      Self::NO_RETURN_VALUE
        .union(Self::RETURN_VALUE_VOID)
        .union(Self::RETURN_VALUE_IN_ARRAY)
        .union(Self::RETURN_VALUE_IN_ARRAY),
    );
    let signature_flags = flags.intersection(Self::METHOD_SIGNATURE_IN_ARRAY);
    let exception_flags = flags.intersection(Self::EXCEPTION_IN_ARRAY);

    // For each flags category given in the preceding table (Arg, Context, Signature, Return, Exception,
    // Property, and Generic), the value MUST NOT have more than one flag from the Category set.
    if args_flags.bits().count_ones() > 1
      || context_flags.bits().count_ones() > 1
      || return_flags.bits().count_ones() > 1
    {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMessageFlags)))
    }

    // The Args and Exception flag categories are exclusive: if a flag from the Args category is set, the
    // value MUST NOT have any flag from the Exception category set, and vice versa.
    if !args_flags.is_empty() && !exception_flags.is_empty() {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMessageFlags)))
    }

    // The Return and Exception flag categories are exclusive: if a flag from the Return category is set,
    // the value MUST NOT have any flag from the Exception category set, and vice versa.
    if !return_flags.is_empty() && !exception_flags.is_empty() {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMessageFlags)))
    }

    // The Return and Signature categories are exclusive: if a flag from the Return category is set, the
    // value MUST NOT have any flag from the Signature category set, and vice versa.
    if !return_flags.is_empty() && !signature_flags.is_empty() {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMessageFlags)))
    }

    // The Exception and Signature categories are exclusive: if a flag from the Signature category is set,
    // the value MUST NOT have any flag from the Exception category set, and vice versa.
    if !exception_flags.is_empty() && !signature_flags.is_empty() {
      return Err(nom::Err::Failure(error_position!(err_input, InvalidMessageFlags)))
    }

    Ok((input, flags))
  }
}
