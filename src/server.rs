use std::sync::{Arc, RwLock, Weak};
use std::{thread, time};

use serde_json::json;
use webthing::property::ValueForwarder;
use webthing::server::ActionGenerator;
use webthing::{
    Action, BaseAction, BaseEvent, BaseProperty, BaseThing, Thing, ThingsType, WebThingServer,
};

use crate::{VControl, DataType};

struct Generator;

impl ActionGenerator for Generator {
  fn generate(
    &self,
    thing: Weak<RwLock<Box<dyn Thing>>>,
    name: String,
    input: Option<&serde_json::Value>,
  ) -> Option<Box<dyn Action>> {
    None
  }
}

fn make_thing(vcontrol: &mut VControl) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
  let mut thing = BaseThing::new(
      "urn:dev:ops:heating-1234".to_owned(),
      vcontrol.device().name().to_owned(),
      Some(vec!["ObjectProperty".to_owned()]),
      None,
  );

  for (command_name, command) in vcontrol.device().commands() {
    let mut description = serde_json::Map::new();
    description.insert("@type".into(), json!("LevelProperty"));
    description.insert("type".into(), json!(match command.data_type {
      DataType::Int | DataType::Double | DataType::Byte => "number",
      DataType::String | DataType::DateTime => "string",
      DataType::Error | DataType::CircuitTimes => "object",
      DataType::ByteArray => "array",
    }));

    let create_enum = |enum_description: &mut serde_json::Map<String, serde_json::Value>, mapping: &'static phf::Map<i32, &'static str>| {
      enum_description.insert("enum_values".into(), json!(mapping));
      enum_description.insert("enum".into(), json!(mapping.keys().collect::<Vec<_>>()));
    };

    if let Some(mapping) = &command.mapping {
      create_enum(&mut description, mapping);
    } else if command.data_type == DataType::Error {
      let mut enum_description = serde_json::Map::new();
      enum_description.insert("type".into(), json!("number"));
      create_enum(&mut enum_description, vcontrol.device().errors());

      description.insert("required".into(), json!(["index", "time"]));
      description.insert("properties".into(), json!({
        "index": enum_description,
        "time": { "type": "string" },
      }));
    };

    if let Some(unit) = &command.unit {
      description.insert("unit".into(), json!(unit));
    }

    description.insert("readOnly".into(), json!(!command.mode.is_write()));

    thing.add_property(Box::new(BaseProperty::new(
      command_name.to_string(),
      json!(null),
      None,
      Some(description),
    )));
  }

  Arc::new(RwLock::new(Box::new(thing)))
}

pub struct Server {
  port: u16,
}

impl Server {
  pub fn new() -> Self {
    Self { port: 8888 }
  }

  pub async fn start(&self, mut vcontrol: VControl) -> std::io::Result<()> {
    let thing = make_thing(&mut vcontrol);

    let mut server = WebThingServer::new(
      ThingsType::Single(thing.clone()),
      Some(self.port),
      None,
      None,
      Box::new(Generator),
      None,
      Some(true),
    );

    thread::spawn(move || {
      loop {
        for (command_name, command) in vcontrol.device().commands() {
          if !command.mode.is_read() {
            continue;
          }

          let new_value = if let Ok(value) = vcontrol.get(command_name) {
            serde_json::to_value(&value.value).unwrap()
          } else {
            json!(null)
          };

          let mut t = thing.write().unwrap();
          let prop = t.find_property(&command_name.to_string()).unwrap();

          let old_value = prop.get_value();

          if let Err(err) = prop.set_cached_value(new_value.clone()) {
            log::error!("Failed setting cached value for property '{}': {}", command_name, err)
          }

          if old_value != new_value {
            log::info!("Property '{}' changed from {} to {}.", command_name, old_value, new_value);
          }

          t.property_notify(command_name.to_string(), new_value);
        }
      }
    });

    server.start(None).await
  }
}
