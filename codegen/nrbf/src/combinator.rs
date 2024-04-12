use nom::{
  Err,
};

pub fn into_failure<E>(err: Err<E>) -> nom::Err<E> {
  match err {
    Err::Error(e) => Err::Failure(e),
    err => err,
  }
}
