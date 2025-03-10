use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DefaultCulture {
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@DefaultCultureId")]
  #[allow(unused)]
  pub default_culture_id: u8,
}

#[derive(Debug, Deserialize)]
pub struct DefaultCultures {
  #[serde(rename = "DefaultCulture")]
  #[allow(unused)]
  pub default_culture: DefaultCulture,
}

#[derive(Debug, Deserialize)]
pub struct Culture {
  #[serde(rename = "@Id")]
  pub id: u8,
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Cultures {
  #[serde(default, rename = "Culture")]
  pub culture: Vec<Culture>,
}

#[derive(Debug, Deserialize)]
pub struct TextResource {
  #[serde(rename = "@CompanyId")]
  #[allow(unused)]
  pub company_id: u8,
  #[serde(rename = "@CultureId")]
  pub culture_id: u8,
  #[serde(rename = "@Label")]
  pub label: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct TextResources {
  #[serde(rename = "TextResource")]
  pub text_resource: Vec<TextResource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentElement {
  #[allow(unused)]
  pub default_cultures: DefaultCultures,
  pub cultures: Cultures,
  pub text_resources: TextResources,
}

pub fn parse_translation_text(text: String) -> String {
  lazy_static! {
    static ref WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
  }

  let text = text.trim();
  let text = WHITESPACE.replace_all(&text, " ");
  let text = text
    .replace("##ecnnewline##", "\n")
    .replace("##ecntab##", "\t")
    .replace("##ecnsemicolon##", ";")
    .replace("##nl##", "\n");

  text.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

pub fn clean_enum_text<'a, 'b, 'c>(translation_id: &'a str, index: Option<&'b str>, text: String) -> String {
  lazy_static! {
    static ref INDEX: Regex = Regex::new(r"^(?<name>.*)~(?<index>\d+)$").unwrap();
  }

  let index = if let Some(captures) = INDEX.captures(translation_id) {
    if captures.name("name").map(|m| m.as_str()) == Some("viessmann.eventvaluetype.K73_KonfiZpumpeIntervallFreigabe") {
      // False positive: <index> per hour
      None
    } else {
      captures.name("index").map(|m| m.as_str())
    }
  } else {
    index
  };

  let text = if let Some(index) = index {
    let index = regex::escape(index);

    let text =
      Regex::new(&format!(r"^(?:(?:{index}|\(0*{index}\))(?:\s*:|\s+-)?\s+)([^\s]+)")).unwrap().replace(&text, "$1");
    let text = Regex::new(r"^-*$").unwrap().replace(&text, "");
    text.into_owned()
  } else {
    text
  };

  text.trim().to_owned()
}
