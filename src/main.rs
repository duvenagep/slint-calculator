use i_slint_backend_winit::winit::platform::macos::WindowBuilderExtMacOS;
use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let mut backend = i_slint_backend_winit::Backend::new().unwrap();
    backend.window_builder_hook = Some(Box::new(|builder| {
        builder
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true)
            .with_has_shadow(true)
    }));

    let _ = slint::platform::set_platform(Box::new(backend));

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let calc = Rc::new(RefCell::new(Calculator::default()));

    ui.global::<CalcLogic>().on_button_pressed(move |input| {
        let ui = ui_handle.unwrap();
        let mut calc = calc.borrow_mut();

        match input.as_str() {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => {
                if calc.operator.is_none() {
                    calc.lhs = calc.lhs.to_owned() + &input;
                    ui.set_result(calc.lhs.parse::<f32>().unwrap().into());
                    ui.set_clr("C".into());
                } else {
                    calc.rhs = calc.rhs.to_owned() + &input;
                    ui.set_result(calc.rhs.parse::<f32>().unwrap().into());
                    ui.set_clr("C".into());
                }
            }
            "+" | "-" | "x" | "÷" => {
                calc.operator = Some(input.to_owned().into());
                ui.set_font_size(45.0);
            }
            "=" => {
                let lhs = calc.lhs.parse::<f32>().unwrap();
                let rhs = calc.rhs.parse::<f32>().unwrap();
                calc.result = match calc.operator.as_ref().unwrap().as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "x" => lhs * rhs,
                    "÷" => lhs / rhs,
                    _ => 0.0,
                };
                ui.set_result(calc.result.into());
                calc.lhs = calc.result.to_string();
                calc.rhs = "".to_owned();
                calc.operator = None;
            }
            "C" => {
                calc.lhs = "".to_owned();
                calc.rhs = "".to_owned();
                calc.operator = None;
                calc.result = 0.0;
                ui.set_result(calc.result.into());
                ui.set_clr("AC".into());
                ui.set_font_size(45.0);
            }
            _ => {}
        }
        let rlen = ui.get_result().to_string().chars().count() as f32;
        match rlen.to_string().as_str() {
            "9" => ui.set_font_size(40.0),
            "10" => ui.set_font_size(38.0),
            "11" => ui.set_font_size(36.0),
            "12" => ui.set_font_size(35.0),
            "13" => ui.set_font_size(34.0),
            "14" => ui.set_font_size(33.0),
            "15" => ui.set_font_size(32.0),
            "16" => ui.set_font_size(30.0),
            "17" => ui.set_font_size(28.0),
            _ => {}
        }
    });

    ui.run()
}

#[derive(Debug, Clone, Default)]
struct Calculator {
    lhs: String,
    rhs: String,
    operator: Option<String>,
    result: f32,
}
