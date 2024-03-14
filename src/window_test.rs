// use druid::widget::{Button, Column, TextBox};
// use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};



// fn main() {
//     // Describe the main window
//     let main_window = WindowDesc::new(build_ui)
//         .title(LocalizedString::new("Simple Rust UI"))
//         .window_size((300.0, 150.0));

//     // Create the initial app state
//     let initial_state = AppState {
//         username: String::new(),
//     };

//     // Start the application
//     AppLauncher::with_window(main_window)
//         .launch(initial_state)
//         .expect("Failed to launch application");
// }

// // Define the application state
// #[derive(Clone, Default)]
// struct AppState {
//     username: String,
// }

// // Define the UI structure
// fn build_ui() -> impl Widget<AppState> {
//     // Create text input for username
//     let username_input = TextBox::new()
//         .with_placeholder("Enter your username...")
//         .expand_width()
//         .lens(AppState::username);

//     // Create button to greet user
//     let greet_button = Button::new("Greet")
//         .on_click(|_ctx, data: &mut String, _env| {
//             druid::window::show_dialog("Greeting", &format!("Hello Mr, {}!!!", data));
//         });

//     // Combine the widgets into a column layout
//     Column::new()
//         .with_child(username_input)
//         .padding(10.0)
//         .with_child(greet_button)
//         .padding(10.0)
// }