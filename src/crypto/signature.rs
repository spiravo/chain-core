
use ed25519_dalek::{
    SigningKey, VerifyingKey, Signature, Signer, Verifier
};

/// Generate a new SigningKey (for demo/testing).
pub fn generate_signing_key() -> SigningKey {
    use rand::rngs::OsRng;
    let mut csprng = OsRng{};
    SigningKey::generate(&mut csprng)
}

/// Sign a message with the provided SigningKey.
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Signature {
    signing_key.sign(message)
}

/// Verify a signature against a message and a public key.
pub fn verify_signature(public_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}
