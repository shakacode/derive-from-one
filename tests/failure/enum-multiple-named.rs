use derive_from_one::FromOne;

#[derive(FromOne)]
pub enum T {
    A { foo: bool, bar: u32 },
    B { b: &'static str },
}

fn main() {
    let _: T = T::from("b");
    let _: T = T::from(true);
}
