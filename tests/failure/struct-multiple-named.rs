use derive_from_one::FromOne;

#[derive(FromOne)]
pub struct T {
    a: u32,
    b: bool,
}

fn main() {
    let _: T = T::from(1);
}
