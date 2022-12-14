use futures::channel::mpsc::unbounded;
use futures::channel::{mpsc::UnboundedReceiver, oneshot::channel};
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, Element, HtmlElement};

pub struct BlightComponent {
  pub unbounded_receiver: UnboundedReceiver<()>,
}

impl BlightComponent {
  fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
    let (mut click_sender, click_receiver) = unbounded();
    let on_click = BlightComponent::closure_wrap(Box::new(move || {
      let _result: Result<(), futures::channel::mpsc::SendError> =
        click_sender.start_send(());
    }) as Box<dyn FnMut()>);
    elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();
    click_receiver
  }

  fn closure_wrap<T: WasmClosure + ?Sized>(data: Box<T>) -> Closure<T> {
    Closure::wrap(data)
  }

  pub fn initialize(id: &str) -> Option<Self> {
    let document: Document = window().unwrap().document().unwrap();
    let element_option: Option<Element> = document.get_element_by_id(id);
    if let Some(element) = element_option {
      let html_element: HtmlElement = element.dyn_into().unwrap();
      let unbounded_receiver: UnboundedReceiver<()> =
        BlightComponent::add_click_handler(html_element);
      return Some(Self {
        unbounded_receiver,
      });
    }
    None
  }

  pub fn make_html(id: &str) -> String {
    format!("<button id=\"{}\">Garden of Eden</button>", id)
  }
}
