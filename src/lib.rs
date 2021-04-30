// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 7,
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

struct TodoItem {
    name: String,
    checked: bool,
}

struct Model {
    counter: i32,
    items: Vec<TodoItem>,
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
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
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
    div![
        div![
            "This is a counter: ",
            C!["counter"],
            model.counter,
            button!["Increment", ev(Ev::Click, |_| Msg::Increment),],
            button!["Decrement", ev(Ev::Click, |_| Msg::Decrement),]
        ],
        div![
            h1!["Todo List"],
            ul![model
                .items
                .iter()
                .map(todo_item_view)
                .collect::<Vec<Node<Msg>>>()]
        ]
    ]
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
