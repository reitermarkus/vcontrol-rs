pub fn into_failure<E>(err: nom::Err<E>) -> nom::Err<E> {
  match err {
    nom::Err::Error(e) => nom::Err::Failure(e),
    err => err,
  }
}
