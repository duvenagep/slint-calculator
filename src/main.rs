use i_slint_backend_winit::winit::platform::macos::WindowBuilderExtMacOS;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mut backend = i_slint_backend_winit::Backend::new().unwrap();
    backend.window_builder_hook = Some(Box::new(|builder| {
        builder
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
    }));

    slint::platform::set_platform(Box::new(backend));
    let ui = AppWindow::new()?;
    let calc = ui.window();
    ui.run()
}

// ui.on_request_increase_value(move || {
//     let ui = ui_handle.unwrap();
//     ui.set_counter(ui.get_counter() + 1);
// });
