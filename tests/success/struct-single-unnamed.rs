use derive_from_one::FromOne;

#[derive(FromOne)]
pub struct T(u32);

fn main() {
    let _: T = T::from(1);
}
