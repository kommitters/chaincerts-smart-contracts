# Chaincerts Governance

![Version Badge](https://img.shields.io/github/v/release/kommitters/chaincerts_governance?style=for-the-badge)
[![License badge](https://img.shields.io/github/license/kommitters/chaincerts_governance?style=for-the-badge)](https://github.com/kommitters/chaincerts_governance/blob/main/LICENSE)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts_governance?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts_governance)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts_governance?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts_governance)

**Chaincerts Governance** is a set of smart contracts designed for the creation, distribution, and revocation of digital certificates on the blockchain. The goal of this project is to provide a decentralized and secure way to issue and manage certificates using Soroban smart contracts for [**Chaincerts**](https://chaincerts.co).

This repository contains two smart contracts:

- [**Cert Governance Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_governance):
  The `cert_governance` contract is responsible for defining the governance rules for Chaincerts. With this contract, users can set the revocability and expiration of certificates, define which users can receive a certificate, and restrict the distribution of certificates. Additionally, the `cert_governance` contract is responsible for executing the distribution and revocation actions for certificates to different users.

- [**Cert Wallet Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_wallet):
  The `cert_wallet` contract stores the Chaincerts of a specific user, and each user must have their own `cert_wallet`. This contract allows users to view their Chaincerts, add organizations authorized to issue certificates to them, and execute distribution and revocation actions on their certificates. Only functions that can be executed by the `cert_governance` contract can modify certificates in a `cert_wallet`.

## System overview
The following image shows the communication of each contract with the Stellar network and the interaction between them.
![Flow of the system](how_it_works.png)

## Pre-requirements
To be able to work with the contracts, you first need some programs or dependencies, for this you can read the following guide: [Soroban setup](https://soroban.stellar.org/docs/getting-started/setup).

## Setup
``` 
# Clone the repository 
git clone git@github.com:kommitters/chaincerts_governance.git

# Build the project and get dependencies 
cd chaincerts_governance
cargo build
 ```

## Testing
To test the contract run `cargo test -- --show-output` or ` cargo test -- --nocapture`

## Contracts workflow
1. The first thing we need to start is to have the public and private keys of Stellar laboratory. We can generate them by accessing the following URL on [Futurenet](https://laboratory.stellar.org/#account-creator?network=futurenet) and using the Friendbot tool.
2. Once we have the public and private keys, we can [deploy](#deploy_certs_wallet) the `certs_wallet` contract, taking into account that `receiver-secret-key` is the private key we generated in Stellar Laboratory.
3. If the deployment is successful, it will generate a `wallet_contract_id` that we must save to continue interacting with our contract. After this, we can [initialize](#initialize_certs_wallet) our `certs_wallet`, where once again, our `receiver-secret-key` corresponds to the private key we generated in the first step, our `wallet_contract_id` corresponds to the ID generated during deployment in the previous step, and our `receiver-public-key` corresponds to the public key generated in the first step.
4. Now that we have initialized our `certs_wallet`, we can receive certificates from an organization. However, to do this we must first create one by [deploy](#deploy_certs_governance) the `certs_governance` contract. We also need to generate the public and private keys in the Stellar Laboratory.
5. After deploying the `certs_governance` contract, we can [initialize](#initialize_certs_governance) and create our certificate according to our own governance rules.
6. Now that we have both contracts deployed and initialized, we can send a certificate from the `certs_governance` contract to the `certs_wallet` contract. However, there is one small detail we need to take care of: we need to add the organization we just created to our `Access Control List` (a list that allows organizations to deliver certificates to us). [Add organization](#add_organization).
7. Now that we have added our organization to the `Access Control List`, we can safely and securely [distribute](#distribute) certificates from the `certs_governance` contract to the `certs_wallet` in a controlled manner.
8. If a certificate has been marked as revocable in the `governance_rules`, we can easily and securely [revoke](#revoke) it.

While this is a basic flow of the contracts, there are [other functions](#other_functions) that can provide us with more information.

## Contracts deployment and usage
All this steps require the [Pre-requirements](#pre-requirements) and [Setup](#setup) 

1. Build both contracts with `cargo build --target wasm32-unknown-unknown --release`
2. Deploy `certs_wallet` contract
<a name="deploy_certs_wallet"></a>
```
soroban contract deploy \
    --source-account <receiver-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/certs_wallet.wasm

sucess
sucess
<wallet_contract_id>
```
3. Initialize `certs_wallet` contract
<a name="initialize_certs_wallet"></a>
```
soroban contract invoke \
    --source-account <receiver-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <wallet_contract_id> \
    -- initialize \
    --owner <receiver-public-key>
```
4. Deploy `certs_governance` contract
<a name="deploy_certs_governance"></a>
```
soroban contract deploy \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/certs_governance.wasm
sucess
sucess
<governance_contract_id>
```
<a name="initialize_certs_governance"></a>
5. Initialize `certs_governance` contract, to initialize this contract we can do it by sending the addresses of the receivers that will receive the contract or by sending the number of certificates that can distribute this contract, in case both values enter `null` it is initialized with a distribution limit of 10.

`initialize with distribution_limit`

```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- initialize \
    --file_storage 66696c65 \
    --name 636f6e74726163742031 \
    --receivers null \
    --distribution_limit 5 \
    --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "Some" }, {"u64": 31556926}] }] }' \
    --organization '{"id": "6f726731" , "admin": "<org-public-key>"}'
```

`Initialize with receivers`

```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- initialize \
    --file_storage 66696c65 \
    --name 636f6e74726163742031 \
    --receivers '["<receiver-public-key>","<receiver-public-key>"]' \
    --distribution_limit null \
    --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "Some" }, {"u64": 31556926}] }] }' \
    --organization '{"id": "6f726731" , "admin": "<org-public-key>"}'
```

`Initialize without expiration_date in governance_rules`

```
soroban contract invoke \
    --source-account SC54VFEAE3MXFJ2QYR5TCJDZYIYADAJFUPGZIDIEOFTAYQRMOVLRHM6U \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id a13d49e0eb690e002940fe74e8d1304989a1e0f30218ec2317975925e7cfb17e \
    -- initialize \
    --file_storage 66696c65 \
    --name 636f6e74726163742031 \
    --receivers null \
    --distribution_limit 5 \
    --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "None" }] }] }' \
    --organization '{"id": "6f726731" , "admin": "<org-public-key>"}'
```

> **Note** The `governance_rules` field is a tuple with two fields. The first field is a `bool` value that indicates whether the contract is `revocable` or not. The second field is `expiration_time`, a field of type `OptionU64` that represents the duration of validity of the issued certificate. Time is managed in `Epoch Unix Timestamps` format. For this example, we will use the value of `31556926`, which equals one year. This means that the certificate will only be valid for one year after distribution. For more information on this date format, please visit the following website: https://www.unixtimestamp.com/

6. Add the organization to the `certs_wallet` in order to receive certificates.
<a name="add_organization"></a>
```
soroban contract invoke \
    --source-account <receiver-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <wallet_contract_id> \
    -- add_organization \
    --org_id 6f726731
```
7. The organization has already been added to the `Access Control List` of the wallet, then we can distribute a certificate from the `certs_governance`.
<a name="distribute"></a>
```
soroban contract invoke \
    --source-account SC54VFEAE3MXFJ2QYR5TCJDZYIYADAJFUPGZIDIEOFTAYQRMOVLRHM6U \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id a13d49e0eb690e002940fe74e8d1304989a1e0f30218ec2317975925e7cfb17e \
    -- distribute \
    --admin <org-public-key> \
    --receiver <receiver-public-key> \
    --wallet_contract_id <wallet_contract_id>\
    --cid 516d647479665459625653334b3969597142506a58786e346d624237614276456a59477a59576e7a52634d724543\
    --distribution_date 1681414979
```
8. If at the moment of initializing the `certs_governance` contract in the `governance_rules` field we mark as `true`, we can revoke the certificate.
<a name="revoke"></a>
```
  soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- revoke \
    --admin <org-public-key> \
    --holder <receiver-public-key> \
    --wallet_contract_id <wallet_contract_id>
```

<a name="other_functions"></a>
# Other functions

## Certs Governane
1. This function returns the name of the certificate
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- name
```
2. This function returns whether the certificate is revocable or not.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- is_revocable
```
3. This function returns the expiration time of the certificate after its distribution.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- expiration_time
```
4. This function returns the number of certificates we can distribute.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- distribution_limit
```
6. This function returns the number of certificates we have distributed.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- supply
```
7. This function return then name of decentralized storage service
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- file_storage
```
8. This function returns the receivers that we have stored in the contract.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- receivers
```
9. This function returns information related to the certificate.
```
soroban contract invoke \
    --source-account <org-secret-key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id <governance_contract_id> \
    -- info
```

## Certs wallet
1. This function allows us to remove an organization from the `Access Control List`.
```
soroban contract invoke \
    --source-account SA6BUT3AQCI757TGKZDZYFOX6ABNNR4SOSB3IS6POZ4D7EUOPOBEFB4C \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id b38c29ee40c874afebd34b6b82b9d2107ef87045d475db8b074e2e02c8d49f77 \
    -- remove_organization \
    --org_id 6f726731
```
2. This function returns all the chaincerts that have been sent to us.
```
soroban contract invoke \
    --source-account SA6BUT3AQCI757TGKZDZYFOX6ABNNR4SOSB3IS6POZ4D7EUOPOBEFB4C \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id b38c29ee40c874afebd34b6b82b9d2107ef87045d475db8b074e2e02c8d49f77 \
    -- get_chaincerts
```
3. This function returns the organizations we have in the `Access Control List`.
```
soroban contract invoke \
    --source-account SA6BUT3AQCI757TGKZDZYFOX6ABNNR4SOSB3IS6POZ4D7EUOPOBEFB4C \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id b38c29ee40c874afebd34b6b82b9d2107ef87045d475db8b074e2e02c8d49f77 \
    -- get_access_control_list
```


# Types of errors in the contract
For ease of error handling, it was decided to use error codes. The meaning of each of these codes will be explained below.
| Code | Error | Description |
| --- | --- | --- |
| 1 | AlreadyInit | Contract already initialized
| 2 | NotAuthorized | Does not have administrator permissions
| 3 | LimitReached | It is not possible to issue more chaincerts
| 4 | AlreadyInACL | The organization is already in the ACL
| 5 | AlreadyIssued | Chaincert has already beend issued to the entered address
| 6 | NoOrganizationsInACL | There are no organizations in the ACL
| 7 | NoRevocable | Chaincert cannot be revoked
| 8 | OrganizationNotFound | The organization doen's exist in the ACL
| 9 | ChaincertAlreadyInTheWallet | The chaincert is already deposited in the wallet
| 10 | ChaincertDoesNotExist | The chaincer doesn't exist
| 11 | WalletDoesNotOwnChaincerts | This wallet doesn't own any chaincert for the moment
## Changelog

Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct

We welcome everyone to contribute. Make sure you have read the [CODE_OF_CONDUCT][coc] before.

## Contributing

For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License

This library is licensed under a GNU AGPLv3 license. See [LICENSE][license] for details.

## Acknowledgements

Made with 💙 by [kommitters Open Source](https://kommit.co)

[license]: https://github.com/kommitters/chaincerts_governance/blob/main/LICENSE
[coc]: https://github.com/kommitters/chaincerts_governance/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/chaincerts_governance/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/chaincerts_governance/blob/main/CONTRIBUTING.md
