use sha2::{Sha512, Digest};
use bnum::types::U256;
use curve25519_dalek::{edwards::{EdwardsPoint, CompressedEdwardsY}, Scalar, MontgomeryPoint, scalar::clamp_integer};

pub struct XEdDSA {
    prime: U256,
}

#[allow(non_snake_case)]
#[allow(deprecated)]
impl XEdDSA {
    /**
     * Instance of an XEdDSA, will likely change to be non object
     */
    pub fn new() -> Self {
        Self {
            prime: (U256::ONE << 255) - U256::from_str_radix("19", 10).unwrap(),
        }
    }

    /**
     * Signs a message along with a secret key. Nonce NEEDS to be randomly generated
     * preferably by a cryptographically secure PRG.
     * 
     * Returns a 64 byte signature
     */
    pub fn sign(&self, sk: [u8; 32], msg: &[u8], nonce: &[u8]) -> [u8; 64] {
        // Scalar value derived from X25519's way of clamping byte array and
        // directly using it without (mod q)
        let sk_scalar = Scalar::from_bits(clamp_integer(sk));
        let (ed_pk, ed_sk) = self.calculate_key_pair(sk_scalar);
        
        let concat1 = [&ed_sk, msg, nonce].concat();
        let r0 = self.hash_i(1, concat1.as_slice());
        let r = Scalar::from_bytes_mod_order_wide(&r0);
        
        let R = EdwardsPoint::mul_base(&r).compress().to_bytes();

        let hash: [u8; 64] = Sha512::new()
            .chain_update(R)
            .chain_update(ed_pk)
            .chain_update(msg)
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

    /**
     * Verifies a message to its signature using a public key generated
     * from the secret key derive from the X25519-Dalek functions. 
     */
    pub fn verify(&self, pk: [u8; 32], msg: &[u8], sig: [u8; 64]) -> bool {
        let pk_int: U256 = U256::from_le_slice(&pk).unwrap();
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
        let A = match (MontgomeryPoint{0:pk}).to_edwards(0) {
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

    /**
     * Acts almost like a keyed hash function (except not)
     */
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