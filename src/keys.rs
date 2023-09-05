use jwt_simple::prelude::*;

fn main() {
    let key_pair = Ed25519KeyPair::generate();

    println!("Public key: {:?}", key_pair.public_key());
    println!("Pair: {:?}", key_pair.to_bytes());
}
