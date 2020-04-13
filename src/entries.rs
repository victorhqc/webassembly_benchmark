use crate::app::{Entry, EntryStatus};
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.webassembly.self";

pub struct Entries {
  link: ComponentLink<Self>,
  pub props: Props,
  storage: StorageService,
  state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
  pub entries: Vec<Entry>,
  pub edit_value: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Props {
  pub entries: Vec<Entry>,
}

pub enum Msg {
  Edit(usize),
  UpdateEdit(String),
  Remove(usize),
  ToggleEdit(usize),
  Toggle(usize),
  Nope,
}

impl Component for Entries {
  type Properties = Props;
  type Message = Msg;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    let storage = StorageService::new(Area::Local).expect("Couldn't create Storage service");
    let state = State {
      entries: props.entries.clone(),
      edit_value: String::from(""),
    };

    Entries {
      props,
      link,
      state,
      storage,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ToggleEdit(idx) => {
        self.state.edit_value = self.state.entries[idx].description.clone();
        self.state.toggle_edit(idx);
      }
      Msg::Toggle(idx) => {
        self.state.toggle(idx);
      }
      Msg::Edit(idx) => {
        let edit_value = self.state.edit_value.clone();
        self.state.complete_edit(idx, edit_value);
        self.state.edit_value = String::from("");
      }
      Msg::UpdateEdit(val) => {
        println!("Edit Input: {}", val);
        self.state.edit_value = val;
      }
      Msg::Remove(idx) => {
        self.state.remove(idx);
      }
      Msg::Nope => {}
    }

    self.storage.store(KEY, Json(&self.state.entries));
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    if self.props != props {
      self.state = State {
        entries: props.entries.clone(),
        edit_value: String::from(""),
      };
      self.props = props;
      true
    } else {
      false
    }
  }

  fn view(&self) -> Html {
    html! {
      <ul class="todo-list">
          {
              for self.props.entries.iter()
              .enumerate()
              .map(|val| self.view_entry(val))
          }
      </ul>
    }
  }
}

impl Entries {
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

impl State {
  fn toggle(&mut self, idx: usize) {
    let mut entries = self.entries.iter_mut().collect::<Vec<_>>();
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
    let mut entries = self.entries.iter_mut().collect::<Vec<_>>();
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
    let mut entries = self.entries.iter_mut().collect::<Vec<_>>();
    let entry = entries.get_mut(idx).unwrap();
    entry.description = val;
    entry.status = EntryStatus::New;
  }

  fn remove(&mut self, idx: usize) {
    let idx = {
      let entries = self.entries.iter().enumerate().collect::<Vec<_>>();
      let &(idx, _) = entries.get(idx).unwrap();
      idx
    };

    self.entries.remove(idx);
  }
}
