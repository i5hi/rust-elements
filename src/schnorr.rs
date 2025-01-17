// Rust Bitcoin Library
// Written in 2014 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Schnorr Bitcoin keys.
//!
//! This module provides Schnorr keys used in Bitcoin, reexporting Secp256k1
//! Schnorr key types.
//!

use std::fmt;

use crate::taproot::{TapBranchHash, TapTweakHash};
use crate::SchnorrSighashType;
use secp256k1_zkp::{self, constants::SCHNORR_SIGNATURE_SIZE, Secp256k1, Verification};
pub use secp256k1_zkp::{KeyPair, XOnlyPublicKey};

/// Untweaked Schnorr public key
pub type UntweakedPublicKey = XOnlyPublicKey;

/// Tweaked Schnorr public key
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TweakedPublicKey(XOnlyPublicKey);

/// Untweaked Schnorr key pair
pub type UntweakedKeyPair = KeyPair;

/// Tweaked Schnorr key pair
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TweakedKeyPair(KeyPair);

/// A trait for tweaking Schnorr key types (x-only public keys and key pairs).
pub trait TapTweak {
    /// Tweaked key type with optional auxiliary information
    type TweakedAux;
    /// Tweaked key type
    type TweakedKey;

    /// Tweaks an untweaked key with corresponding public key value and optional script tree merkle
    /// root. For the [`KeyPair`] type this also tweaks the private key in the pair.
    ///
    /// This is done by using the equation Q = P + H(P|c)G, where
    ///  * Q is the tweaked public key
    ///  * P is the internal public key
    ///  * H is the hash function
    ///  * c is the commitment data
    ///  * G is the generator point
    ///
    /// # Returns
    /// The tweaked key and its parity.
    fn tap_tweak<C: Verification>(
        self,
        secp: &Secp256k1<C>,
        merkle_root: Option<TapBranchHash>,
    ) -> Self::TweakedAux;

    /// Directly converts to a `TweakedKey`
    ///
    /// This method is dangerous and can lead to loss of funds if used incorrectly.
    /// Specifically, in multi-party protocols a peer can provide a value that allows them to steal.
    fn dangerous_assume_tweaked(self) -> Self::TweakedKey;
}

impl TapTweak for UntweakedPublicKey {
    type TweakedAux = (TweakedPublicKey, secp256k1_zkp::Parity);
    type TweakedKey = TweakedPublicKey;

    /// Tweaks an untweaked public key with corresponding public key value and optional script tree
    /// merkle root.
    ///
    /// This is done by using the equation Q = P + H(P|c)G, where
    ///  * Q is the tweaked public key
    ///  * P is the internal public key
    ///  * H is the hash function
    ///  * c is the commitment data
    ///  * G is the generator point
    ///
    /// # Returns
    /// The tweaked key and its parity.
    fn tap_tweak<C: Verification>(
        self,
        secp: &Secp256k1<C>,
        merkle_root: Option<TapBranchHash>,
    ) -> (TweakedPublicKey, secp256k1_zkp::Parity) {
        let tweak = TapTweakHash::from_key_and_tweak(self, merkle_root).to_scalar();
        let (output_key, parity) = self.add_tweak(secp, &tweak).expect("Tap tweak failed");

        debug_assert!(self.tweak_add_check(secp, &output_key, parity, tweak));
        (TweakedPublicKey(output_key), parity)
    }


    fn dangerous_assume_tweaked(self) -> TweakedPublicKey {
        TweakedPublicKey(self)
    }
}

impl TapTweak for UntweakedKeyPair {
    type TweakedAux = TweakedKeyPair;
    type TweakedKey = TweakedKeyPair;

    /// Tweaks private and public keys within an untweaked [`KeyPair`] with corresponding public key
    /// value and optional script tree merkle root.
    ///
    /// This is done by tweaking private key within the pair using the equation q = p + H(P|c), where
    ///  * q is the tweaked private key
    ///  * p is the internal private key
    ///  * H is the hash function
    ///  * c is the commitment data
    /// The public key is generated from a private key by multiplying with generator point, Q = qG.
    ///
    /// # Returns
    /// The tweaked key and its parity.
    fn tap_tweak<C: Verification>(self, secp: &Secp256k1<C>, merkle_root: Option<TapBranchHash>) -> TweakedKeyPair {
        let (pubkey, _parity) = XOnlyPublicKey::from_keypair(&self);
        let tweak = TapTweakHash::from_key_and_tweak(pubkey, merkle_root).to_scalar();
        let tweaked = self.add_xonly_tweak(secp, &tweak).expect("Tap tweak failed");
        TweakedKeyPair(tweaked)
    }

    fn dangerous_assume_tweaked(self) -> TweakedKeyPair {
        TweakedKeyPair(self)
    }
}

