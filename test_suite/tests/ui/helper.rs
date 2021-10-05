use std::iter;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use hcaptcha::HcaptchaCaptcha;


pub fn random_response() -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(100)
        .collect()
}

pub fn dummy_captcha() -> HcaptchaCaptcha {
    HcaptchaCaptcha::new(&random_response())
        .unwrap()
        .set_remoteip(&fakeit::internet::ipv4_address())
        .unwrap()
        .set_sitekey(&fakeit::unique::uuid_v4())
        .unwrap()
}

pub fn random_string(characters: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(characters)
        .collect()
}