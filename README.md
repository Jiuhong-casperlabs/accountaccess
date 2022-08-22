## main contract

Prepare a list of public key which is to be authorized when deploying the main contract

`let pks: Vec<PublicKey> = runtime::get_named_arg("pks");`

## authorize accounts

[authorize_accounts](./contract/src/authorize_account.rs)

This contract has to be called by the to be authorized account itself.

