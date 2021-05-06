// web_sys_mod.rs
//! helper functions for web_sys, window, document, dom, console,
//! local_storage, session_storage,...

// region: use
use unwrap::unwrap;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, Response};
// endregion: use

/// Simple macro to set listener of on_click events to an element_id.  
/// fn with 1 arg(element_id): on_click!(element_id, function_ident)
#[macro_export]
macro_rules! on_click {
    ($element_id: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($element_id);
        }) as Box<dyn FnMut()>);

        let html_element = crate::web_sys_mod::get_html_element_by_id($element_id);
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// list contains rows, every row item needs a separate event handler
/// the element Id is concatenation of element_prefix plus the row_number
#[macro_export]
macro_rules! row_on_click {
    ($element_prefix: expr, $row_number: expr, $function_ident: ident) => {{
        let closure = Closure::wrap(Box::new(move || {
            $function_ident($element_prefix, $row_number);
        }) as Box<dyn FnMut()>);

        let html_element = crate::web_sys_mod::get_html_element_by_id(&format!(
            "{}{}",
            $element_prefix, $row_number
        ));
        html_element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }};
}

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    let element_opt = document.get_element_by_id(element_id);
    if element_opt.is_none() {
        debug_write(&format!("Error: element not exists: {}", element_id));
    }
    unwrap!(element_opt)
}

/// get html element by id
pub fn get_html_element_by_id(element_id: &str) -> web_sys::HtmlElement {
    let element = get_element_by_id(element_id);
    let html_element: web_sys::HtmlElement = unwrap!(element.dyn_into::<web_sys::HtmlElement>());
    // return
    html_element
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}

/// get text from element_id
pub fn get_text(element_id: &str) -> String {
    let div = get_html_element_by_id(element_id);
    div.inner_text()
}

/// set text to element_id
pub fn set_text(element_id: &str, text: &str) {
    let div = get_html_element_by_id(element_id);
    div.set_inner_text(text);
}

/// set inner html to element_id
pub fn set_inner_html(element_id: &str, inner_html: &str) {
    let div = get_element_by_id(element_id);
    div.set_inner_html(inner_html);
}

/// fetch in Rust with async await for executor spawn_local()
/// return the response as String. Any error will panic.
pub async fn fetch_response(url: &str) -> String {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(url, &opts));
    // log1("before fetch");
    let resp_jsvalue = unwrap!(JsFuture::from(window().fetch_with_request(&request)).await);
    // log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    // log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    // debug_write(&txt_response);
    // returns response as String
    txt_response
}
