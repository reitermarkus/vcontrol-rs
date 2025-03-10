use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

use base64::prelude::*;
use encoding_rs_io::DecodeReaderBytes;
use glob::glob;

mod raw;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut cultures = BTreeMap::<u8, String>::new();
  let mut translations = BTreeMap::<String, BTreeMap<String, String>>::new();

  for text_resource in glob("src/Textresource_*.xml")? {
    let text_resource = text_resource?;

    let f = File::open(text_resource)?;
    let decoder = DecodeReaderBytes::new(f);
    let io = BufReader::new(decoder);

    let document: raw::DocumentElement = quick_xml::de::from_reader(io)?;

    for culture in document.cultures.culture {
      cultures.insert(culture.id, culture.name);
    }

    for text_resource in document.text_resources.text_resource {
      let name = cultures.get(&text_resource.culture_id).unwrap();

      let value = raw::parse_translation_text(text_resource.value);
      let value = raw::clean_enum_text(&text_resource.label, None, value);

      let inner = translations.entry(text_resource.label).or_insert_with(BTreeMap::new);
      inner.insert(name.clone(), value);
    }
  }

  let f = File::create("translations.raw.yml")?;
  serde_yaml::to_writer(f, &translations)?;

  return Ok(());

  let input = env::args().nth(1).unwrap();
  dbg!(&input);

  let input = BASE64_STANDARD.decode(input)?;
  let message = nrbf::RemotingMessage::parse(&input).unwrap();

  dbg!(message);

  Ok(())
}
