use std::collections::HashMap;

use tokio::{
  fs::OpenOptions,
  io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

use vcontrol::{Optolink, Protocol, VControl};

pub async fn scan(mut optolink: Optolink) -> Result<(), Box<dyn std::error::Error>> {
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
          content.extend_from_slice(buf);
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

  let vcontrol = VControl::connect(optolink).await?;

  let mut commands = HashMap::new();

  for (command_name, command) in vcontrol::commands::system_commands() {
    commands.insert(command.addr(), (command_name, command.block_len()));
  }

  for (command_name, command) in vcontrol.device().commands() {
    commands.insert(command.addr(), (command_name, command.block_len()));
  }

  let mut i = 0;

  while i < content.len() {
    if let Some((command_name, block_len)) = commands.get(&(i as u16)) {
      let bytes = &content[i..i + block_len];

      if bytes.iter().any(|&byte| byte != 0xff) {
        println!(
          "{i:04X} ({command_name}): {}",
          bytes.iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<String>>().join("")
        );
      }

      i += block_len;
    } else {
      let byte = content[i];
      if byte != 0xff && byte != 0x00 {
        println!("{i:04X} (unknown): {:02X}", byte);
      }

      i += 1;
    }
  }

  Ok(())
}
