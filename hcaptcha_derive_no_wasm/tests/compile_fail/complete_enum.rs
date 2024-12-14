use hcaptcha_derive_no_wasm::Hcaptcha;

#[derive(Hcaptcha)]
pub enum ContactEnum {
    Name,
    #[captcha]
    Token,
}

fn main() {
    println!("hello");
}
