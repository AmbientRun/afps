use std::sync::{Arc, Mutex};

use ambient_api::{
    components::core::{
        layout::{docking_bottom, docking_fill, fit_horizontal_parent, margin, min_height},
        rendering::color,
        text::font_style,
    },
    prelude::*,
};
use shared::*;

mod shared;

#[main]
pub fn main() {
    let console = Console::new(false);
    {
        let mut console = console.lock().unwrap();
        let engine = console.engine();
        engine.register_fn("server", |input: &str| {
            messages::ConsoleServerInput {
                input: input.to_string(),
            }
            .send_server_reliable();
        });
    }

    messages::ConsoleServerOutput::subscribe({
        let console = console.clone();
        move |source, msg| {
            if !source.server() {
                return;
            }

            console.lock().unwrap().push(
                ConsoleLine {
                    text: msg.text,
                    ty: ConsoleLineType::try_from(msg.ty).unwrap(),
                    is_server: msg.is_server,
                },
                None,
            );
        }
    });

    App { console }.el().spawn_interactive();
}

#[element_component]
pub fn App(hooks: &mut Hooks, console: Arc<Mutex<Console>>) -> Element {
    let (toggle, set_toggle) = hooks.use_state(false);
    hooks.use_keyboard_input(move |_, keycode, modifiers, pressed| {
        if modifiers == ModifiersState::empty() && keycode == Some(VirtualKeyCode::F1) && !pressed {
            set_toggle(!toggle);
        }
    });

    if toggle {
        ConsoleView::el(console)
    } else {
        Element::new()
    }
}

#[element_component]
pub fn ConsoleView(hooks: &mut Hooks, console: Arc<Mutex<Console>>) -> Element {
    hooks.use_spawn(|_| {
        messages::RequestInput {}.send_local_broadcast(false);
        |_| {
            messages::ReleaseInput {}.send_local_broadcast(false);
        }
    });

    let render_signal = hooks.use_rerender_signal();
    hooks.use_spawn({
        let console = console.clone();
        move |_| {
            console.lock().unwrap().on_output(move || render_signal());
            move |_| {
                console.lock().unwrap().clear_on_output();
            }
        }
    });
    let (command, set_command) = hooks.use_state_with(|_| String::new());

    FocusRoot::el([WindowSized::el([with_rect(Dock::el([
        // text entry
        TextEditor::new(command, set_command.clone())
            .auto_focus()
            .placeholder(Some("Enter command..."))
            .on_submit({
                let console = console.clone();
                move |text| {
                    console.lock().unwrap().input(&text, |_| {});
                    set_command(String::new());
                }
            })
            .el()
            .with_background(vec4(0.0, 0.0, 0.0, 0.5))
            .with_padding_even(4.0)
            .with_default(fit_horizontal_parent())
            .with_default(docking_bottom())
            .with(min_height(), 22.0)
            .with(margin(), vec4(STREET, STREET, 0.0, STREET)),
        // log
        ScrollArea::el(
            ScrollAreaSizing::FitParentWidth,
            FlowColumn::el({
                let console = console.lock().unwrap();
                console.lines().iter().map(line_to_text).collect::<Vec<_>>()
            })
            .with_padding_even(4.0),
        )
        .with_background(vec4(0.0, 0.0, 0.0, 0.5))
        .with_default(docking_fill())
        .with_margin_even(STREET),
    ]))
    .with_background(vec4(0.0, 0.0, 0.0, 0.5))])
    .with_padding_even(20.)])
}

fn line_to_text(line: &ConsoleLine) -> Element {
    let (line_font_style, line_color) = match line.ty {
        ConsoleLineType::Normal => ("Regular", vec3(0.8, 0.8, 0.8)),
        ConsoleLineType::User => (
            if line.is_server { "Regular" } else { "Bold" },
            vec3(0.0, 0.8, 0.0),
        ),
        ConsoleLineType::Error => ("Regular", vec3(0.8, 0.0, 0.0)),
    };

    let line_color = if line.is_server {
        line_color * 0.75
    } else {
        line_color
    };

    let text = if line.is_server {
        format!("[s] {}", line.text)
    } else {
        line.text.clone()
    };

    Text::el(text)
        .with(font_style(), line_font_style.to_string())
        .with(color(), line_color.extend(1.0))
}
