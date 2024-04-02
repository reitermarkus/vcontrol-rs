use std::{collections::BTreeMap, fmt, iter};

use nom::{
  branch::alt,
  combinator::{fail, map, map_opt, opt, verify},
  multi::many_m_n,
  IResult, ToUsize,
};
#[cfg(feature = "serde")]
use serde::{
  de::{self, value::Error, Expected, IntoDeserializer, Visitor},
  forward_to_deserialize_any, Deserializer,
};

use crate::{
  common::{AdditionalTypeInfo, MemberTypeInfo},
  data_type::{Int32, LengthPrefixedString},
  enumeration::{BinaryArrayType, BinaryType},
  grammar::Class,
  record::{
    ArraySingleObject, ArraySinglePrimitive, ArraySingleString, BinaryArray, BinaryLibrary, BinaryObjectString,
    ClassWithId, ClassWithMembers, ClassWithMembersAndTypes, MemberPrimitiveTyped, MemberPrimitiveUnTyped,
    MemberReference, ObjectNull, ObjectNullMultiple, ObjectNullMultiple256, SystemClassWithMembers,
    SystemClassWithMembersAndTypes,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectClass<'i> {
  pub name: &'i str,
  pub library: Option<&'i str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object<'i> {
  Object { class: ObjectClass<'i>, members: BTreeMap<&'i str, Object<'i>> },
  Array(Vec<Object<'i>>),
  Primitive(MemberPrimitiveUnTyped),
  String(&'i str),
  Null(usize),
  Ref(Int32),
}

struct ExpectedInArray(usize);

impl Expected for ExpectedInArray {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    if self.0 == 1 {
      formatter.write_str("1 element in array")
    } else {
      write!(formatter, "{} elements in array", self.0)
    }
  }
}

pub(crate) struct ObjectArrayDeserializer<'de, 'o, I> {
  objects: &'o BTreeMap<Int32, Object<'de>>,
  iter: iter::Fuse<I>,
  null_count: usize,
  count: usize,
}

impl<'de, 'o, I> ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator,
{
  pub fn new(objects: &'o BTreeMap<Int32, Object<'de>>, iter: I) -> Self {
    Self { objects, iter: iter.fuse(), null_count: 0, count: 0 }
  }
}

impl<'de, I> ObjectArrayDeserializer<'de, '_, I>
where
  I: Iterator,
{
  /// Check for remaining elements after passing a `SeqDeserializer` to
  /// `Visitor::visit_seq`.
  pub fn end<E: de::Error>(self) -> Result<(), E> {
    let remaining = self.iter.count() + self.null_count;
    if remaining == 0 {
      Ok(())
    } else {
      // First argument is the number of elements in the data, second
      // argument is the number of elements expected by the Deserialize.
      Err(de::Error::invalid_length(self.count + remaining, &ExpectedInArray(self.count)))
    }
  }
}

impl<'de, 'o, I> de::Deserializer<'de> for ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Object<'de>>,
{
  type Error = Error;

  fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    let v = visitor.visit_seq(&mut self)?;
    self.end()?;
    Ok(v)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

impl<'de, 'o, I> de::SeqAccess<'de> for ObjectArrayDeserializer<'de, 'o, I>
where
  I: Iterator<Item = &'o Object<'de>>,
{
  type Error = Error;

  fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    if self.null_count > 0 {
      self.count += 1;
      self.null_count -= 1;
      return seed.deserialize(Object::Null(1)).map(Some)
    }

    match self.iter.next() {
      Some(Object::Null(null_count @ 2..)) => {
        self.count += 1;
        self.null_count = null_count - 1;
        seed.deserialize(Object::Null(1)).map(Some)
      },
      Some(object) => {
        self.count += 1;
        seed.deserialize(ObjectDeserializer::new(self.objects, object)).map(Some)
      },
      None => Ok(None),
    }
  }
}

pub(crate) struct ObjectDeserializer<'de, 'o> {
  objects: &'o BTreeMap<Int32, Object<'de>>,
  object: &'o Object<'de>,
}

impl<'de, 'o> ObjectDeserializer<'de, 'o> {
  pub fn new(objects: &'o BTreeMap<Int32, Object<'de>>, object: &'o Object<'de>) -> Self {
    Self { objects, object }
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for ObjectDeserializer<'de, '_> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for ObjectDeserializer<'de, '_> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    use serde::de::{Error, Unexpected};

    match self.object {
      Object::Object { class, members } => {
        if class.library.is_some() {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        }

        let value = if let Some(value) = members.get("m_value") {
          value
        } else {
          return Err(Error::invalid_type(Unexpected::Other(class.name), &visitor))
        };

        match (class.name, value) {
          ("System.Boolean", Object::Primitive(MemberPrimitiveUnTyped::Boolean(n))) => visitor.visit_bool((*n).into()),
          ("System.Byte", Object::Primitive(MemberPrimitiveUnTyped::Byte(n))) => visitor.visit_u8((*n).into()),
          ("System.SByte", Object::Primitive(MemberPrimitiveUnTyped::SByte(n))) => visitor.visit_i8((*n).into()),
          ("System.Char", Object::Primitive(MemberPrimitiveUnTyped::Char(c))) => visitor.visit_char((*c).into()),
          ("System.Decimal", Object::Primitive(MemberPrimitiveUnTyped::Decimal(_c))) => unimplemented!(),
          ("System.Double", Object::Primitive(MemberPrimitiveUnTyped::Double(n))) => visitor.visit_f64((*n).into()),
          ("System.Single", Object::Primitive(MemberPrimitiveUnTyped::Single(n))) => visitor.visit_f32((*n).into()),
          ("System.Int32", Object::Primitive(MemberPrimitiveUnTyped::Int32(n))) => visitor.visit_i32((*n).into()),
          ("System.UInt32", Object::Primitive(MemberPrimitiveUnTyped::UInt32(n))) => visitor.visit_u32((*n).into()),
          ("System.Int64", Object::Primitive(MemberPrimitiveUnTyped::Int64(n))) => visitor.visit_i64((*n).into()),
          ("System.UInt64", Object::Primitive(MemberPrimitiveUnTyped::UInt64(n))) => visitor.visit_u64((*n).into()),
          ("System.Int16", Object::Primitive(MemberPrimitiveUnTyped::Int16(n))) => visitor.visit_i16((*n).into()),
          ("System.UInt16", Object::Primitive(MemberPrimitiveUnTyped::UInt16(n))) => visitor.visit_u16((*n).into()),
          (name, _) => Err(Error::custom(format!("invalid system type: {}", name))),
        }
      },
      Object::Array(members) => {
        ObjectArrayDeserializer::new(self.objects, members.into_iter()).deserialize_any(visitor)
      },
      Object::Ref(id) => {
        if let Some(object) = self.objects.get(&id) {
          Self::new(self.objects, object).deserialize_any(visitor)
        } else {
          Err(Error::invalid_value(Unexpected::Other("unresolved object ID"), &visitor))
        }
      },
      Object::Primitive(primitive) => primitive.clone().deserialize_any(visitor),
      Object::String(s) => visitor.visit_borrowed_str(s),
      Object::Null(1) => visitor.visit_none(),
      Object::Null(_) => Err(Error::invalid_value(Unexpected::Other("unresolved null object"), &visitor)),
    }
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

#[cfg(feature = "serde")]
impl<'de> IntoDeserializer<'de, Error> for Object<'de> {
  type Deserializer = Self;

  fn into_deserializer(self) -> Self::Deserializer {
    self
  }
}

#[cfg(feature = "serde")]
impl<'de> Deserializer<'de> for Object<'de> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: Visitor<'de>,
  {
    ObjectDeserializer::new(&Default::default(), &self).deserialize_any(visitor)
  }

  forward_to_deserialize_any! {
      bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
      bytes byte_buf option unit unit_struct newtype_struct seq tuple
      tuple_struct map struct enum identifier ignored_any
  }
}

#[derive(Debug, Default)]
pub struct BinaryParser<'i> {
  pub binary_libraries: BTreeMap<Int32, LengthPrefixedString<'i>>,
  pub classes: BTreeMap<Int32, Class<'i>>,
  pub objects: BTreeMap<Int32, Object<'i>>,
}

impl<'i> BinaryParser<'i> {
  pub fn parse_binary_library(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, binary_library) = opt(verify(BinaryLibrary::parse, |binary_library| {
      !self.binary_libraries.contains_key(&binary_library.library_id())
    }))(input)?;

    if let Some(binary_library) = binary_library {
      self.binary_libraries.insert(binary_library.library_id(), binary_library.library_name);
    }

    Ok((input, ()))
  }

  fn parse_binary_object_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, s) = BinaryObjectString::parse(input)?;

    self.objects.insert(s.object_id(), Object::String(s.as_str()));

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `memberReference`
  pub fn parse_member_reference(
    &mut self,
    input: &'i [u8],
    type_enum_and_additional_type_info: Option<(BinaryType, Option<&AdditionalTypeInfo<'i>>)>,
  ) -> IResult<&'i [u8], Object<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, object)) =
      if let Some((type_enum, additional_type_info)) = type_enum_and_additional_type_info {
        match (type_enum, additional_type_info) {
          (BinaryType::Primitive, Some(AdditionalTypeInfo::Primitive(primitive_type))) => {
            map(|input| MemberPrimitiveUnTyped::parse(input, *primitive_type), |p| (None, Object::Primitive(p)))(input)?
          },
          (BinaryType::String, None) => {
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str())))(input)?
          },
          (BinaryType::Object, None) => return self.parse_member_reference(input, None),
          (BinaryType::SystemClass, Some(class_name)) => unimplemented!("{class_name:?}"),
          (BinaryType::Class, Some(class_type_info)) => {
            unimplemented!("{class_type_info:?}")
          },
          (BinaryType::ObjectArray, None) => return self.parse_member_reference(input, None),
          (BinaryType::StringArray, None) => alt((
            map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str()))),
            map(MemberReference::parse, |member_reference| (None, Object::Ref(member_reference.id_ref))),
            map(|input| Self::parse_null_object(input), |null_object| (None, null_object)),
          ))(input)?,
          (BinaryType::PrimitiveArray, Some(AdditionalTypeInfo::Primitive(_primitive_type))) => {
            map(MemberReference::parse, |member_reference| (None, Object::Ref(member_reference.id_ref)))(input)?
          },
          _ => unreachable!(),
        }
      } else {
        alt((
          map(MemberPrimitiveTyped::parse, |p| (None, Object::Primitive(p.into_untyped()))),
          map(MemberReference::parse, |member_reference| (None, Object::Ref(member_reference.id_ref))),
          map(BinaryObjectString::parse, |s| (Some(s.object_id()), Object::String(s.as_str()))),
          map(|input| Self::parse_null_object(input), |null_object| (None, null_object)),
          map(|input| self.parse_classes(input), |o| (None, o)),
        ))(input)?
      };

    let object = if let Some(object_id) = object_id {
      self.objects.insert(object_id, object);
      Object::Ref(object_id)
    } else {
      object
    };

    Ok((input, object))
  }

  fn parse_members_with_type_info(
    &mut self,
    mut input: &'i [u8],
    member_type_info: &MemberTypeInfo<'i>,
  ) -> IResult<&'i [u8], Vec<Object<'i>>> {
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

  /// 2.7 Binary Record Grammar - `Classes`
  pub fn parse_classes(&mut self, input: &'i [u8]) -> IResult<&'i [u8], Object<'i>> {
    let (input, ()) = self.parse_binary_library(input)?;

    let (input, (object_id, class)) = verify(
      alt((
        map_opt(ClassWithId::parse, |class| {
          let object_id = class.object_id();
          self.classes.get(&class.metadata_id()).map(|class| (object_id, class.clone()))
        }),
        map(verify(ClassWithMembers::parse, |class| self.binary_libraries.contains_key(&class.library_id)), |class| {
          (class.object_id(), Class::ClassWithMembers(class))
        }),
        map(
          verify(ClassWithMembersAndTypes::parse, |class| self.binary_libraries.contains_key(&class.library_id)),
          |class| (class.object_id(), Class::ClassWithMembersAndTypes(class)),
        ),
        map(SystemClassWithMembers::parse, |class| (class.object_id(), Class::SystemClassWithMembers(class))),
        map(SystemClassWithMembersAndTypes::parse, |class| {
          (class.object_id(), Class::SystemClassWithMembersAndTypes(class))
        }),
      )),
      |(object_id, _)| !self.classes.contains_key(&object_id),
    )(input)?;

    let (input, (object_class, member_references)) = match class {
      Class::ClassWithMembers(ref class) => {
        let object_class = ObjectClass {
          name: class.class_info().name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::ClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass {
          name: class.class_info().name.as_str(),
          library: Some(self.binary_libraries[&class.library_id].as_str()),
        };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembers(ref class) => {
        let object_class = ObjectClass { name: class.class_info().name.as_str(), library: None };

        let member_count = class.class_info().member_names.len();
        let (input, member_references) =
          many_m_n(member_count, member_count, |input| self.parse_member_reference(input, None))(input)?;

        (input, (object_class, member_references))
      },
      Class::SystemClassWithMembersAndTypes(ref class) => {
        let object_class = ObjectClass { name: class.class_info().name.as_str(), library: None };

        let (input, member_references) = self.parse_members_with_type_info(input, &class.member_type_info)?;

        (input, (object_class, member_references))
      },
    };

    let class_info = class.class_info();
    let object = Object::Object {
      class: object_class,
      members: BTreeMap::from_iter(
        class_info
          .member_names
          .iter()
          .zip(member_references.iter().cloned())
          .map(|(member_name, member_value)| (member_name.as_str(), member_value)),
      ),
    };

    self.classes.insert(object_id, class);
    self.objects.insert(object_id, object.clone());

    Ok((input, object))
  }

  fn parse_array_single_object(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (mut input, array) = ArraySingleObject::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, None)?;

      let count = match member {
        Object::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    self.objects.insert(array.object_id(), Object::Array(members));

    Ok((input, ()))
  }

  fn parse_array_single_primitive(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, array) = ArraySinglePrimitive::parse(input)?;

    let length = array.array_info.len();
    let (input, members) = many_m_n(
      length,
      length,
      map(|input| MemberPrimitiveUnTyped::parse(input, array.primitive_type), |primitive| Object::Primitive(primitive)),
    )(input)?;

    self.objects.insert(array.object_id(), Object::Array(members));

    Ok((input, ()))
  }

  fn parse_array_single_string(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (mut input, array) = ArraySingleString::parse(input)?;

    let mut members = vec![];

    let mut len_remaining = array.array_info.len();
    while len_remaining > 0 {
      let member;
      (input, member) = self.parse_member_reference(input, Some((BinaryType::StringArray, None)))?;

      let count = match member {
        Object::Null(count) => count,
        _ => 1,
      };

      members.push(member);
      len_remaining -= count;
    }

    self.objects.insert(array.object_id(), Object::Array(members));

    Ok((input, ()))
  }

  fn parse_binary_array(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, array) = BinaryArray::parse(input)?;

    let member_count = match array.binary_array_type_enum {
      BinaryArrayType::Single | BinaryArrayType::SingleOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
      BinaryArrayType::Rectangular | BinaryArrayType::RectangularOffset => {
        array.lengths.iter().try_fold(1u32, |acc, n| acc.checked_mul(i32::from(*n) as u32))
      },
      BinaryArrayType::Jagged | BinaryArrayType::JaggedOffset => array.lengths.first().map(|n| i32::from(*n) as u32),
    };
    let member_count = match member_count {
      Some(member_count) => member_count.to_usize(),
      None => return fail(input),
    };
    let (input, members) = many_m_n(member_count, member_count, |input| {
      self.parse_member_reference(input, Some((array.type_enum, array.additional_type_info.as_ref())))
    })(input)?;

    self.objects.insert(array.object_id(), Object::Array(members));

    Ok((input, ()))
  }

  /// 2.7 Binary Record Grammar - `Arrays`
  pub fn parse_arrays(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    let (input, ()) = self.parse_binary_library(input)?;

    if let Ok((input, ())) = self.parse_array_single_object(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_array_single_primitive(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_array_single_string(input) {
      return Ok((input, ()))
    }

    self.parse_binary_array(input)
  }

  /// 2.7 Binary Record Grammar - `referenceable`
  pub fn parse_referenceable(&mut self, input: &'i [u8]) -> IResult<&'i [u8], ()> {
    if let Ok((input, _)) = self.parse_classes(input) {
      return Ok((input, ()))
    }

    if let Ok((input, ())) = self.parse_arrays(input) {
      return Ok((input, ()))
    }

    self.parse_binary_object_string(input)
  }

  /// 2.7 Binary Record Grammar - `nullObject`
  pub fn parse_null_object(input: &'i [u8]) -> IResult<&'i [u8], Object<'i>> {
    alt((
      map(ObjectNull::parse, |n| Object::Null(n.null_count())),
      map(ObjectNullMultiple::parse, |n| Object::Null(n.null_count())),
      map(ObjectNullMultiple256::parse, |n| Object::Null(n.null_count())),
    ))(input)
  }
}
