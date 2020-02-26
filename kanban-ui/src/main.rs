use yew::prelude::*;

mod components;
use components::sidebar::Sidebar;

// Base component methods and states
struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

// Think of these as local redux actions
enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    // ComponentLink is used similarly to binding 'this' in React. You can pass that link around to modify this component
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    // Reducer style state update. Actions come in and you decide whether the component will act on them
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Not clicked!"
        };

        html! {
            <div class="app-container">
                <Sidebar />
                <button onclick=&self.onclick class="clicker">{{ button_text }}</button>
            </div>
        }
    }
}

fn main() {
    println!("Hello, world!");
    yew::start_app::<App>();
}