impl TweakedPublicKey {
    /// Returns the [`TweakedPublicKey`] for `keypair`.
    #[inline]
    pub fn from_keypair(keypair: TweakedKeyPair) -> Self {
        let (xonly, _parity) = keypair.0.x_only_public_key();
        TweakedPublicKey(xonly)
    }

    /// Create a new [`TweakedPublicKey`] from a [`XOnlyPublicKey`]. No tweak is applied.
    pub fn new(key: XOnlyPublicKey) -> TweakedPublicKey {
        TweakedPublicKey(key)
    }

    /// Returns the underlying public key
    pub fn into_inner(self) -> XOnlyPublicKey {
        self.0
    }

    /// Returns a reference to underlying public key
    pub fn as_inner(&self) -> &XOnlyPublicKey {
        &self.0
    }
}

impl TweakedKeyPair {
    /// Creates a new [`TweakedKeyPair`] from a [`KeyPair`]. No tweak is applied, consider
    /// calling `tap_tweak` on an [`UntweakedKeyPair`] instead of using this constructor.
    ///
    /// This method is dangerous and can lead to loss of funds if used incorrectly.
    /// Specifically, in multi-party protocols a peer can provide a value that allows them to steal.
    #[inline]
    pub fn dangerous_assume_tweaked(pair: KeyPair) -> TweakedKeyPair {
        TweakedKeyPair(pair)
    }

    /// Returns the underlying key pair.
    #[inline]
    pub fn to_inner(self) -> KeyPair {
        self.0
    }

    /// Returns the [`TweakedPublicKey`] and its [`secp256k1_zkp::Parity`] for this [`TweakedKeyPair`].
    #[inline]
    pub fn public_parts(&self) -> (TweakedPublicKey, secp256k1_zkp::Parity) {
        let (xonly, parity) = self.0.x_only_public_key();
        (TweakedPublicKey(xonly), parity)
    }
}

impl From<TweakedPublicKey> for XOnlyPublicKey {
    #[inline]
    fn from(pair: TweakedPublicKey) -> Self {
        pair.0
    }
}

impl From<TweakedKeyPair> for KeyPair {
    #[inline]
    fn from(pair: TweakedKeyPair) -> Self {
        pair.0
    }
}

impl From<TweakedKeyPair> for TweakedPublicKey {
    #[inline]
    fn from(pair: TweakedKeyPair) -> Self {
        TweakedPublicKey::from_keypair(pair)
    }
}

/// A BIP340-341 serialized schnorr signature with the corresponding hash type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(crate = "actual_serde"))]
pub struct SchnorrSig {
    /// The underlying schnorr signature
    pub sig: secp256k1_zkp::schnorr::Signature,
    /// The corresponding hash type
    pub hash_ty: SchnorrSighashType,
}

impl SchnorrSig {

    /// Deserialize from slice
    pub fn from_slice(sl: &[u8]) -> Result<Self, SchnorrSigError> {
        if sl.len() == SCHNORR_SIGNATURE_SIZE {
            // default type
            let sig = secp256k1_zkp::schnorr::Signature::from_slice(sl)
                .map_err(|_| SchnorrSigError::InvalidSchnorrSig)?;
            return Ok( SchnorrSig { sig, hash_ty : SchnorrSighashType::Default });
        }
        let (hash_ty, sig) = sl.split_last()
            .ok_or(SchnorrSigError::InvalidSchnorrSig)?;
        let hash_ty = SchnorrSighashType::from_u8(*hash_ty)
            .ok_or(SchnorrSigError::InvalidSighashType(*hash_ty))?;
        let sig = secp256k1_zkp::schnorr::Signature::from_slice(sig)
            .map_err(|_| SchnorrSigError::InvalidSchnorrSig)?;
        Ok(SchnorrSig { sig, hash_ty })
    }

    /// Serialize SchnorrSig
    pub fn to_vec(&self) -> Vec<u8> {
        // TODO: add support to serialize to a writer to SerializedSig
        let mut ser_sig = self.sig.as_ref().to_vec();
        if let SchnorrSighashType::Default = self.hash_ty {
            // default sighash type, don't add extra sighash byte
        } else {
            ser_sig.push(self.hash_ty as u8);
        }
        ser_sig
    }

}

/// A schnorr sig related error.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SchnorrSigError {
    /// Base58 encoding error
    InvalidSighashType(u8),
    /// secp256k1-related error
    InvalidSchnorrSig,
}


impl fmt::Display for SchnorrSigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SchnorrSigError::InvalidSighashType(hash_ty) =>
                write!(f, "Invalid signature hash type {}", hash_ty),
            SchnorrSigError::InvalidSchnorrSig => write!(f, "Cannot parse Schnorr Signature"),
        }
    }
}

impl ::std::error::Error for SchnorrSigError {}
