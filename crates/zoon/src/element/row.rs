use wasm_bindgen::JsCast;
use crate::{RenderContext, dom::dom_element, __TrackedCall, __TrackedCallStack, Element, IntoElement, ApplyToElement, render, element_macro};
use dominator::{Dom, html, DomBuilder};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};

// ------ ------
//    Element 
// ------ ------

element_macro!(row, Row::default());

#[derive(Default)]
pub struct Row {
    key: u64,
    after_removes: Vec<Box<dyn FnOnce()>>,
    items: Vec<Dom>,
    items_signal: Option<Box<dyn SignalVec<Item = Dom> + Unpin>>
}

impl Element for Row {
    fn render(mut self) -> Dom {
        let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html("div")
            .class("row");

        if !self.items.is_empty() {
            builder = builder
                .children(self.items);
        }

        if let Some(items_signal) = self.items_signal {
            builder = builder
                .children_signal_vec(items_signal);
        }

        self.after_removes.push(Box::new(|| {
            crate::println!("Row after_remove");
        }));

        for after_remove in self.after_removes {
            builder = builder.after_removed(move |_| after_remove());
        }

        builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Row {
    pub fn after_remove(mut self, after_remove: impl FnOnce() + 'static) -> Self {
        self.after_removes.push(Box::new(after_remove));
        self
    }

    pub fn after_removes(mut self, after_removes: Vec<Box<dyn FnOnce()>>) -> Self {
        self.after_removes.extend(after_removes);
        self
    }

    pub fn item(mut self, item: impl IntoElement<'a> + 'a) -> Self {
        item.into_element().apply_to_element(&mut self);
        self
    }

    pub fn items<IE: IntoElement<'a> + 'a>(mut self, items: impl IntoIterator<Item = IE>) -> Self {
        for item in items.into_iter() {
            item.into_element().apply_to_element(&mut self);
        }
        self
    }

    pub fn items_signal<IE: IntoElement<'a> + 'a>(mut self, items: impl SignalVec<Item = IE> + Unpin + 'static) -> Self {
        self.items_signal = Some(Box::new(items.map(|item| item.into_element().render())));
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Row> for T {
    fn apply_to_element(self, row: &mut Row) {
        row.items.push(self.into_element().render());
    }
}
