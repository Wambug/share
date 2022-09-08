use std::rc::Rc;
use stylist::style;
use yew::prelude::*;

#[derive(Debug)]
struct Upload {
    filename: Rc<Option<String>>,
    filetype: Rc<Option<String>>,
}

pub enum Msg {}

impl Component for Upload {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Upload {
            filename: Rc::new(None),
            filetype: Rc::new(None),
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = style!(
            "
   
"
        )
        .expect("Failed to mount style")
        .get_class_name()
        .to_string();
        html! {
        <div class={style}>
        <label>{"Select file"}</label>
        <input type="file"/>
        </div>
        }
    }
}

#[function_component(Header)]
fn header() -> Html {
    let style = style!(
        r#" 
    header h1 {
      font-size: 70px;
      font-weight: 600;
      background-image: linear-gradient(to left,#3E3E3D,#0D0C0D);
      color: transparent;
      text-align: center;
      background-clip: text;
      -webkit-background-clip: text;
    }
    h2 {
    font-size:50px;
    font-weight:500;
    color: white;
    border-right: 4px solid #000;
    animation: cursor 1s infinite step-end, typing 10s infinite steps(16);
    white-space:nowrap;
    overflow:hidden; 
    }
   @keyframes cursor {
    0%, 100%{border-color:transparent;}
    50%{border-color:#000;}
     }
     @keyframes typing{
     0%{width: 0ch}
     50%{width:26ch;}
     80%{width:26ch;}
     90%{width:0ch;}
     100%{width:0ch;}
     }
   .flex-container {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 100vw;
      height: 20vh;
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
        <h1>{"Share 🔗"}</h1>
        </header>
       <div class="flex-container">
        <h2>{"Share is a file sharing web app."}</h2>
        </div>
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
       <Upload />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
