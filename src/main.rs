#[macro_use] extern crate rocket;

use std::sync::{Arc, Mutex};
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::futures::TryFutureExt;
use rocket::http::hyper::body::Buf;
use rocket::log::private::logger;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use rocket::tokio::sync::RwLock;


#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {

    pub player: bool,
    pub num: i32

}

fn sAdd(res:i32, num: u32) -> i32 {

   if (res & 2_i32.pow(num)) != 2_i32.pow(num) {
       return  res + 2_i32.pow(num);
    }else {
       return  res;
    }

}

#[derive(Debug, Clone,Copy, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MainState {
    pub BX: i32,
    pub BO: i32,
    pub prime: bool,
}

static mut x: i32 = 0;
static mut o: i32 = 0;
static mut p: bool = true;

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the post handler.
#[get("/events")]
async fn events(queue: &State<Sender<MainState>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}



/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(mut form: Form<Message>, queue: &State<Sender<MainState>>) {
    // A send 'fails' if there are no active subscribers. That's okay.


    unsafe {

        if x<0|| o <0 {
            x = 0;
            o = 0;
            p = true;
        }

        if form.num<0 {
            x = 0;
            o = 0;
            p = true;
        }else {
            if form.player == p {
                if form.player {
                    x = sAdd(x,form.num as u32);
                }else {
                    o = sAdd(o, form.num as u32);
                }
                p = !p;
            }
        }

        for win in vec![7, 56, 448, 73, 146, 292, 273, 84] {
            if (x & win ) == win {
                o = -1;
                x = win;
                p = true;
                println!("X:{} win:{}",x,win);
                break;
            }
            else if (o & win) == win{
                x = -1;
                o = win;
                p = true;
                println!("O: {} win: {}",o,win);
                break;
            }
        }

    let _res = queue.send(MainState{BX:x,BO: o,prime:p});

    } // let _res = queue.send(form.into_inner());
}


#[launch]
fn rocket() -> _ {


    rocket::build()
        .manage(channel::<MainState>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}