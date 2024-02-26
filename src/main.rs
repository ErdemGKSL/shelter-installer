use std::io::{Read, Write};
use std::path::PathBuf;
use platform_dirs::AppDirs;
use ascii::ToAsciiChar;
use tokio::time::sleep;

#[derive(Clone)]
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

impl From<&str> for Type {
  fn from(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "ptb" | "discordptb" | "p" => Self::PTB,
      "can" | "canary" | "c" => Self::CAN,
      _ => Self::STD,
    }
  }
}

const DOWNLOAD_FILES: [&str; 3] = [
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/index.js",
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/preload.js",
  "https://raw.githubusercontent.com/uwu/shelter/main/injectors/desktop/app/package.json"
];

#[tokio::main]
async fn main() {
  let args: Vec<_> = std::env::args().into_iter().collect();
  let arg = args.get(1);
  let discord_type: Type = arg.map(|s| s.to_string()).unwrap_or_else(|| {
    println!("\
     Type D for standart discord
     Type P for discord ptb
     Type C for discord canary\
    ");
    
    wait_key("Type: ")
  }).as_str().into();
  
  println!("Type: {}", discord_type.as_dirname());
  let result = toggle_inject(discord_type).await;
  println!("{result:?}");
}

async fn toggle_inject(discord_type: Type) -> Result<(), String> {
  let (resources, installed) = get_resources_folder(discord_type.clone(), true).or(get_resources_folder(discord_type, false))?;

  let app_path = resources.join("app");

  if installed {
    std::fs::remove_dir_all(app_path).map_err(|_| "Couldn't remove app folder in the resources while uninstalling.")?;

    std::fs::rename(
      resources.join("original.asar"),
      resources.join("app.asar")
    ).map_err(|_| "Couldn't rename the original.asar in the resources while uninstalling.")?;

    return Ok(());
  }

  std::fs::rename(
    resources.join("app.asar"),
    resources.join("original.asar")
  ).map_err(|_| "Couldn't rename the app.asar in the resources while installing.")?;

  std::fs::create_dir_all(&app_path)
      .map_err(|_| "Couldn't create app directory while installing.")?;

  for file_url in DOWNLOAD_FILES {
    let file_name = file_url.split("/").last().unwrap();

    download_file(app_path.join(file_name), file_url.to_string()).await?;
  }
  Ok(())
}

async fn download_file(path: PathBuf, url: String) -> Result<(), String> {
  let content = reqwest::get(url.clone())
      .await
      .map_err(|_| format!("Couldn't fetch data from {url}"))?
      .text()
      .await
      .map_err(|_| format!("Couldn't parse data from {url}"))?;

  std::fs::write(&path, content)
      .map_err(|_| format!("Couldn't write data to {}", path.display()))?;

  Ok(())
}

fn get_data_dir(xdg: bool) -> Result<PathBuf, String> {
  let user_dirs = AppDirs::new(None, xdg);
  Ok(user_dirs.ok_or("Couldn't find data directory")?.data_dir)
}

fn get_resources_folder(discord_type: Type, xdg: bool) -> Result<(PathBuf, bool), String> {
  let app_data = get_data_dir(xdg)?;

  let app_data_path = app_data.as_path();
  let app_path = app_data_path.join(discord_type.as_dirname());

  let app_path = app_path.as_path();
  
  let dir = std::fs::read_dir(app_path).map_err(|_| "Couldn't read the directory")?;
  
  let dir = dir
      .filter(|entry| {
        if let Ok(entry) = entry {
          entry.file_name().to_str().unwrap_or("").to_string().starts_with("app-1.") &&
              (
                entry.path().join("resources/app.asar").exists() ||
                    entry.path().join("resources/original.asar").exists()
              )
        } else {
          false
        }
      })
      .map(|entry| {
        let entry = entry.unwrap();
        entry.path()
      })
      .max()
      .ok_or("Couldn't find valid app version directory with asar file")?;

  let resources_dir = dir.join("resources");
  let is_installed = resources_dir.join("original.asar").exists();

  Ok((
    resources_dir,
    is_installed
  ))
}

fn wait_key(msg: &str) -> String {
  std::io::stdout().flush().unwrap();
  print!("{msg}");
  std::io::stdout().flush().unwrap();
  let mut buffer = [0u8; 1];
  std::io::stdin().read_exact(&mut buffer).unwrap_or(());
  
  (*buffer.get(0).unwrap_or(&0)).to_ascii_char().unwrap().to_string().to_lowercase()
}