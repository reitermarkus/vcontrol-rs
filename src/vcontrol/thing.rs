use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::thread;

use schemars::{schema, schema_for};
use serde_json::json;
use webthing::{
  BaseProperty, BaseThing, Thing,
  property::ValueForwarder,
};

use crate::{VControl, Device, AccessMode, Command, DataType, Value, types::{DeviceId, DeviceIdF0, Date, DateTime, Error, CircuitTimes}};

struct VcontrolValueForwarder {
  command_name: &'static str,
  command: &'static Command,
  vcontrol: Arc<RwLock<Mutex<VControl>>>,
}

impl ValueForwarder for VcontrolValueForwarder {
  fn set_value(&mut self, value: serde_json::Value) -> Result<serde_json::Value, &'static str> {
    let new_value = value.clone();

    let vcontrol_value = match self.command.data_type {
      DataType::DeviceId => serde_json::from_value::<DeviceId>(value).map(Value::DeviceId),
      DataType::DeviceIdF0 => serde_json::from_value::<DeviceIdF0>(value).map(Value::DeviceIdF0),
      DataType::Int => serde_json::from_value::<i64>(value).map(Value::Int),
      DataType::Double => serde_json::from_value::<f64>(value).map(Value::Double),
      DataType::Byte | DataType::ErrorIndex => serde_json::from_value::<u8>(value).map(|b| Value::Int(b as i64)),
      DataType::String => serde_json::from_value::<String>(value).map(Value::String),
      DataType::Date => serde_json::from_value::<Date>(value).map(Value::Date),
      DataType::DateTime => serde_json::from_value::<DateTime>(value).map(Value::DateTime),
      DataType::Error => serde_json::from_value::<Error>(value).map(Value::Error),
      DataType::CircuitTimes => serde_json::from_value::<CircuitTimes>(value).map(Value::CircuitTimes),
      DataType::ByteArray => serde_json::from_value::<Vec<u8>>(value).map(Value::ByteArray),
    };

    let vcontrol_value = match vcontrol_value {
      Ok(vcontrol_value) => vcontrol_value,
      Err(err) => {
        log::error!("Failed setting property {}: {}", self.command_name, err);
        return Err("Parsing value failed")
      }
    };

    log::info!("Setting property {} to {}.", self.command_name, new_value);
    let vcontrol = self.vcontrol.write().unwrap();
    let mut vcontrol = vcontrol.lock().unwrap();
    if let Err(err) = vcontrol.set(self.command_name, vcontrol_value) {
      log::error!("Failed setting property {}: {}", self.command_name, err);
      return Err("Failed setting value")
    }
    log::info!("Property {} successfully set to {}.", self.command_name, new_value);

    Ok(new_value)
  }
}

fn add_command(thing: &mut dyn Thing, vcontrol: Arc<RwLock<Mutex<VControl>>>, device: &Device, command_name: &'static str, command: &'static Command) {
  let mut root_schema = match command.data_type {
    DataType::DeviceId => schema_for!(DeviceId),
    DataType::DeviceIdF0 => schema_for!(DeviceIdF0),
    DataType::Int => schema_for!(i64),
    DataType::Double => schema_for!(f64),
    DataType::Byte | DataType::ErrorIndex => schema_for!(u8),
    DataType::String => schema_for!(String),
    DataType::Date => schema_for!(Date),
    DataType::DateTime => schema_for!(DateTime),
    DataType::Error => schema_for!(Error),
    DataType::CircuitTimes => schema_for!(CircuitTimes),
    DataType::ByteArray => schema_for!(Vec<u8>),
  };

  match command.mode {
    AccessMode::Read => {
      root_schema.schema.metadata().read_only = true;
    },
    AccessMode::Write => {
      root_schema.schema.metadata().write_only = true;
    },
    AccessMode::ReadWrite => (),
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
  } else if command.data_type == DataType::ErrorIndex {
    create_enum(&mut root_schema.schema, device.errors());
  } else if command.data_type == DataType::Error {
    if let Some(ref mut validation) = root_schema.schema.object {
      if let Some(schema::Schema::Object(index_schema)) = validation.properties.get_mut("index") {
        create_enum(index_schema, device.errors());
      }
    }
  };

  let schema = serde_json::to_value(root_schema).unwrap().as_object().unwrap().clone();
  let description = schema;

  let value_forwarder = VcontrolValueForwarder {
    command_name: command_name.clone(),
    command: command.clone(),
    vcontrol: vcontrol.clone(),
  };

  thing.add_property(Box::new(BaseProperty::new(
    command_name.to_string(),
    json!(null),
    Some(Box::new(value_forwarder)),
    Some(description),
  )));
}

pub fn make_thing(vcontrol: VControl) -> Arc<RwLock<Box<dyn Thing + 'static>>> {
  let device = vcontrol.device();
  let mut commands = HashMap::new();

  for (command_name, command) in crate::commands::system_commands() {
    commands.insert(command_name, command);
  }

  for (command_name, command) in device.commands() {
    commands.insert(command_name, command);
  }

  let vcontrol = Arc::new(RwLock::new(Mutex::new(vcontrol)));

  // TODO: Get from `vcontrol`.
  let device_id = 1234;

  let mut thing = BaseThing::new(
    format!("urn:dev:ops:heating-{}", device_id),
    device.name().to_owned(),
    Some(vec!["ObjectProperty".to_owned()]),
    None,
  );

  for (command_name, command) in &commands {
    add_command(&mut thing, vcontrol.clone(), &device, command_name, command);
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

      for (command_name, command) in &commands {
        if !command.mode.is_read() {
          continue;
        }

        let new_value = {
          let vcontrol = vcontrol.read().unwrap();
          let mut vcontrol = vcontrol.lock().unwrap();
          match vcontrol.get(command_name) {
            Ok(value) => json!(value.value),
            Err(err) => {
              log::error!("Failed getting value for property '{}': {}", command_name, err);
              json!(null)
            }
          }
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
