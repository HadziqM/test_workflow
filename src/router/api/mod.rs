use rocket::{Route, routes};

mod monitor;
mod user;
mod input;

pub fn reg()->Vec<Route> {
    routes![input::general,monitor::flow,monitor::temp,monitor::status,monitor::message,monitor::vibration,user::login,user::ships]
}