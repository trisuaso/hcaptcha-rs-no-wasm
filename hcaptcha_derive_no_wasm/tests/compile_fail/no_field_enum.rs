use hcaptcha_derive_no_wasm::Hcaptcha;

#[derive(Hcaptcha)]
pub enum ContactEnum {
    Name,
    Token,
}

fn main() {
    println!("hello");
}
