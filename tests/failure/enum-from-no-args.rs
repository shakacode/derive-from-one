use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    #[from]
    A(bool),
    B(&'static str),
}

fn main() {}
