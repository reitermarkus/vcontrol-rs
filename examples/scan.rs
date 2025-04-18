use std::{collections::BTreeMap, env, error::Error};

use tokio::{
  fs::OpenOptions,
  io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};
use vcontrol::{Optolink, Protocol};

// Scan all possible addresses and save their values in `scan-cache.yml`.
// Helpful for finding addresses for undocumented event types.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  env_logger::init();

  let optolink_port = env::args().nth(1).expect("no serial port specified");
  let mut optolink = if optolink_port.contains(':') {
    Optolink::connect(optolink_port).await
  } else {
    Optolink::open(optolink_port).await
  }?;

  let mut file = OpenOptions::new().read(true).append(true).create(true).open("scan-cache.yml").await?;
  let mut content = String::new();
  BufReader::new(&mut file).read_to_string(&mut content).await?;
  let cache: BTreeMap<u16, u8> = serde_yaml::from_str(&content).unwrap_or_default();
  let mut file = BufWriter::new(file);

  let protocol = Protocol::detect(&mut optolink).await.unwrap();

  for i in 0..u16::MAX {
    let mut stdout = io::stdout();
    let output = format!("\r{}/{}", i, u16::MAX);
    stdout.write_all(output.as_bytes()).await?;
    stdout.flush().await?;

    if cache.contains_key(&i) {
      continue;
    }

    loop {
      let addr = i as u16;
      let mut buf = [0];
      match protocol.get(&mut optolink, addr, &mut buf).await {
        Ok(()) => {
          let value = buf[0];
          let line = format!("0x{:04X}: {}", addr, value);
          file.write_all(line.as_bytes()).await?;
        },
        Err(e) => {
          eprintln!("Error: {}", e);
          protocol.negotiate(&mut optolink).await?;
          continue;
        },
      }

      break;
    }
  }

  println!();

  Ok(())
}
