// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::serde::conv::{Conv, Hex, Pem};
use ring::signature::VerificationAlgorithm;
use serde::{Deserialize, Serialize};
use untrusted::Input;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "keytype")]
pub enum Key {
    Ecdsa {
        scheme: EcdsaScheme,
        keyval: EcdsaKey,
    },
    Ed25519 {
        scheme: Ed25519Scheme,
        keyval: Ed25519Key,
    },
    Rsa {
        scheme: RsaScheme,
        keyval: RsaKey,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EcdsaScheme {
    EcdsaSha2Nistp256,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EcdsaKey {
    public: Conv<Pem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Ed25519Scheme {
    Ed25519,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Ed25519Key {
    public: Conv<Hex>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RsaScheme {
    RsassaPssSha256,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RsaKey {
    public: Conv<Pem>,
}

impl Key {
    /// Verify a signature of an object made with this key.
    #[allow(unused)]
    pub(crate) fn verify(&self, msg: &[u8], signature: &[u8]) -> bool {
        let (alg, public_key): (&dyn VerificationAlgorithm, Input) = match self {
            Key::Ecdsa {
                scheme: EcdsaScheme::EcdsaSha2Nistp256,
                keyval,
            } => (
                &ring::signature::ECDSA_P256_SHA256_ASN1,
                Input::from(keyval.public.as_slice()),
            ),
            Key::Ed25519 {
                scheme: Ed25519Scheme::Ed25519,
                keyval,
            } => (
                &ring::signature::ED25519,
                Input::from(keyval.public.as_slice()),
            ),
            Key::Rsa {
                scheme: RsaScheme::RsassaPssSha256,
                keyval,
            } => (
                &ring::signature::RSA_PSS_2048_8192_SHA256,
                Input::from(keyval.public.as_slice()),
            ),
        };

        ring::signature::verify(alg, public_key, Input::from(msg), Input::from(signature)).is_ok()
    }
}
