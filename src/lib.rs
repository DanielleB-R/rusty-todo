#![allow(clippy::wildcard_imports)]

mod model;

use crate::model::{Model, TodoItem};
use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        items: vec![
            TodoItem {
                name: "Implement the Model field".to_owned(),
                checked: true,
            },
            TodoItem {
                name: "Implement the view".to_owned(),
                checked: false,
            },
            TodoItem {
                name: "Implement actions".to_owned(),
                checked: false,
            },
        ],
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
    Decrement,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => (),
        Msg::Decrement => (),
    }
}

// ------ ------
//     View
// ------ ------

static CHECKED: &str = "✔️";
static UNCHECKED: &str = "❌";

fn todo_item_view(item: &TodoItem) -> Node<Msg> {
    li![if item.checked { CHECKED } else { UNCHECKED }, &item.name]
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![div![
        h1!["Todo List"],
        ul![model
            .items
            .iter()
            .map(todo_item_view)
            .collect::<Vec<Node<Msg>>>()]
    ]]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
