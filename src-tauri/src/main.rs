#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    ops::Add,
    path::Path,
};

use image::{io::Reader, DynamicImage, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use tauri::InvokeError;
use uuid::Uuid;

#[tauri::command]
fn create_meme(app_handle: tauri::AppHandle, input: Input) -> Result<(), Error> {
    let resource_dir = app_handle
        .path_resolver()
        .resource_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace("\\\\?\\", "");
    let asset_dir = resource_dir
        .clone()
        .add(format!("{sep}assets", sep = std::path::MAIN_SEPARATOR).as_str());
    let meme_dir = resource_dir
        .clone()
        .add(format!("{sep}memes", sep = std::path::MAIN_SEPARATOR).as_str());
    let save_path = meme_dir.clone().add(
        format!(
            "{sep}{uuid}.jpg",
            uuid = Uuid::new_v4().to_string(),
            sep = std::path::MAIN_SEPARATOR,
        )
        .as_str(),
    );

    let mut user_memes = get_user_memes(&meme_dir);
    let mut meme = Meme::new(input.text_input, input.name, &asset_dir)?;
    meme.apply_text(&asset_dir)?;
    meme.image
        .to_rgba8()
        .save(save_path.clone())
        .expect("Error saving file");

    user_memes.push(save_path);
    update_user_memes(user_memes, &meme_dir);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_meme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_user_memes(meme_dir: &String) -> Vec<String> {
    let file_name = format!(
        "{dir}{sep}user_memes.json",
        dir = meme_dir,
        sep = std::path::MAIN_SEPARATOR
    );
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = vec![];

    // Use read_until to read until EOF.
    reader.read_until(u8::MIN, &mut buf);

    // Convert vector of bytes to string.
    let data = String::from_utf8(buf).unwrap();
    return serde_json::from_str(&data).unwrap();
}

fn update_user_memes(user_memes: Vec<String>, meme_dir: &String) {
    let file_name = format!(
        "{dir}{sep}user_memes.json",
        dir = meme_dir,
        sep = std::path::MAIN_SEPARATOR
    );
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_name)
        .unwrap();
    let mut writer = BufWriter::new(file);
    let data = serde_json::to_string(&user_memes).unwrap();
    writer.write_all(data.as_bytes()).unwrap();
}

#[derive(Deserialize)]
struct Input {
    text_input: Vec<String>,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemeRecord {
    pub name: String,
    image_path: String,
    text_instances: Vec<(u32, u32, u8)>,
    text_color: (u8, u8, u8),
    text_scale: (f32, f32),
}

impl MemeRecord {
    pub fn get_memes(path: &Path) -> Vec<MemeRecord> {
        let mut file = File::open(path).expect("Expected file called 'templates.json'");
        let mut buff = String::new();
        if let Ok(_) = file.read_to_string(&mut buff) {
            serde_json::from_str::<Vec<MemeRecord>>(&buff).expect("Error obtaining meme records")
        } else {
            vec![]
        }
    }
}

pub struct Meme {
    pub image: DynamicImage,
    text_input: Vec<String>,
    record: MemeRecord,
}

impl Meme {
    pub fn new(
        text_input: Vec<String>,
        name: String,
        resource_dir: &String,
    ) -> Result<Self, Error> {
        let memes =
            MemeRecord::get_memes(Path::new(&resource_dir.clone().add(
                format!("{sep}templates.json", sep = std::path::MAIN_SEPARATOR).as_str(),
            )));
        let record = memes.iter().find(|meme| meme.name == name);
        if let Some(record) = record {
            let image =
                Reader::open(resource_dir.clone().add(&record.image_path.clone()))?.decode()?;
            Ok(Meme {
                image,
                text_input,
                record: record.clone(),
            })
        } else {
            Err(Error::MemeError("Error obtaining meme record".to_string()))
        }
    }

    pub fn apply_text(&mut self, resource_dir: &String) -> Result<(), Error> {
        if self.text_input.len() != self.record.text_instances.len() {
            return Err(Error::MemeError(
                "Incorrect number of arguments for this meme".to_string(),
            ));
        }

        let scale = Scale {
            x: self.record.text_scale.0,
            y: self.record.text_scale.1,
        };
        let font = resource_dir
            .clone()
            .add(format!("{sep}Nasa21-l23X.ttf", sep = std::path::MAIN_SEPARATOR).as_str());
        let mut reader = BufReader::new(File::open(font).expect("Error finding font"));
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let font = Font::try_from_vec(buffer).unwrap();

        for i in 0..self.record.text_instances.len() {
            let text_instance = &self.record.text_instances[i];
            let text_input = &self.text_input[i].clone();
            let split = text_input.split(" ");
            let mut line_count = 0u8;
            let mut text_input = String::new();
            for word in split.into_iter() {
                if word.chars().count() as u8 + line_count > text_instance.2 {
                    text_input += &("\n".to_owned() + word + " ");
                    line_count = 0u8;
                } else {
                    text_input += &(word.to_owned() + " ");
                    line_count += word.chars().count() as u8;
                }
            }

            println!(
                "Applying text input: {} to image at position {:?}",
                text_input, text_instance
            );
            let mut y = text_instance.1;
            for line in text_input.lines() {
                draw_text_mut(
                    &mut self.image,
                    Rgba([
                        self.record.text_color.0,
                        self.record.text_color.1,
                        self.record.text_color.2,
                        1u8,
                    ]),
                    text_instance.0,
                    y,
                    scale,
                    &font,
                    line,
                );
                y += scale.y as u32;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub enum Error {
    IOError(String),
    ImageError(String),
    MemeError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.to_string())
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Error::ImageError(err.to_string())
    }
}

impl From<InvokeError> for Error {
    fn from(err: InvokeError) -> Self {
        Error::MemeError(format!("{:?}", err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::MemeError(err)
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}
