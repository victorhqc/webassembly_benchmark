use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.webassembly.self";

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    status: EntryStatus,
}

#[derive(PartialEq, Serialize, Deserialize, ToString)]
pub enum EntryStatus {
    New,
    Completed,
    Editing,
}

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Couldn't create Storage service");
        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };

        let state = State {
            entries,
            filter: Filter::All,
            value: String::from(""),
            edit_value: String::from(""),
        };

        Self {
            link,
            storage,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let entry = Entry {
                    description: self.state.value.clone(),
                    status: EntryStatus::New,
                };

                self.state.entries.push(entry);
                self.state.value = String::from("");
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.clone();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = String::from("");
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Edit Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::Nope => {}
        }

        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("Rendered!");

        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{"todos"}</h1>
                        { self.view_input() }
                    </header>
                    <section class="main">
                        <input
                            class="toggle-all"
                            type="checkbox"
                            checked=self.state.is_all_completed()
                            onclick=self.link.callback(|_| Msg::ToggleAll )
                        />
                        <ul class="todo-list">
                            {
                                for self.state.entries.iter().filter(|e| self.state.filter.fit(e))
                                .enumerate()
                                .map(|val| self.view_entry(val))
                            }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            <b>{ self.state.total() }</b>
                            {" item(s) left "}
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|filt| self.view_filter(filt)) }
                        </ul>
                        <button class="clear-completed" onclick=self.link.callback(|_| Msg::ClearCompleted)>
                            { format!("Clear completed {}", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{"Double-click to edit a todo"}</p>
                    <p>
                        { "Written by " }
                        <a
                            href="https://github.com/DenisKolodin/"
                            target="_blank">
                            { "Denis Kolodin" }
                        </a>
                    </p>
                    <p>
                        { "Part of " }
                        <a href="http://todomvc.com/" target="_blank">{ "TodoMVC" }</a>
                    </p>
                </footer>
            </div>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter) -> Html {
        let flt = filter.clone();

        html! {
            <li>
                <a
                    class=if self.state.filter == flt { "selected" } else { "not-selected" }
                    href=&flt
                    onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone()))>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                value=&self.state.value
                oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                onkeypress=self.link.callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                })
            />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = String::from("todo");
        match entry.status {
            EntryStatus::Editing => {
                class.push_str(" editing");
            }
            EntryStatus::Completed => {
                class.push_str(" completed");
            }
            _ => {}
        };

        html! {
            <li class=class>
                <div class="view">
                    <input
                        class="toggle"
                        type="checkbox"
                        checked=entry.status == EntryStatus::Completed
                        onclick=self.link.callback(move |_| Msg::Toggle(idx))
                    />
                    <label ondoubleclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>
                        {&entry.description}
                    </label>
                    <button
                        class="destroy"
                        onclick=self.link.callback(move |_| Msg::Remove(idx))
                    />
                </div>
                { self.view_entry_edit_input((&idx, &entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (&usize, &Entry)) -> Html {
        let idx = *idx;
        if entry.status == EntryStatus::Editing {
            html! {
                <input
                    class="edit"
                    type="text"
                    value=&entry.description
                    oninput=self.link.callback(move |e: InputData| Msg::UpdateEdit(e.value))
                    onblur=self.link.callback(move |_| Msg::Edit(idx))
                    onkeypress=self.link.callback(move |e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                    })
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => {
                entry.status == EntryStatus::New || entry.status == EntryStatus::Editing
            }
            Filter::Completed => entry.status == EntryStatus::Completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();
        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.status == EntryStatus::Completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) && entry.status != EntryStatus::Editing {
                if value {
                    entry.status = EntryStatus::Completed
                } else {
                    entry.status = EntryStatus::New
                }
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();

        match entry.status {
            EntryStatus::Completed => {
                entry.status = EntryStatus::New;
            }
            EntryStatus::New => {
                entry.status = EntryStatus::Completed;
            }
            EntryStatus::Editing => {}
        };
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();

        match entry.status {
            EntryStatus::Completed => {}
            EntryStatus::New => {
                entry.status = EntryStatus::Editing;
            }
            EntryStatus::Editing => {
                entry.status = EntryStatus::New;
            }
        };
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.description = val;
        entry.status = EntryStatus::New;
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };

        self.entries.remove(idx);
    }
}
