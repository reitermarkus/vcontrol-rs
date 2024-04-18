use std::{
  collections::{BTreeMap, HashMap},
  num::NonZeroU32,
};

use nom::{
  branch::alt,
  combinator::{map, opt, verify},
  multi::count,
  IResult,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::LengthPrefixedString,
  enumeration::BinaryType,
  error::{error_position, Error, ErrorInner},
  record::{
    ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary, BinaryMethodCall,
    BinaryMethodReturn, BinaryObjectString, ClassWithId, ClassWithMembers, ClassWithMembersAndTypes,
    MemberPrimitiveTyped, MemberPrimitiveUnTyped, MemberReference, MessageEnd, MessageFlags, ObjectNull,
    ObjectNullMultiple, ObjectNullMultiple256, SerializationHeader, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
  value::Object,
  MethodCall, MethodReturn, RemotingMessage, Value,
};

#[derive(Debug, Clone)]
enum ValueOrRef<'i> {
  Value(Value<'i>),
  Null(usize),
  Ref(RefId),
}

#[derive(Debug, Clone, Copy)]
pub struct RefId(NonZeroU32);

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum Class<'i> {
  ClassWithMembers(ClassWithMembers<'i>),
  ClassWithMembersAndTypes(ClassWithMembersAndTypes<'i>),
  SystemClassWithMembers(SystemClassWithMembers<'i>),
  SystemClassWithMembersAndTypes(SystemClassWithMembersAndTypes<'i>),
}

#[derive(Debug, Clone, PartialEq)]
enum MethodCallOrReturn<'i> {
  MethodCall(MethodCall<'i>),
  MethodReturn(MethodReturn<'i>),
}

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  binary_libraries: BTreeMap<NonZeroU32, LengthPrefixedString<'i>>,
  classes: BTreeMap<NonZeroU32, Class<'i>>,
  objects: BTreeMap<NonZeroU32, Value<'i>>,
}

macro_rules! alt_mut {
  ($input:expr => $($expr:expr),+ $(,)?) => {
    'alt: {
      $(
        match $expr($input) {
          Err(nom::Err::Error(_)) => (),
          res => break 'alt res,
        }
      )+

      Err(nom::Err::Error(nom::error::ParseError::from_error_kind($input, nom::error::ErrorKind::Alt)))
    }
  };
}

impl<'i> BinaryParser<'i> {
  fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (), Error<'i>> {
    let (input, binary_library) = opt(BinaryLibrary::parse)(input)?;

    if let Some(binary_library) = binary_library {
      let library_id = binary_library.library_id();

      if self.binary_libraries.insert(library_id, binary_library.library_name).is_some() {
        return Err(nom::Err::Failure(error_position!(input, DuplicateLibraryId)))
      }
    }

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `memberReference`
  fn parse_member_reference(
    &mut self,
    input: &'i [u8],
    type_enum_and_additional_type_info: Option<(BinaryType, Option<&AdditionalTypeInfo<'i>>)>,
  ) -> IResult<&'i [u8], ValueOrRef<'i>, Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, object) = if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
      match (type_enum, additional_type_info) {
        (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => map(
          |input| MemberPrimitiveUnTyped::parse(input, *primitive_type),
          |primitive| ValueOrRef::Value(primitive.into_value()),
        )(input)?,
        (BinaryType::String, None) => {
          map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str())))(input)?
        },
        (BinaryType::Object, None) => return self.parse_member_reference(input, None),
        (BinaryType::SystemClass, Some(AdditionalTypeInfo::SystemClass(class_name))) => {
          if let Ok((input, (_, object))) = self.parse_classes(input) {
            if object.class != class_name.as_str() || object.library.is_some() {
              return Err(nom::Err::Failure(error_position!(input, UnexpectedClass)))
            }

            (input, ValueOrRef::Value(Value::Object(object)))
          } else {
            Self::parse_null_object(input)?
          }
        },
        (BinaryType::Class, Some(AdditionalTypeInfo::Class(class_type_info))) => {
          let err_input = input;

          let library = if let Some(library) = self.binary_libraries.get(&class_type_info.library_id()) {
            library.as_str()
          } else {
            return Err(nom::Err::Failure(error_position!(err_input, MissingLibraryId)))
          };

          if let Ok((input, (_, object))) = self.parse_classes(input) {
            if object.class != class_type_info.type_name.as_str() || object.library != Some(library) {
              return Err(nom::Err::Failure(error_position!(input, UnexpectedClass)))
            }

            (input, ValueOrRef::Value(Value::Object(object)))
          } else {
            Self::parse_null_object(input)?
          }
        },
        (BinaryType::ObjectArray, None) => return self.parse_member_reference(input, None),
        (BinaryType::StringArray, None) => alt((
          map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str()))),
          map(
            |input| MemberReference::parse(input),
            |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref)),
          ),
          Self::parse_null_object,
        ))(input)?,
        (BinaryType::PrimitiveArray, Some(AdditionalTypeInfo::Primitive(_primitive_type))) => map(
          |input| MemberReference::parse(input),
          |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref)),
        )(input)?,
        _ => unreachable!(),
      }
    } else {
      alt((
        map(|input| MemberPrimitiveTyped::parse(input), |primitive| ValueOrRef::Value(primitive.into_value())),
        map(|input| MemberReference::parse(input), |member_reference| ValueOrRef::Ref(RefId(member_reference.id_ref))),
        map(BinaryObjectString::parse, |s| ValueOrRef::Value(Value::String(s.as_str()))),
        Self::parse_null_object,
        map(|input| self.parse_classes(input), |(_, object)| ValueOrRef::Value(Value::Object(object))),
      ))(input)?
    };

    Ok((input, object))
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<ValueOrRef<'i>>, Error<'i>> {
    let mut member_references = vec![];

    for (binary_type_enum, additional_info) in
      member_type_info.binary_type_enums.iter().zip(member_type_info.additional_infos.iter())
    {
      let member;
      (input, member) = self.parse_member_reference(input, Some((*binary_type_enum, additional_info.as_ref())))?;
      member_references.push(member);
    }

    Ok((input, member_references))
  }

  /// Resolves members from already parsed objects or by parsing missing members.
  fn resolve_members(
    &mut self,
    mut input: &'i [u8],
    members: Vec<ValueOrRef<'i>>,
  ) -> IResult<&'i [u8], Vec<Value<'i>>, Error<'i>> {
    let mut members2 = Vec::with_capacity(members.len());

    for member in members.into_iter() {
      match member {
        ValueOrRef::Value(value) => {
          members2.push(value);
        },
        ValueOrRef::Null(count) => {
          for _ in 0..count {
            members2.push(Value::Null);
          }
        },
        ValueOrRef::Ref(id) => {
          if let Some(value) = self.objects.remove(&id.0) {
            members2.push(value);
          } else {
            let member2;
            (input, member2) = verify(|input| self.parse_referenceable(input), |id2| id2.0 == id.0)(input)?;

            if let Some(value) = self.objects.remove(&member2.0) {
              members2.push(value);
            } else {
              return Err(nom::Err::Failure(error_position!(input, UnresolvableMemberReference)))
            }
          }
        },
      }
    }

    Ok((input, members2))
  }

  /// 2.7 Binary Record Grammar - `Classes`
  fn parse_classes(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Object<'i>), Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let err_input = input;

    let (input, (object_id, class)) = match ClassWithId::parse(input) {
      Ok((input, class_with_id)) => {
        if let Some(class) = self.classes.get(&class_with_id.metadata_id()) {
          (input, (class_with_id.object_id(), class.clone()))
        } else {
          return Err(nom::Err::Failure(error_position!(err_input, MissingMetadataId)))
        }
      },
      Err(nom::Err::Error(_)) => alt((
        map(ClassWithMembers::parse, |class| (class.object_id(), Class::ClassWithMembers(class))),
        map(ClassWithMembersAndTypes::parse, |class| (class.object_id(), Class::ClassWithMembersAndTypes(class))),
        map(SystemClassWithMembers::parse, |class| (class.object_id(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id(), Class::SystemClassWithMembersAndTypes(class))
        }),
      ))(input)?,
      Err(err) => return Err(err),
    };

    if self.classes.contains_key(&object_id) {
      return Err(nom::Err::Failure(error_position!(input, DuplicateObjectId)))
    }

    let (input, (class_info, library, member_references)) = match class {
      Class::ClassWithMembers(ref class) => {
        let library = if let Some(library) = self.binary_libraries.get(&class.library_id()) {
          library.as_str()
        } else {
          return Err(nom::Err::Failure(error_position!(err_input, MissingLibraryId)))
        };

        let member_count = class.class_info().member_names.len();
        let (input, member_references) = count(|input| self.parse_member_reference(input, None), member_count)(input)?;

        (input, (class.class_info(), Some(library), member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let library = if let Some(library) = self.binary_libraries.get(&class.library_id()) {
          library.as_str()
        } else {
          return Err(nom::Err::Failure(error_position!(err_input, MissingLibraryId)))
        };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class.class_info(), Some(library), member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let member_count = class.class_info().member_names.len();
        let (input, member_references) = count(|input| self.parse_member_reference(input, None), member_count)(input)?;

        (input, (class.class_info(), None, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (class.class_info(), None, member_references))
      },
    };

    let (input, member_references) = self.resolve_members(input, member_references)?;

    let members = HashMap::from_iter(
      class_info
        .member_names
        .iter()
        .zip(member_references)
        .map(|(member_name, member)| (member_name.as_str(), { member })),
    );

    let class_name = class_info.name.as_str();

    Ok((input, (RefId(object_id), Object { class: class_name, library, members })))
  }

  /// 2.7 Binary Record Grammar - `ArraySingleObject *(memberReference)`
  fn parse_array_single_object(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>), Error<'i>> {
    let (mut input, array_single_object) = ArraySingleObject::parse(input)?;

    let mut members = vec![];

    let len = array_single_object.array_info.len();
    while members.len() < len {
      let member;
      (input, member) = self.parse_member_reference(input, None)?;

      match member {
        ValueOrRef::Null(count) => {
          for _ in 0..count {
            members.push(ValueOrRef::Value(Value::Null));
          }
        },
        _ => {
          members.push(member);
        },
      }
    }

    let (input, members) = self.resolve_members(input, members)?;

    let object_id = array_single_object.object_id();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `ArraySinglePrimitive *(MemberPrimitiveUnTyped)`
  fn parse_array_single_primitive(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>), Error<'i>> {
    let (input, array_single_primitive) = ArraySinglePrimitive::parse(input)?;

    let (input, members) = count(
      map(
        |input| MemberPrimitiveUnTyped::parse(input, array_single_primitive.primitive_type),
        |primitive| primitive.into_value(),
      ),
      array_single_primitive.array_info.len(),
    )(input)?;

    let object_id = array_single_primitive.object_id();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `ArraySingleString *(BinaryObjectString/MemberReference/nullObject)`
  fn parse_array_single_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>), Error<'i>> {
    let (mut input, array_single_string) = ArraySingleString::parse(input)?;

    let mut members = vec![];

    let len = array_single_string.array_info.len();
    while members.len() < len {
      let member;
      (input, member) = self.parse_member_reference(input, Some((BinaryType::StringArray, None)))?;

      match member {
        ValueOrRef::Null(count) => {
          for _ in 0..count {
            members.push(ValueOrRef::Value(Value::Null));
          }
        },
        _ => {
          members.push(member);
        },
      }
    }

    let (input, members) = self.resolve_members(input, members)?;

    let object_id = array_single_string.object_id();
    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `BinaryArray *(memberReference)`
  fn parse_binary_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>), Error<'i>> {
    let err_input = input;

    let (input, binary_array) = BinaryArray::parse(input)?;

    let member_count = match binary_array.lengths.iter().copied().try_fold(1usize, |acc, n| acc.checked_mul(n)) {
      Some(member_count) => member_count,
      None => return Err(nom::Err::Failure(error_position!(err_input, InvalidLength))),
    };
    let (input, members) = count(
      |input| {
        self.parse_member_reference(input, Some((binary_array.type_enum, binary_array.additional_type_info.as_ref())))
      },
      member_count,
    )(input)?;

    let (input, members) = self.resolve_members(input, members)?;
    let object_id = binary_array.object_id();

    let mut members = members;
    for l in binary_array.lengths.into_iter().skip(1).rev() {
      let mut members2 = vec![];

      while !members.is_empty() {
        members2.push(Value::Array(members.drain(0..l).collect::<Vec<_>>()));
      }

      members = members2;
    }

    Ok((input, (RefId(object_id), members)))
  }

  /// 2.7 Binary Record Grammar - `Arrays`
  fn parse_arrays(&mut self, input: &'i [u8]) -> IResult<&'i [u8], (RefId, Vec<Value<'i>>), Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    alt_mut!(input =>
      |input| self.parse_array_single_object(input),
      |input| self.parse_array_single_primitive(input),
      |input| self.parse_array_single_string(input),
      |input| self.parse_binary_array(input),
    )
  }

  /// 2.7 Binary Record Grammar - `referenceable`
  fn parse_referenceable(&mut self, input: &'i [u8]) -> IResult<&'i [u8], RefId, Error<'i>> {
    let (input, (object_id, object)) = alt_mut!(input =>
      map(
        |input| self.parse_classes(input),
        |(object_id, object)| (object_id, Value::Object(object)),
      ),
      map(
        |input| self.parse_arrays(input),
        |(object_id, array)| (object_id, Value::Array(array)),
      ),
      map(
        BinaryObjectString::parse,
        |s| (RefId(s.object_id()), Value::String(s.as_str())),
      ),
    )?;

    self.objects.insert(object_id.0, object);
    Ok((input, object_id))
  }

  /// 2.7 Binary Record Grammar - `nullObject`
  fn parse_null_object(input: &'i [u8]) -> IResult<&'i [u8], ValueOrRef<'i>, Error<'i>> {
    alt((
      map(|input| ObjectNull::parse(input), |n| ValueOrRef::Null(n.null_count())),
      map(|input| ObjectNullMultiple::parse(input), |n| ValueOrRef::Null(n.null_count())),
      map(|input| ObjectNullMultiple256::parse(input), |n| ValueOrRef::Null(n.null_count())),
    ))(input)
  }

  fn parse_call_array(
    &mut self,
    input: &'i [u8],
    root_id: Option<NonZeroU32>,
  ) -> IResult<&'i [u8], Vec<Value<'i>>, Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (call_array_id, call_array)) = self.parse_array_single_object(input)?;

    if Some(call_array_id.0) != root_id {
      return Err(nom::Err::Failure(Error { input, inner: ErrorInner::InvalidCallArrayId }))
    }

    Ok((input, call_array))
  }

  /// 2.7 Binary Record Grammar - `methodCall`
  fn parse_method_call(
    &mut self,
    input: &'i [u8],
    root_id: Option<NonZeroU32>,
  ) -> IResult<&'i [u8], MethodCall<'i>, Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_call) = BinaryMethodCall::parse(input)?;

    let (input, call_array) = opt(|input| self.parse_call_array(input, root_id))(input)?;

    let parse_args = |message_enum: MessageFlags| {
      if message_enum.intersects(MessageFlags::ARGS_IS_ARRAY) {
        if let Some(call_array) = call_array {
          return Ok(Some(call_array))
        }
      } else if message_enum.intersects(MessageFlags::ARGS_IN_ARRAY) {
        if let Some(mut call_array) = call_array {
          if !call_array.is_empty() {
            if let Value::Array(args) = call_array.remove(0) {
              return Ok(Some(args))
            }
          }
        }
      } else {
        return Ok(binary_method_call.args.map(|v| v.into_values()))
      }

      Err(nom::Err::Failure(error_position!(input, InvalidArgs)))
    };

    let args = parse_args(binary_method_call.message_enum)?;

    let method_call = MethodCall {
      method_name: binary_method_call.method_name.as_str(),
      type_name: binary_method_call.type_name.as_str(),
      call_context: binary_method_call.call_context.map(|c| c.as_str()),
      args,
    };

    Ok((input, method_call))
  }

  /// 2.7 Binary Record Grammar - `methodReturn`
  fn parse_method_return(
    &mut self,
    input: &'i [u8],
    root_id: Option<NonZeroU32>,
  ) -> IResult<&'i [u8], MethodReturn<'i>, Error<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, binary_method_return) = BinaryMethodReturn::parse(input)?;

    let (input, call_array) = opt(|input| self.parse_call_array(input, root_id))(input)?;

    let parse_args = |message_enum: MessageFlags| {
      if message_enum.intersects(MessageFlags::ARGS_IS_ARRAY) {
        if let Some(call_array) = call_array {
          return Ok(Some(call_array.clone()))
        }
      } else if message_enum.intersects(MessageFlags::ARGS_IN_ARRAY) {
        if let Some(mut call_array) = call_array {
          if !call_array.is_empty() {
            if let Value::Array(args) = call_array.remove(0) {
              return Ok(Some(args))
            }
          }
        }
      } else {
        return Ok(binary_method_return.args.map(|v| v.into_values()))
      }

      Err(nom::Err::Failure(error_position!(input, InvalidArgs)))
    };

    let args = parse_args(binary_method_return.message_enum)?;

    let method_return = MethodReturn {
      return_value: binary_method_return.return_value.map(|v| v.into_value()),
      call_context: binary_method_return.call_context.map(|c| c.as_str()),
      args,
    };

    Ok((input, method_return))
  }

  /// 2.7 Binary Record Grammar - `(methodCall/methodReturn)`
  fn parse_method_call_or_return(
    &mut self,
    input: &'i [u8],
    root_id: Option<NonZeroU32>,
  ) -> IResult<&'i [u8], MethodCallOrReturn<'i>, Error<'i>> {
    alt_mut!(input =>
      map(|input| self.parse_method_call(input, root_id), MethodCallOrReturn::MethodCall),
      map(|input| self.parse_method_return(input, root_id), MethodCallOrReturn::MethodReturn),
    )
  }

  fn parse_referenceables(&mut self, mut input: &'i [u8]) -> IResult<&'i [u8], (), Error<'i>> {
    loop {
      match self.parse_referenceable(input) {
        Ok((input2, _)) => {
          input = input2;
        },
        Err(nom::Err::Incomplete(n)) => return Err(nom::Err::Incomplete(n)),
        Err(nom::Err::Error(_)) => break,
        Err(nom::Err::Failure(err)) => {
          return Err(nom::Err::Failure(err));
        },
      }
    }

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `remotingMessage`
  fn parse_remoting_message(&mut self, input: &'i [u8]) -> IResult<&'i [u8], RemotingMessage<'i>, Error<'i>> {
    let (mut input, header) = SerializationHeader::parse(input)?;

    (input, ()) = self.parse_referenceables(input)?;

    let (mut input, method_call_or_return) =
      opt(|input| self.parse_method_call_or_return(input, header.root_id))(input)?;

    (input, ()) = self.parse_referenceables(input)?;

    let (input, MessageEnd) = MessageEnd::parse(input)?;

    if !input.is_empty() {
      return Err(nom::Err::Error(error_position!(input, TrailingData)))
    }

    let remoting_message = match method_call_or_return {
      Some(MethodCallOrReturn::MethodCall(method_call)) => RemotingMessage::MethodCall(method_call),
      Some(MethodCallOrReturn::MethodReturn(method_return)) => RemotingMessage::MethodReturn(method_return),
      None => {
        if let Some(root_id) = header.root_id {
          if let Some(root_object) = self.objects.remove(&root_id) {
            RemotingMessage::Value(root_object)
          } else {
            return Err(nom::Err::Error(error_position!(input, MissingRootObject)))
          }
        } else {
          RemotingMessage::Value(Value::Null)
        }
      },
    };

    Ok((input, remoting_message))
  }

  /// Deserializes a [`RemotingMessage`] from bytes.
  pub fn deserialize(mut self, input: &'i [u8]) -> Result<RemotingMessage<'i>, Error> {
    self.parse_remoting_message(input).map(|(_, remoting_message)| remoting_message).map_err(|err| match err {
      nom::Err::Incomplete(_) => Error { input, inner: ErrorInner::Eof },
      nom::Err::Error(err) | nom::Err::Failure(err) => err,
    })
  }
}
