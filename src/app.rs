use druid::{Command, Selector};

pub const COMMAND_CONVERT: Selector<AppState> = Selector::new("app.convert");

#[derive(Clone, Data, Default)]
pub struct AppState {
    pub file_path: String,
    pub table_name: String,
}

pub fn convert_and_save(state: &AppState) {
    println!("Converting JSON to SQL...");
    println!("File Path: {}", state.file_path);
    println!("Table Name: {}", state.table_name);
    println!("Conversion complete!");
}

pub fn convert_handler(data: &mut AppState, _cmd: &Command) {
    convert_and_save(data);
}