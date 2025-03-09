use std::env;
use base64::prelude::*;

fn main() {
  let input = env::args().nth(1).unwrap();
  dbg!(&input);

  let input = BASE64_STANDARD.decode(input).unwrap();

  let message = nrbf::RemotingMessage::parse(&input).unwrap();

  dbg!(message);
}
