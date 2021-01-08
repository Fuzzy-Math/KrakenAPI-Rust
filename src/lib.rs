pub mod auth;
mod api;
mod client;

pub use api::private;
pub use api::public;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
