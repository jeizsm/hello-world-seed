#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;
#[macro_use]
extern crate seed;

use wasm_bindgen::prelude::*;
use seed::prelude::*;


cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Model

#[derive(Clone)]
struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {val: model.val + 1}
    }
}


// View

fn view(model: Model) -> El<Msg> {
    button![
        simple_ev("click", Msg::Increment),
        format!("Hello, World × {}", model.val)
    ]
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let div = document.create_element("div").unwrap();
    div.set_id("main");

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&div)?;
    seed::run(Model::default(), update, view, "main");

    Ok(())
}
