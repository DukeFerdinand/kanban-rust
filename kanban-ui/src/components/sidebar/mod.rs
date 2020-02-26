use yew::prelude::*;

pub struct Sidebar {
  is_expanded: bool,
  expand_sidebar: Callback<ClickEvent>,
}

pub enum Msg {
  ExpandSidebar,
}

impl Component for Sidebar {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      is_expanded: false,
      expand_sidebar: link.callback(|_| Msg::ExpandSidebar),
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::ExpandSidebar => {
        self.is_expanded = !self.is_expanded;
        true
      }
    }
  }

  fn view(&self) -> Html {
    let expanded_text = if self.is_expanded {
      "Expanded"
    } else {
      "Not expanded"
    };
    html! {
      <div class="sidebar">
        {{"sidebar"}}
        <button class="toggle-sidebar" onclick=&self.expand_sidebar>{{ "toggle" }}</button>
        <p>{{ expanded_text }}</p>
      </div>
    }
  }
}
