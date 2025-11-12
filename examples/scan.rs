use std::{env, error::Error};

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

  let mut file = OpenOptions::new().read(true).append(true).create(true).open("scan-cache.bin").await?;
  let mut content = Vec::with_capacity(u16::MAX as usize);
  BufReader::new(&mut file).read_to_end(&mut content).await?;
  let mut file = BufWriter::new(file);

  let protocol = Protocol::detect(&mut optolink).await.unwrap();

  let mut addr = u16::try_from(content.len()).unwrap();
  let mut buf = [0; 119]; // FIXME: `get` gets stuck with a buffer larger than 119 bytes for some reason.
  while addr < u16::MAX {
    let mut stdout = io::stdout();
    let output = format!("\r{addr}/{} ({addr:#04X})", u16::MAX);
    stdout.write_all(output.as_bytes()).await?;
    stdout.flush().await?;

    let buf_len = buf.len().min((u16::MAX - addr) as usize);
    let buf = &mut buf[..buf_len];

    loop {
      match protocol.get(&mut optolink, addr, buf).await {
        Ok(()) => {
          file.write_all(buf).await?;
          file.flush().await?;
        },
        Err(e) => {
          eprintln!("Error: {}", e);
          protocol.negotiate(&mut optolink).await?;
          continue;
        },
      }

      break;
    }

    addr += buf_len as u16;
  }

  println!();

  Ok(())
}
