use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    A(u32),
    B { b: &'static str },
    C,
}

fn main() {
    let _: T = T::from(1);
    let _: T = T::from("b");
}
