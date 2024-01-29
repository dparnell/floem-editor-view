use std::rc::Rc;

use floem::{
    keyboard::{Key, ModifiersState, NamedKey},
    reactive::{create_rw_signal, RwSignal, Scope},
    unit::UnitExt,
    view::View,
    views::{stack, Decorators},
};

use floem_text_editor::{
    editor::{
        editor::Editor,
        keypress::default_key_handler,
        text::{default_dark_color, SimpleStyling, TextDocument},
        view::editor_container_view,
    }
};

fn app_view(editor_a: RwSignal<Editor>, editor_b: RwSignal<Editor>) -> impl View {
    let view = stack((
        editor_container_view(editor_a, |_| true, default_key_handler(editor_a))
            .style(|s| s.height_pct(50.0)),
        editor_container_view(editor_b, |_| true, default_key_handler(editor_b))
            .style(|s| s.height_pct(50.0)),
    ))
    .style(|s| {
        s.size(100.pct(), 100.pct())
            .flex_col()
            .items_center()
            .justify_center()
    });

    let id = view.id();
    view.on_key_up(
        Key::Named(NamedKey::F11),
        ModifiersState::empty(),
        move |_| id.inspect(),
    )
}

fn main() {
    let cx = Scope::new();
    // The doc is the underlying content
    let doc = Rc::new(TextDocument::new(cx, "Hello World!"));
    // The style determines how things are styled in the editor itself
    let style = Rc::new(SimpleStyling::new(default_dark_color));
    // The editor is a view into the doc that can be rendered
    let editor_a = Editor::new(cx, doc.clone(), style.clone());
    let editor_a = create_rw_signal(editor_a);

    let editor_b = Editor::new(cx, doc.clone(), style.clone());
    let editor_b = create_rw_signal(editor_b);

    floem::launch(move || app_view(editor_a, editor_b));
}
