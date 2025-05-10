use russh::keys::PrivateKey;
use russh::keys::signature::rand_core::OsRng;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path for saving the key pair
    let private_key_path = PathBuf::from("id_ed25519");
    let public_key_path = PathBuf::from("id_ed25519.pub");

    // Generate a new Ed25519 keypair
    let mut rng = OsRng;
    let line_ending = russh::keys::ssh_key::LineEnding::LF;
    let secret_key: PrivateKey =
        PrivateKey::random(&mut rng, russh::keys::Algorithm::Ed25519).unwrap();
    let _ = secret_key.write_openssh_file(&private_key_path, line_ending);
    let _ = secret_key.public_key().write_openssh_file(&public_key_path);

    println!("SSH key pair generated successfully:");
    println!("Private Key: {:?}", private_key_path);
    println!("Public Key: {:?}", public_key_path);

    Ok(())
}
