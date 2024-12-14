use hcaptcha_derive_no_wasm::Hcaptcha;

#[derive(Debug, Hcaptcha)]
pub struct ContactForm {
    name: String,
    #[allow(dead_code)]
    phone: String,
    email: String,
    #[allow(dead_code)]
    message: String,
    #[captcha]
    token: String,
}
