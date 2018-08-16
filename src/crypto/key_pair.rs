//! Generate an Ristretto keypair for Schnorr signatures.
pub use redschnorr::{Keypair, PublicKey, SecretKey, Signature};

use rand::OsRng;
use sha2::Sha512;
use Result;

/// Generate a new `Ed25519` key pair.
pub fn generate() -> Keypair {
  let mut cspring: OsRng = OsRng::new().unwrap();
  Keypair::generate::<Sha512, _>(&mut cspring)
}

/// Sign a byte slice using a keypair's private key.
pub fn sign(
  secret: &SecretKey,
  msg: &[u8],
) -> Signature {
  secret.sign::<Sha512>(msg)
}

/// Verify a signature on a message with a keypair's public key.
pub fn verify(
  public: &PublicKey,
  msg: &[u8],
  sig: Option<&Signature>,
) -> Result<()> {
  match sig {
    None => bail!("Signature verification failed"),
    Some(sig) => {
      let success: bool = match public.verify::<Sha512>(msg, sig) {
          Ok(_) => true,
          Err(_) => false,
      };
      ensure!(
        success,
        "Signature verification failed"
      );
      Ok(())
    }
  }
}

#[test]
fn can_verify_messages() {
  let keypair = generate();
  let from = b"hello";
  let sig = sign(&keypair.secret, from);
  verify(&keypair.public, from, Some(&sig)).unwrap();
  verify(&keypair.public, b"oops", Some(&sig)).is_err();
}
