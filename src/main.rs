use druid::commands::SHOW_OPEN_PANEL;
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc };
use druid::widget::{Flex, TextBox, Button};
use druid::Lens;
use druid::FileDialogOptions;
use druid_shell::FileSpec;
use std::fs::File;
use std::io::Write;
use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    table_name: String,
    file_content: String,
}

struct TableNameLens;

impl Lens<AppState, String> for TableNameLens {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &AppState, f: F) -> V {
        f(&data.table_name)
    }

    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut AppState, f: F) -> V {
        f(&mut data.table_name)
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            table_name: String::new(),
            file_content: String::new(),
        }
    }
}

impl druid::Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.table_name.same(&other.table_name) && self.file_content.same(&other.file_content)
    }
}

struct MyAppDelegate;

impl druid::AppDelegate<AppState> for MyAppDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        match cmd {
            x if x.is(crate::COMMAND_CONVERT) => {
                println!("Starting conversion...");
                if !data.file_content.is_empty() {
                    match serde_json::from_str::<Vec<Value>>(&data.file_content) {
                        Ok(parsed_json) => {
                            let table_name = &data.table_name;
                            let columns: Vec<&str> = parsed_json.first().unwrap().as_object().unwrap().keys().map(|s| s.as_ref()).collect();
            
                            let mut output_file = File::create(format!("{}.sql", table_name)).expect("Unable to create SQL File");
            
                            for i in 0..parsed_json.len() {
                                let item = &parsed_json[i];
                                let values: Vec<String> = columns.iter().map(|col| item[col].to_string()).collect();
                                let sql_query = format!("INSERT INTO {} ({}) VALUES ({});\n", table_name, columns.join(", "), values.join(", "));
                                output_file.write_all(sql_query.as_bytes()).expect("Unable to write to SQL file");
                            }
            
                            ctx.submit_command(crate::COMMAND_SHOW_MESSAGE.with(data.clone()));
                            println!("Conversion completed successfully");
                        }
                        Err(err) => {
                            ctx.submit_command(crate::COMMAND_SHOW_ERROR.with(data.clone()));
                            println!("Error parsing JSON: {:?}", err);
                        }
                    }
                } else {
                    println!("File content is empty. Cannot parse an empty JSON file.");
                }
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        }
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("JSON to SQL Converter")
        .window_size((800.0, 400.0));

    AppLauncher::with_window(main_window)
        .delegate(MyAppDelegate)
        .log_to_console()
        .launch(AppState::default())
}

fn build_ui() -> Box<dyn Widget<AppState>> {
    let table_name = TextBox::new()
        .with_placeholder("Enter table name")
        .lens(TableNameLens);

    let open_button = Button::new("Download JSON File")
        .on_click(|ctx, _data: &mut AppState, _env| {
            println!("Starting uploaded...");
            let json = FileSpec::new("JSON Files", &["json"]);
            let file_dialog_options = FileDialogOptions::new()
                .allowed_types(vec![json])
                .default_type(json);

            let file_dialog_options_clone = file_dialog_options.clone();
            ctx.submit_command(SHOW_OPEN_PANEL.with(file_dialog_options_clone));
        });

    let convert_button = Button::new("Convert")
        .on_click(|ctx, data: &mut AppState, _env| {
            ctx.submit_command(crate::COMMAND_CONVERT.with(data.clone()));
        });

    let content: Box<dyn Widget<_>> = Box::new(
        Flex::column()
            .with_spacer(100.0)
            .with_child(table_name)
            .with_spacer(100.0)
            .with_child(open_button)
            .with_spacer(50.0)
            .with_child(convert_button)
            .boxed()   
    );

    content
}

pub const COMMAND_CONVERT: druid::Selector<AppState> = druid::Selector::new("app.convert");
pub const COMMAND_SHOW_MESSAGE: druid::Selector<AppState> = druid::Selector::new("app.show-message");
pub const COMMAND_SHOW_ERROR: druid::Selector<AppState> = druid::Selector::new("app.show-error");
pub const COMMAND_SET_FILE_PATH: druid::Selector<String> = druid::Selector::new("app.set-file-path");