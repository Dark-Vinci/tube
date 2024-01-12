use argon2::{
    password_hash::SaltString, 
    Argon2, 
    Algorithm, 
    Version, 
    Params, 
    PasswordHash, 
    PasswordVerifier
};

use crate::constants::types::E;


pub fn compute_password_hash(password: String) -> Result<String, E> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let mut us: Vec<u8> = vec![];

    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password_into(password.as_bytes(), &salt.as_str().as_bytes(), &mut us).unwrap();

    let a = String::from_utf8(us);

    if let Err(e) = a {
        return Err(Box::new(e));
    }

    Ok(a.unwrap())
}

pub fn compare_password(expected: &str, password: String) -> bool {
    let password_hash = PasswordHash::new(expected);

    if let Err(_e) = password_hash {
        return false;
    }

    let _ = Argon2::default()
        .verify_password(
            password.as_bytes(), 
            &password_hash.unwrap()
        ).map_err(|_| false);

    true
}
