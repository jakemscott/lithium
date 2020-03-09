extern crate rand;
extern crate argon2rs;

use argon2rs::defaults::{ KIB, LANES, PASSES };
use argon2rs::verifier::Encoded;
use argon2rs::{ Argon2, Variant };

use std::env;
use std::default::Default;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct Password {
    pub local_salt: String,
    pub random_salt: String,
}
impl Default for Password {
    // Then able to pass salt directly to creation through: let pass = Password { random_salt: retrieved .. Password::default() }
    fn default() -> Self {
        let ls = env::var("LOCAL_SALT").expect("Local Salt variable not set");
        let rs = rand::thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>();

        Password { local_salt: ls, random_salt: rs }
    }
}
impl Password {
    fn a2() -> Argon2 {
        Argon2::new(PASSES, LANES, KIB, Variant::Argon2d).unwrap()
    }
    fn hashed_salt(&self) -> String {
        let rs_hash = Encoded::new(Password::a2(), self.random_salt.as_bytes(), self.local_salt.as_bytes(), b"", b"").to_u8();
        String::from_utf8(rs_hash).unwrap()
    }
    pub(crate) fn hash(&self, input: &str) -> String {
        // be sure to store this impl random salt and returned hash
        let hashed = Encoded::new(Password::a2(), input.as_bytes(), self.hashed_salt().as_bytes(), b"", b"").to_u8();
        String::from_utf8(hashed).unwrap()
    }
    pub(crate) fn verify(&self, input: &str, hash: &str) -> bool {
        let mut resp = false;
        let hashed_input: String = self.hash(input);

        if hashed_input.as_str() == hash { resp = true; }
        resp
    }
}