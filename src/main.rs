use chrono::Local;
use dioxus::prelude::*;
use todos::Todo;

static CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut input = use_signal(|| String::new());
    let mut todos: Signal<Vec<Todo>> = use_signal(|| Vec::new());
    let mut next_id = use_signal(|| 1i32);

    let add = move |event: FormEvent| {
        event.prevent_default();

        let title = input();
        if title.trim().is_empty() {
            return;
        }

        let id = next_id();
        todos.write().push(Todo {
            id,
            title,
            created: Local::now().to_utc(),
            completed: None,
        });

        next_id += 1;
        input.set(String::new());
    };

    let mut toggle_complete = move |id: i32| {
        todos
            .write()
            .iter_mut()
            .find(|todo| todo.id == id)
            .map(|todo| {
                todo.completed = if todo.completed.is_some() {
                    None
                } else {
                    Some(Local::now().to_utc())
                };
            });
    };

    let mut delete = move |id: i32| {
        todos.write().retain(|todo| todo.id != id);
    };

    rsx! {
        document::Stylesheet { href: CSS }

        main {
            class: "m-4",
            h1 {
                class: "flex justify-between items-center mb-4",
                span {
                    class: "font-bold text-xl underline",
                    "Todos"
                }
                {
                    let completed = todos.read().iter().filter(|x| x.completed.is_some()).count();
                    let all = todos.len();

                    rsx! {
                        span {
                            class: "text-gray-600",
                            "{completed}/{all}"
                        }
                    }
                }
            }
            div {
                class: "space-y-2 mb-4",
                for todo in todos.read().iter() {
                    {
                        let todo_id = todo.id;
                        let is_completed = todo.completed.is_some();
                        let todo_title = todo.title.clone();

                        rsx! {
                            div {
                                class: "flex justify-between p-2 bg-white rounded-lg hover:shadow-sm transition-shadow",
                                key: "{todo_id}",
                                span {
                                    class: if is_completed { "line-through text-gray-500" },
                                    class: "cursor-pointer",
                                    onclick: move |_| toggle_complete(todo_id),
                                    "{todo_title}"
                                }
                                button {
                                    class: "text-red-500 cursor-pointer focus:outline-none focus:text-read-800",
                                    onclick: move |_| delete(todo_id),
                                    "del"
                                }
                            }
                        }
                    }
                }
            }
            form {
                onsubmit: add,
                class: "flex justify-between gap-4",
                div {
                    class: "flex-1",
                    label {
                        class: "text-sm text-gray-600",
                        "enter a todo: "
                    }
                    input {
                        class: "w-min-[50%] border-b border-gray-300 focus:outline-none focus:border-black",
                        oninput: move |event| input.set(event.value()),
                        value: "{input}",
                    }
                }
                button {
                    class: "underline decoration-gray-300 hover:decoration-black focus:decoration-black focus:outline-none",
                    type: "submit",
                    "add"
                }
            }
        }
    }
}
