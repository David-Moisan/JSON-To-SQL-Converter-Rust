use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{Flex, TextBox, Button};
use druid::Lens;
use std::fs::File;
use std::io::Write;
use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    table_name: String
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
        }
    }
}

impl druid::Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.table_name.same(&other.table_name)
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("JSON to SQL Converter")
        .window_size((800.0, 400.0));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState::default())
}

fn convert_and_save(ctx: &mut druid::EventCtx, data: &mut AppState, file_content: String) {
    if let Ok(parsed_json) = serde_json::from_str::<Vec<Value>>(&file_content) {
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
    } else {
        ctx.submit_command(crate::COMMAND_SHOW_ERROR.with(data.clone()));
    }
}

fn build_ui() -> Box<dyn Widget<AppState>> {
    let table_name = TextBox::new()
        .with_placeholder("Enter table name")
        .lens(TableNameLens);

    let convert_button = Button::new("Convert")
        .on_click(|ctx, data: &mut crate::AppState, _| {
            let file_content = r#"[
                {"name": "John", "age": 30, "city": "New York"},
                {"name": "Alice", "age": 25, "city": "San Francisco"}
            ]"#
            .to_string();
            convert_and_save(ctx, data, file_content);
        });

    let content: Box<dyn Widget<_>> = Box::new(
        Flex::column()
            .with_child(table_name)
            .with_spacer(10.0)
            .with_child(convert_button)
            .boxed()   
    );

    content
}

pub const COMMAND_CONVERT: druid::Selector<AppState> = druid::Selector::new("app.convert");
pub const COMMAND_SHOW_MESSAGE: druid::Selector<AppState> = druid::Selector::new("app.show-message");
pub const COMMAND_SHOW_ERROR: druid::Selector<AppState> = druid::Selector::new("app.show-error");