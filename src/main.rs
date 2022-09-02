use stylist::style;
use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    let style = style!(
        r#" 
    header h1 {
      font-size: 70px;
      font-weight: 600;
      background-image: linear-gradient(to left,#553c9a,#b393d3);
      color: transparent;
      background-clip: text;
      -webkit-background-clip: text;
    }
    "#
    )
    .expect("Failed to mount style")
    .get_class_name()
    .to_string();
    html! {
    <>
        <div class={style} >
        <header>
        <h1>{"Modern Frontend Monitoring and Product Analytics."}</h1>
        </header>
        </div>
    </>
        }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Header />
        <h2> {"Initial Setup trying Wasm in rust!"}</h2>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
