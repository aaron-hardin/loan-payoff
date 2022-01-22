mod loans;

use loans::Loans;
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <Loans />
                <MyComponentWithProps prop1="lorem" prop2="ipsum" />
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Props {
    prop1: String,
    prop2: String,
}

struct MyComponentWithProps;

impl Component for MyComponentWithProps {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <span>
                    { format!(
                        "prop1: {} and prop2: {}",
                        ctx.props().prop1,
                        ctx.props().prop2
                    ) }
                </span>
            </div>
        }
    }
}

fn main() {
    let document = gloo_utils::document();
    let mount_point = document.query_selector("div.mountpoint").unwrap().unwrap();
    yew::start_app_in_element::<Model>(mount_point);
}