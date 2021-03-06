#![deny(missing_docs, unsafe_code, unstable_features)]
//! # Crypto.com Chain Client
//!
//! This crate provides `WalletClient` trait which is responsible for interacting with transaction index and provide
//! following functionalities on per-wallet basis:
//!
//! - Wallet creation
//! - Address generation
//! - Balance tracking
//! - Transaction history
//! - Transaction creation and signing (with automatic unspent transaction selection)

pub mod hd_seed;
pub mod hd_wallet;
pub mod input_selection;
pub mod mnemonic;
#[cfg(feature = "experimental")]
pub mod multi_sig;
pub mod service;
pub mod signer;

pub mod transaction_builder;
pub mod types;
pub mod unspent_transactions;
pub mod wallet;

#[doc(inline)]
pub use crate::hd_seed::HDSeed;
#[doc(inline)]
pub use crate::input_selection::InputSelectionStrategy;
#[doc(inline)]
pub use crate::mnemonic::Mnemonic;
#[doc(inline)]
pub use crate::service::WalletStateMemento;
#[doc(inline)]
pub use crate::signer::{SignCondition, Signer};
#[doc(inline)]
pub use crate::transaction_builder::WalletTransactionBuilder;
#[doc(inline)]
pub use crate::unspent_transactions::{SelectedUnspentTransactions, UnspentTransactions};
#[doc(inline)]
pub use crate::wallet::WalletClient;

#[cfg(feature = "experimental")]
#[doc(inline)]
pub use crate::wallet::MultiSigWalletClient;
