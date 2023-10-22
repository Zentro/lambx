use sha2::{Sha512, Digest};
use bnum::types::U256;
use curve25519_dalek::{edwards::{EdwardsPoint, CompressedEdwardsY}, Scalar, MontgomeryPoint, scalar::clamp_integer};
use x25519_dalek::{PublicKey, StaticSecret};

struct XEdDSA {
    prime: U256,
    sk_scalar: Scalar,
}

#[allow(non_snake_case)]
#[allow(deprecated)]
impl XEdDSA {
    pub fn new(sk: StaticSecret) -> Self {
        let prime = (U256::ONE << 255) - U256::from_str_radix("19", 10).unwrap();
        Self {
            prime,
            sk_scalar: Scalar::from_bits(clamp_integer(sk.to_bytes())),
        }
    }

    fn sign(&self, msg: &[u8], nonce: &[u8]) -> [u8; 64] {
        let (ed_pk, ed_sk) = self.calculate_key_pair(self.sk_scalar);
        
        let concat1 = [&ed_sk, msg, nonce].concat();
        let r0 = self.hash_i(1, concat1.as_slice());
        let r = Scalar::from_bytes_mod_order_wide(&r0);
        
        let R = EdwardsPoint::mul_base(&r).compress().to_bytes();

        let concat2 = [&R, &ed_pk, msg].concat();
        let hash: [u8; 64] = Sha512::new()
            .chain_update(concat2.as_slice())
            .finalize()
            .as_slice()
            .try_into()
            .unwrap();
        let h: Scalar = Scalar::from_bytes_mod_order_wide(&hash);
        let a: Scalar = Scalar::from_bytes_mod_order(ed_sk);
        let s = (r +(h*a)).to_bytes();
        let to_ret: [u8; 64] = [R, s].concat().try_into().unwrap();
        to_ret
    }

    fn verify(&self, pk: PublicKey, msg: &[u8], sig: [u8; 64]) -> bool {
        let mont_pk = pk.to_bytes();
        let pk_int: U256 = U256::from_le_slice(&mont_pk).unwrap();
        // Separate R from sig
        let mut R: [u8; 32] = [0; 32];
        R.clone_from_slice(&sig[0..32]);
        let mut R0 = R.clone();
        R0[31] &= 0b0111_1111;
        let R_int = U256::from_le_slice(&R0).unwrap();

        let mut s: [u8; 32] = [0; 32];
        s.clone_from_slice(&sig[32..]);
        let s_int = U256::from_le_slice(&s).unwrap();

        if pk_int >= self.prime || 
           R_int >= (U256::ONE << 255) || 
           s_int >= (U256::ONE << 253) {
            return false;
        }
        let A = match (MontgomeryPoint{0:mont_pk}).to_edwards(0) {
            Some(res) => res,
            None => return false, // Not on the curve
        };

        let hash: [u8; 64] = Sha512::new()
            .chain_update(R)
            .chain_update(A.compress().as_bytes())
            .chain_update(msg)
            .finalize()
            .as_slice()
            .try_into()
            .unwrap();
        let h: Scalar = Scalar::from_bytes_mod_order_wide(&hash);
        let s_scalar = Scalar::from_bytes_mod_order(s);
        let R_check = (EdwardsPoint::mul_base(&s_scalar) - (h*A)).compress().to_bytes();
        if R.eq(&R_check) {
            true
        } else {
            false
        }
    }

    fn hash_i(&self, i: u128, msg: &[u8]) -> [u8; 64]{
        let prefix = U256::MAX.sub(i.into());
        let hash = Sha512::new()
            .chain_update(prefix.to_radix_le(256))
            .chain_update(msg)
            .finalize();
        let res: [u8; 64] = hash.as_slice().try_into().unwrap();
        res
    }

    fn calculate_key_pair(&self, scal: Scalar) -> ([u8; 32], [u8; 32]) {
        let ed: CompressedEdwardsY = (EdwardsPoint::mul_base(&scal)).compress();

        let mut pkb: [u8; 32] = ed.to_bytes();
        // Force public key sign bit to 0
        let pkb_m = pkb.as_mut();
        pkb_m[31] &= 0b0111_1111;
        
        // Derive the secret key
        let skb: [u8; 32];
        let ed_sign = (ed.to_bytes()[31] & 0b1000_0000) >> 7;
        if ed_sign == 1 {
            skb = (-scal).to_bytes();
        } else {
            skb = scal.to_bytes();
        }
        (pkb, skb)
    }
}