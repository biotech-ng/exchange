use base64::engine::{general_purpose, GeneralPurpose};
use base64::{DecodeError, Engine};
use hmac::Hmac;
use hmac::Mac;
use rand::{thread_rng, RngCore};
use sha2::Sha512;

const SALT_LENGTH: usize = 64;
const BASE_64: GeneralPurpose = general_purpose::STANDARD;

fn generate_salt(salt_bytes: &mut [u8]) {
    let mut rng = thread_rng();
    rng.fill_bytes(salt_bytes);
}

fn hash_password(password: impl AsRef<str>, salt_bytes: &[u8]) -> String {
    let salted_password = [salt_bytes, password.as_ref().as_bytes()].concat();

    let mut mac = Hmac::<Sha512>::new_from_slice(salt_bytes).unwrap();
    mac.update(&salted_password);
    let result = mac.finalize();

    BASE_64.encode(result.into_bytes())
}

pub fn generate_hash_and_salt_for_text(password: impl AsRef<str>) -> (String, String) {
    let mut salt_bytes = [0u8; SALT_LENGTH];
    generate_salt(&mut salt_bytes);

    (
        hash_password(password, &salt_bytes),
        BASE_64.encode(salt_bytes),
    )
}

pub fn generate_b64_hash_for_text_and_salt(
    test: impl AsRef<str>,
    salt_b64: impl AsRef<str>,
) -> Result<String, DecodeError> {
    let salt_bytes = BASE_64.decode(salt_b64.as_ref())?;

    Ok(hash_password(test, salt_bytes.as_slice()))
}
