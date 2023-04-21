use yew::{html, prelude::Component};

fn main() {
    println!("Hello, world! Starting the Yew App");

    yew::start_app::<Counter>();
}

struct Counter {
    count: i64,
}

impl Component for Counter {
    type Message = Message;
    type Properties = ();

    // Those are the life cycle methods

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddOne => {
                self.count += 1;
                true // re-render component},
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let link = ctx.link();

        html! {
        <div class="container">
            <p>{ self.count }</p>
            <button onclick={link.callback(|_|Message::AddOne)}>
            { "+1" }
            </button>
        </div>
        }
    }
}

enum Message {
    AddOne,
}
