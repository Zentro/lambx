use crate::xeddsa;

use rand::{rngs::StdRng, SeedableRng, Rng};
use x25519_dalek::{StaticSecret, PublicKey};
use hex::{self, ToHex};

pub fn test() {
    let r = StdRng::from_entropy();
    let alice_secret = StaticSecret::random_from_rng(r);
    let alice_public = PublicKey::from(&alice_secret);
    let r2 = StdRng::from_entropy();
    let bob_secret = StaticSecret::random_from_rng(r2);
    let bob_public = PublicKey::from(&bob_secret);
    let x = xeddsa::XEdDSA::new(alice_secret);
    for i in 0..10 {
        let msg = rand::thread_rng().gen::<[u8; 16]>();
        let nonce = rand::thread_rng().gen::<[u8; 16]>();
        let sig = x.sign(&msg, &nonce);
        println!("Alice sent message {}", hex::encode(msg));
        println!("Alice's signature on the message {}", hex::encode(sig));
        if x.verify(alice_public, &msg, sig) {
            println!("Alice succesfully verified with her public key.");
        } else {
            println!("Alice failed to verify with her public key.");
        }
        if x.verify(bob_public, &msg, sig) {
            println!("Bob should not be able to do this.");
        } else {
            println!("Bob tried to verify Alice's signature using his public key and FAILED.");
        }
        println!();
    }
}