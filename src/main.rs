use std::fmt::format;
use std::fs::FileType;
use std::io::Read;
use platform_dirs::{AppDirs, UserDirs};

fn main() {
  
  match inject() {
    Ok(result) => {
      println!("{result}");
    }
    Err(result) => {
      println!("ERROR: {result}");
    }
  }
 
  wait_key();
}

enum Type {
  STD,
  PTB,
  CAN
}

impl Type {
  pub fn as_dirname(&self) -> String {
    match self {
      Type::STD => "Discord".to_string(),
      Type::PTB => "DiscordPTB".to_string(),
      Type::CAN => "DiscordCanary".to_string()
    }
  }
}

const TYPE: Type = Type::CAN;

fn inject() -> Result<String, String> {
  let user_dirs = AppDirs::new(None, false);
  let app_data = user_dirs.ok_or("Couldn't find data directoryy")?.data_dir;
  println!("Local appdata path: {}", app_data.display());
  
  let app_data_path = app_data.as_path();
  let app_path = app_data_path.join(TYPE.as_dirname());

  let app_path = app_path.as_path();
  
  let dir = std::fs::read_dir(app_path).map_err(|_| "Couldn't read the directory")?;
  
  let dir = dir
      .filter(|entry| {
        if let Ok(entry) = entry {
          entry.file_name().to_str().unwrap_or("").to_string().starts_with("app-1.")
        } else {
          false
        }
      })
      .map(|entry| {
        let entry = entry.unwrap();
        entry.path()
      })
      .max()
      .ok_or("Couldn't find app version directory")?;
  
  println!("{:?}", dir);
  
  unimplemented!("everything works fine but bro wait this isnt done yet!??")
}

fn wait_key() {
  println!("Press any key to continue...");
  let mut buffer = [0u8; 1];
  std::io::stdin().read_exact(&mut buffer).unwrap();
}