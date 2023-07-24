use ambient_api::{
    components::core::{
        layout::{align_vertical_center, space_between_items},
        rendering::color,
        text::font_style,
        wasm::{module, module_enabled, module_name, module_on_server},
    },
    prelude::*,
};

use super::{use_hotkey_toggle, use_input_request, Window};
use crate::messages::{self, WasmSetEnabled};

#[element_component]
pub fn WasmManager(hooks: &mut Hooks) -> Element {
    let (visible, set_visible) = use_hotkey_toggle(hooks, VirtualKeyCode::F3);

    FocusRoot::el([Window::el(
        "WASM Manager".to_string(),
        visible,
        Some(cb(move || set_visible(false))),
        WasmManagerInner::el(),
    )])
}

#[element_component]
fn WasmManagerInner(hooks: &mut Hooks) -> Element {
    use_input_request(hooks);

    let mut modules: Vec<_> = hooks
        .use_query((module(), module_name(), module_enabled()))
        .into_iter()
        .map(|(id, (_, name, enabled))| {
            (
                id,
                (entity::has_component(id, module_on_server()), name, enabled),
            )
        })
        .collect();
    modules.sort_by_key(|(_, (is_server, name, _))| (*is_server, name.clone()));

    let (server_modules, client_modules): (Vec<_>, Vec<_>) = modules
        .into_iter()
        .partition(|(_, (is_server, _, _))| *is_server);

    fn render_modules(heading: String, modules: Vec<(EntityId, (bool, String, bool))>) -> Element {
        FlowColumn::el(Iterator::chain(
            [Text::el(heading).with(font_style(), "Bold".to_string())].into_iter(),
            modules.into_iter().map(|(id, (_, name, enabled))| {
                FlowRow::el([
                    Checkbox::new(enabled, move |value| {
                        WasmSetEnabled::new(value, id).send_server_reliable();
                    })
                    .el(),
                    Text::el(name).with_default(align_vertical_center()),
                    Button::new(FontAwesomeIcon::el(0xf2f1, true), move |_| {
                        messages::WasmReload::new(id).send_server_reliable();
                    })
                    .style(ButtonStyle::Flat)
                    .el(),
                ])
            }),
        ))
        .with(space_between_items(), 4.0)
    }

    FlowRow::el([
        render_modules("Server modules".to_string(), server_modules),
        render_modules("Client modules".to_string(), client_modules),
    ])
    .with_margin_even(STREET)
    .with(space_between_items(), STREET)
}
