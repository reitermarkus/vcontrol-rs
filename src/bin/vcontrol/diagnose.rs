use std::{collections::HashMap, error::Error, io};

use serde_json::{Map, json};
use vcontrol::{EventCategory, Optolink, VControl, Value};

const ROOT_CATEGORY_NAME: &str = "Diagnose Anlage";

pub async fn diagnose(optolink: Optolink) -> Result<(), Box<dyn Error>> {
  let mut vcontrol = VControl::connect(optolink).await?;

  log::info!("Connected to '{}' via {} protocol.", vcontrol.device().name(), vcontrol.protocol());

  let categories = vcontrol.device().event_categories();
  let root_category_id = categories
    .iter()
    .find(|category| category.name() == ROOT_CATEGORY_NAME)
    .map(EventCategory::id)
    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("Category '{ROOT_CATEGORY_NAME}' not found")))?;

  let mut ids = vec![root_category_id];
  let mut index = 0;
  while index < ids.len() {
    let parent = ids[index];
    ids.extend(categories.iter().filter(|category| category.parent_id() == Some(parent)).map(|category| category.id()));
    index += 1;
  }

  let mut values: HashMap<u32, Map<String, serde_json::Value>> = HashMap::new();
  let mut had_errors = false;

  for category in categories.iter().filter(|category| ids.contains(&category.id())) {
    let entry = values.entry(category.id()).or_default();
    for (name, _command) in category.event_types() {
      let new_value = match vcontrol.get(name).await {
        Ok(value) => match value.value {
          Value::Int(number) => value
            .mapping
            .as_ref()
            .and_then(|mapping| mapping.get(&(number as i32)))
            .map_or(json!(number), |mapped_value| json!(mapped_value)),
          other_value => json!(other_value),
        },
        Err(error) => {
          had_errors = true;
          log::error!("Failed getting value for property '{}': {}", name, error);
          json!(null)
        },
      };
      entry.insert(name.to_string(), new_value);
    }
  }

  fn build(
    categories: &[EventCategory],
    values: &mut HashMap<u32, Map<String, serde_json::Value>>,
    id: u32,
  ) -> serde_json::Value {
    let category = categories.iter().find(|category| category.id() == id).unwrap();
    let children: Vec<_> = categories
      .iter()
      .filter(|category| category.parent_id() == Some(id))
      .map(|category| build(categories, values, category.id()))
      .collect();
    json!({
      "id": id,
      "name": category.name(),
      "values": values.remove(&id).unwrap_or_default(),
      "categories": children,
    })
  }

  let tree = build(categories, &mut values, root_category_id);
  println!("{}", serde_json::to_string_pretty(&tree)?);

  if had_errors {
    return Err(io::Error::other("One or more diagnostic values could not be read").into());
  }

  Ok(())
}
