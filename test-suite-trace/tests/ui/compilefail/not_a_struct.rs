use hcaptcha_no_wasm::Hcaptcha;
// use hcaptcha_derive::Hcaptcha;

#[derive(Hcaptcha)]
enum Test {
    #[captcha]
    Hcaptcha(String),
}

fn main() {
    println!("Super!");
}