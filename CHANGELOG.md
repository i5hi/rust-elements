
# 0.23.0 - 2023-06-18

* https://github.com/ElementsProject/rust-elements/pull/167 Implement Ord for Transaction
* https://github.com/ElementsProject/rust-elements/pull/168 add Height::ZERO associated constant
* https://github.com/ElementsProject/rust-elements/pull/169 rename all Sighash types downcasing the middle "h", for example: SigHash -> Sighash

# 0.22.0 - 2023-06-08

* [https://github.com/ElementsProject/rust-elements/pull/159](Update `TapTweak`, and `schnorr` module generally, to match rust-bitcoin)
* [https://github.com/ElementsProject/rust-elements/pull/160](Make `Prevouts` generic over type of `TxOut`)
* [https://github.com/ElementsProject/rust-elements/pull/161](Add `Transaction::vsize` method)
* [https://github.com/ElementsProject/rust-elements/pull/157](dynafed: extract `FullParams` from `Params`)
* [https://github.com/ElementsProject/rust-elements/pull/166](**Update bitcoin dependency to 0.30.0 and secp256k1-zkp dependency to 0.9.1**)

# 0.21.1 - 2022-10-21

- Add `PeginData::parse_tx`
- Add `PeginData::parse_merkle_proof`

# 0.21.0 - 2022-10-19

- Copy `Sequence` and `LockTime` structures from rust-bitcoin 0.29.1
- Add `Txin::pegin_prevout` method which returns a `bitcoin::Outpoint`; modify `PeginData::from_pegin_witness` to take a bitcoin outpoint

# 0.20.0 - 2022-06-10

- Remove has_issuance field in TxIn, calculate it directly to avoid the user provide it.
- Blinding API cleanup into smaller chunks
- Issuance: add support for blinding, and surjection proof verification
- Pset: allow inserting inputs/outputs at specified position, fix Tweak serde and key bug
- Add liquid test parameters
- the feature "serde-feature" is now renamed to just "serde"
- update MSRV to 1.41.1
- breaking change in serde in how the Nonce is serialized
- `Block`, `BlockHeader`, `PeginData`, `PegoutData` loose the Default impl
- update rust-bitcoin to 0.29.1
- update secp256k1-zkp to 0.7.0
- update bitcoin_hases to 0.11.0

# 0.19.2 - 2022-06-16

- revert dynafed field `fedpeg_program` back to `bitcoin::Script`

# 0.19.1 - 2022-06-10

- revert use of `io::BufRead` back to `io::Read` in `ConsensusEncodable` trait
- deprecate `Block::get_size` in favor of new `Block::size`
- deprecate `Block::get_weight` in favor of new `Block::weight`
- deprecate `Transaction::get_size` in favor of new `Transaction::size`
- deprecate `Transaction::get_weight` in favor of new `Transaction::weight`
- implement `Default` on `PartiallySignedTransaction`, `TxIn`

# 0.19 - 2022-04-30 "The Taproot Release"

- Taproot support for complex taptrees compatible with elements taproot signature.
- Taproot psbt support with BIP 371
hash. Refer to spec [here](https://github.com/ElementsProject/elements/blob/master/doc/taproot-sighash.mediawiki)
- Support for new tapscript transaction introspection opcodes as per the [spec](https://github.com/ElementsProject/elements/blob/master/doc/tapscript_opcodes.md).
- Works with bitcoin 0.28 key types.
