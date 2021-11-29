use std::sync::{Arc, RwLock};
use std::{thread, time};

use schemars::{schema, schema_for};
use serde_json::json;
use webthing::{
  BaseProperty, BaseThing, Thing,
  property::ValueForwarder,
};

use crate::{VControl, DataType, types::{DateTime, Error, CircuitTimes}};

struct VcontrolValueForwarder {
  command: &'static str,
  vcontrol: Arc<RwLock<VControl>>,
}

impl ValueForwarder for VcontrolValueForwarder {
  fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
    log::info!("Setting property {} to {}.", self.command, value);
    Ok(value)
  }
}

pub fn make_thing(vcontrol: VControl) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
  let device = vcontrol.device();
  let commands = device.commands().clone();

  let vcontrol = Arc::new(RwLock::new(vcontrol));

  // TODO: Get from `vcontrol`.
  let device_id = 1234;

  let mut thing = BaseThing::new(
    format!("urn:dev:ops:heating-{}", device_id),
    device.name().to_owned(),
    Some(vec!["ObjectProperty".to_owned()]),
    None,
  );

  for (command_name, command) in commands {
    let mut root_schema = match command.data_type {
      DataType::Int => schema_for!(i64),
      DataType::Double => schema_for!(f64),
      DataType::Byte => schema_for!(u8),
      DataType::String => schema_for!(String),
      DataType::DateTime => schema_for!(DateTime),
      DataType::Error => schema_for!(Error),
      DataType::CircuitTimes => schema_for!(CircuitTimes),
      DataType::ByteArray => schema_for!(Vec<u8>),
    };

    if command.mode.is_read() && !command.mode.is_write() {
      root_schema.schema.metadata().read_only = true;
    }

    if command.mode.is_write() && !command.mode.is_read() {
      root_schema.schema.metadata().write_only = true;
    }

    root_schema.schema.extensions.insert("@type".into(), json!("LevelProperty"));

    if let Some(unit) = &command.unit {
      root_schema.schema.extensions.insert("unit".into(), json!(unit));
    }

    let create_enum = |enum_schema: &mut schema::SchemaObject, mapping: &'static phf::Map<i32, &'static str>| {
      // Use `oneOf` schema in order to add description for enum values.
      // https://github.com/json-schema-org/json-schema-spec/issues/57#issuecomment-815166515
      let subschemas = mapping.entries().map(|(k, v)| {
        schema::SchemaObject {
          const_value: Some(json!(k)),
          metadata: Some(Box::new(schemars::schema::Metadata {
            description: Some(v.to_string()),
            ..Default::default()
          })),
          ..Default::default()
        }.into()
      }).collect();

      enum_schema.subschemas = Some(Box::new(
        schemars::schema::SubschemaValidation {
          one_of: Some(subschemas),
          ..Default::default()
        }
      ));
    };

    if let Some(mapping) = &command.mapping {
      create_enum(&mut root_schema.schema, mapping);
    } else if command.data_type == DataType::Error {
      if let Some(ref mut validation) = root_schema.schema.object {
        if let Some(schema::Schema::Object(index_schema)) = validation.properties.get_mut("index") {
          create_enum(index_schema, device.errors());
        }
      }
    };

    let schema = serde_json::to_value(root_schema).unwrap().as_object().unwrap().clone();
    let mut description = schema;

    let value_forwarder = VcontrolValueForwarder {
      command: command_name.clone(),
      vcontrol: vcontrol.clone(),
    };

    thing.add_property(Box::new(BaseProperty::new(
      command_name.to_string(),
      json!(null),
      Some(Box::new(value_forwarder)),
      Some(description),
    )));
  }

  let thing: Box<dyn Thing + 'static> = Box::new(thing);
  let thing = Arc::new(RwLock::new(thing));
  let weak_thing = Arc::downgrade(&thing);

  thread::spawn(move || {
    loop {
      let thing = if let Some(thing) = weak_thing.upgrade() {
        thing
      } else {
        return
      };

      for (command_name, command) in commands {
        if !command.mode.is_read() {
          continue;
        }

        let new_value = if let Ok(value) = vcontrol.write().unwrap().get(command_name) {
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

  thing
}
