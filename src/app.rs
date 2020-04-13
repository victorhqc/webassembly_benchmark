use crate::entries::Entries;
use crate::random_words::random_animal;
use log::*;
use regex::Regex;
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
    entries_bkp: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
    search_value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Entry {
    pub description: String,
    pub status: EntryStatus,
}

#[derive(PartialEq, Serialize, Deserialize, ToString, Clone)]
pub enum EntryStatus {
    New,
    Completed,
    Editing,
}

pub enum Msg {
    Add,
    Update(String),
    SetFilter(Filter),
    ToggleAll,
    ClearCompleted,
    UpdateSearch(String),
    Search,
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
                App::generate_random_todos(10_000)
                // Vec::new()
            }
        };

        let state = State {
            entries,
            entries_bkp: Vec::new(),
            filter: Filter::All,
            value: String::from(""),
            edit_value: String::from(""),
            search_value: String::from(""),
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
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
            }
            Msg::UpdateSearch(val) => {
                println!("Search Input: {}", val);
                self.state.search_value = val;
            }
            Msg::Search => {
                let search_value = self.state.search_value.clone();
                self.state.search_todos(search_value);
            }
            Msg::Nope => {}
        }

        self.storage.store(KEY, Json(&self.state.entries));
        true
    }

    fn view(&self) -> Html {
        info!("Rendered!");

        let mut footer_class = String::from("footer");
        match self.state.filter {
            Filter::Search(_) => {
                footer_class.push_str(" big_footer");
            }
            _ => {}
        };

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
                        <Entries entries=self.state.entries.clone() />
                    </section>
                    <footer class=footer_class>
                        <div class="row">
                            <span class="todo-count">
                                <b>{ self.state.total() }</b>
                                {" item(s) left "}
                            </span>
                            <ul class="filters">
                                { for Filter::iter().map(|filt| self.view_filter(filt)) }
                            </ul>
                            <button
                                class="clear-completed"
                                onclick=self.link.callback(|_| Msg::ClearCompleted)>
                                { format!("Clear completed {}", self.state.total_completed()) }
                            </button>
                        </div>
                        <div class="row">
                            { self.view_search_input() }
                        </div>
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
                        { "Modified by " }
                        <a
                            href="https://github.com/victorhqc"
                            target="_blank">
                            { "Victor Quiroz" }
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
    fn generate_random_todos(amount: u32) -> Vec<Entry> {
        let mut entries = Vec::new();
        for _ in 0..amount {
            let entry = Entry {
                description: random_animal(),
                status: EntryStatus::New,
            };
            entries.push(entry);
        }

        entries
    }

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

    fn view_search_input(&self) -> Html {
        match self.state.filter {
            Filter::Search(_) => {
                html! {
                    <input
                        class="search"
                        type="text"
                        placeholder="Search..."
                        value=&self.state.search_value
                        oninput=self.link.callback(move |e:InputData| Msg::UpdateSearch(e.value))
                        onblur=self.link.callback(|_| Msg::Search)
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Msg::Search } else { Msg::Nope }
                        })
                    />
                }
            }
            _ => {
                html! {}
            }
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Search(String),
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match &*self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
            Filter::Search(_) => "#/search".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match &*self {
            Filter::All => true,
            Filter::Active => {
                entry.status == EntryStatus::New || entry.status == EntryStatus::Editing
            }
            Filter::Completed => entry.status == EntryStatus::Completed,
            Filter::Search(val) => {
                let needle = Regex::new(&format!("(?i){}", val)).unwrap();
                let found = needle.find(&entry.description);

                match found {
                    Some(_) => true,
                    None => false,
                }
            }
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

    fn search_todos(&mut self, val: String) {
        let needle = String::from(val.trim());

        if needle == "" && self.entries_bkp.len() > 0 {
            self.entries = self.entries_bkp.clone();
            self.entries_bkp = Vec::new();
            return;
        } else if needle == "" && self.entries_bkp.len() == 0 {
            // Nothing to do here.
            return;
        }

        self.entries_bkp = self.entries.clone();

        let entries = self
            .entries
            .drain(..)
            .filter(move |e| Filter::Search(needle.clone()).fit(e))
            .collect();

        self.entries = entries;
    }
}
