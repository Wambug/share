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
      text-align: center;
      background-clip: text;
      -webkit-background-clip: text;
    }
    h2 {
    font-size:50px;
    font-weight:500;
    color: white;
    text-align:center;
    border-right: 4px solid #000;
    animation: cursor 1s infinite step-end, typing 15s infinite steps(16);
    white-space:nowrap;
    overflow:hidden; 
    }
   @keyframes cursor {
    0%, 100%{border-color:transparent;}
    50%{border-color:#000;}
     }
     @keyframes typing{
     0%{width: 0ch}
     50%{width:28ch;}
     80%{width:28ch;}
     90%{width:0ch;}
     100%{width:0ch;}
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
        <h1>{"Share."}</h1>
        </header>
        <h2>{"Share is a file sharing web app."}</h2>

        </div>
    </>
        }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
        <Header />
        <h2> {"Initial Setup trying Wasm in rust!"}</h2>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
