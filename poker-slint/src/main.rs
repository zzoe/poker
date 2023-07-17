slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let app_weak = app.as_weak();
    app.on_request_increase_value(move || {
        let ui = app_weak.unwrap();
        ui.set_counter(ui.get_counter() + 1);
    });

    app.run()
}
