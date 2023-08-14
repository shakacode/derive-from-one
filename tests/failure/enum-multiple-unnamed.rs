use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    A(bool, u32),
    B(&'static str),
}

fn main() {
    let _: T = T::from("b");
    let _: T = T::from(true);
}
