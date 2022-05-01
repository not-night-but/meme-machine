#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Read},
    ops::Add,
    path::Path,
    sync::Mutex,
};

use image::{io::Reader, DynamicImage, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusqlite::{params, Connection, Params};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use tauri::{InvokeError, State};
use uuid::Uuid;

struct AppState {
    initialized: Mutex<bool>,
    conn: Mutex<Option<DbConnection>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            initialized: Mutex::new(false),
            conn: Mutex::new(None),
        }
    }
    pub fn check(&self, app_handle: &tauri::AppHandle) {
        let mut initialized = self.initialized.lock().unwrap();
        if !*initialized {
            let app_dir = app_handle
                .path_resolver()
                .app_dir()
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace("\\\\?\\", "")
                .add(std::path::MAIN_SEPARATOR.to_string().as_str());

            fs::create_dir_all(app_dir.clone()).expect("Error creating directory");
            let user_memes_path = format!("{}meme_machine.db", app_dir);

            let initialize_database = !Path::new(user_memes_path.as_str()).exists();

            // Create connections to database.
            let mut conn = self.conn.lock().unwrap();
            *conn = Some(DbConnection::new(user_memes_path.as_str()));

            // Initialize database if they were just created.
            if initialize_database {
                if let Some(conn) = &*conn {
                    let file_name = app_handle
                        .path_resolver()
                        .resource_dir()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .replace("\\\\?\\", "")
                        .add(
                            format!(
                                "{sep}assets{sep}initialize_db.sql",
                                sep = std::path::MAIN_SEPARATOR
                            )
                            .as_str(),
                        );

                    let file = OpenOptions::new().read(true).open(file_name).unwrap();
                    let mut reader = BufReader::new(file);
                    let mut buf = vec![];

                    // Use read_until to read until EOF.
                    reader
                        .read_until(u8::MIN, &mut buf)
                        .expect("Error reading initialize_db.sql");

                    // Convert vector of bytes to string.
                    let init_query = String::from_utf8(buf).unwrap();

                    for query in init_query.split(";") {
                        if query.trim().is_empty() {
                            continue;
                        }
                        conn.raw_query(query, params![]);
                    }
                }
            }
        }

        *initialized = true;
    }
}

struct DbConnection {
    conn: Connection,
}

impl DbConnection {
    pub fn new(path: &str) -> Self {
        Self {
            conn: Connection::open(path).expect("Error opening database"),
        }
    }

    pub fn raw_query<P>(&self, query: &str, params: P)
    where
        P: Params,
    {
        self.conn.execute(query, params).unwrap();
    }

    pub fn insert<P>(&self, table: &str, columns: Vec<&str>, params: P)
    where
        P: Params,
    {
        let mut values = "(".to_owned();
        let mut cols = "(".to_owned();
        for i in 0..columns.len() {
            values = format!("{}${}, ", values, i + 1);
            cols = format!("{}{}, ", cols, columns[i]);
        }
        values = format!("{})", values.trim_end_matches(", "));
        cols = format!("{})", cols.trim_end_matches(", "));
        self.conn
            .execute(
                format!("INSERT INTO {} {} VALUES {}", table, cols, values).as_str(),
                params,
            )
            .expect("Error inserting into database");
    }

    pub fn get_user_memes(&self) -> Result<Vec<String>, Error> {
        Ok(self
            .conn
            .prepare("SELECT path FROM memes")?
            .query_map([], |row| row.get(0))?
            .map(|path| path.unwrap())
            .collect())
    }

    pub fn get_templates(&self) -> Result<Vec<MemeRecord>, Error> {
        Ok(self
            .conn
            .prepare(
                "SELECT name, image_path, text_instances, text_color, text_scale FROM templates",
            )?
            .query_map([], |row| {
                Ok(MemeRecord {
                    name: row.get(0)?,
                    image_path: row.get(1)?,
                    text_instances: serde_json::from_str(row.get::<usize, String>(2)?.as_str())
                        .unwrap(),
                    text_color: serde_json::from_str(row.get::<usize, String>(3)?.as_str())
                        .unwrap(),
                    text_scale: serde_json::from_str(row.get::<usize, String>(4)?.as_str())
                        .unwrap(),
                })
            })?
            .map(|record| record.unwrap())
            .collect())
    }
}

#[tauri::command]
fn get_user_memes(
    app_handle: tauri::AppHandle,
    state: State<AppState>,
) -> Result<Vec<String>, Error> {
    state.check(&app_handle);

    (*state.conn.lock().unwrap())
        .as_ref()
        .unwrap()
        .get_user_memes()
}

#[tauri::command]
fn get_templates(
    app_handle: tauri::AppHandle,
    state: State<AppState>,
) -> Result<Vec<MemeRecord>, Error> {
    state.check(&app_handle);

    (*state.conn.lock().unwrap())
        .as_ref()
        .unwrap()
        .get_templates()
}

#[tauri::command]
fn create_meme(
    app_handle: tauri::AppHandle,
    state: State<AppState>,
    input: Input,
) -> Result<(), Error> {
    state.check(&app_handle);

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
    let app_dir = app_handle
        .path_resolver()
        .app_dir()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .replace("\\\\?\\", "");

    let mut save_path = app_dir
        .clone()
        .add(format!("{sep}memes{sep}", sep = std::path::MAIN_SEPARATOR).as_str());

    fs::create_dir_all(save_path.clone()).expect("Error creating directory");
    save_path = save_path.add(format!("{}.png", Uuid::new_v4()).as_str());

    let mut meme = Meme::new(input.text_input, input.name, &asset_dir)?;
    meme.apply_text(&asset_dir)?;
    meme.image
        .to_rgba8()
        .save(save_path.clone())
        .expect("Error saving file");

    (*state.conn.lock().unwrap()).as_ref().unwrap().raw_query(
        r#"
        INSERT INTO memes (path) VALUES (?);
        "#,
        params![save_path],
    );

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_meme,
            get_user_memes,
            get_templates
        ])
        .manage(AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    DbError(String),
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

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::DbError(err.to_string())
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
