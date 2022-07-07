use web_sys::HtmlInputElement;
use yew::{events::Event, html, Component, Context, Html, Properties, TargetCast};
enum Msg {
    AddOne,
    SubtractOne,
    ChangeValue(Option<String>),
}

struct CounterComponent {
    value: i64,
}

#[derive(PartialEq, Properties)]
struct CounterProps {
    initial_value: i64,
}

impl Default for CounterProps {
    fn default() -> Self {
        Self { initial_value: 10 }
    }
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = CounterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: _ctx.props().initial_value,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubtractOne => {
                self.value -= 1;
                true
            }
            Msg::ChangeValue(str) => {
                if str.is_none() {
                    return true;
                }

                let str = str.unwrap();

                if let Ok(num) = str.parse::<i64>() {
                    self.value = num;
                    return true;
                }

                return false;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let onchange = link.callback(|e: Event| {
            let input: Option<HtmlInputElement> = e.target_dyn_into();
            match input {
                Some(html_input) => Msg::ChangeValue(Some(html_input.value())),
                None => Msg::ChangeValue(None),
            }
        });
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::SubtractOne)}>{ "-1" }</button>
                <input type="number" {onchange} value={self.value.to_string()} />
                <p>{self.value.to_string()}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
