# Chaincerts Governance

![Version Badge](https://img.shields.io/github/v/release/kommitters/chaincerts_governance?style=for-the-badge)
[![License badge](https://img.shields.io/github/license/kommitters/chaincerts_governance?style=for-the-badge)](https://github.com/kommitters/chaincerts_governance/blob/main/LICENSE)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts_governance?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts_governance)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts_governance?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts_governance)

**Chaincerts Governance** is a set of smart contracts designed for the creation, distribution, and revocation of digital certificates on the blockchain. The goal of this project is to provide a decentralized and secure way to issue and manage certificates using Soroban smart contracts for [**Chaincerts**](https://chaincerts.co).

This repository contains two smart contracts:

- [**Cert Governance Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_governance):
  The `cert_governance` contract is responsible for defining the governance rules for Chaincerts. With this contract, organizations can set the revocability and expiration of certificates, define which users can receive a certificate, and restrict the distribution of certificates. Additionally, the `cert_governance` contract is responsible for executing the distribution and revocation actions for certificates to different users.

- [**Cert Wallet Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_wallet):
  The `cert_wallet` contract stores the Chaincerts of a specific user, and each user must have their own `cert_wallet`. This contract allows users to view their Chaincerts, add organizations authorized to issue certificates to them, and execute distribution and revocation actions on their certificates. Only functions that can be executed by the `cert_governance` contract can modify certificates in a `cert_wallet`.

## System overview
The following image illustrates how the smart contracts manage the certificate governance process.
![Workflow](https://user-images.githubusercontent.com/32212593/232539606-6999b76c-f005-4771-83f2-323622dfaded.jpeg)

## Contracts workflow
1. In order to deploy the contracts, we require an account that can be created on the Stellar laboratory using the Friendbot tool.
2. Two key aspects are involved in this process: the **wallet contract**, which enables users to hold and manage their certificates, and the **governance contract**, which allows organizations to issue, verify, and distribute certificates to user wallets.
3. Both contracts need to be initialized to add the fundamental configuration to them.
4. To enable organizations to distribute certificates to the wallet, wallets must include these organizations on their **Access Control List** for authorization purposes.
5. Once a wallet authorizes an organization, it can proceed with certificate distribution using the governance contract.
    * Organizations can revoke certificates as well, provided that revocability is allowed by the governance contract.
	* Certificates can be set to expire if an **expiration_time** is specified during the contract initialization.
	* Revocation of a certificate is only permissible by the organization that distributed it.


## Pre-requirements
To work with the contracts, Rust and Soroban need to be installed. The process for installation is detailed in the [Soroban setup](https://soroban.stellar.org/docs/getting-started/setup).

## Setup
``` 
# Clone the repository 
git clone git@github.com:kommitters/chaincerts_governance.git

# Build the project and get dependencies 
cd chaincerts_governance
cargo build
 ```

## Testing
To test the contract run `cargo test -- --show-output`
## Contracts deployment and usage

All this steps require the [Pre-requirements](#pre-requirements) and [Setup](#setup) 

1. Build both contracts (Wallet & Governance) with `cargo build --target wasm32-unknown-unknown --release`
2. Create the deployer account with the [Friendbot](https://laboratory.stellar.org/#account-creator?network=futurenet)
3. Deploy **certs_governance** contract
    ```
    soroban contract deploy \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --wasm target/wasm32-unknown-unknown/release/certs_governance.wasm
    SUCCESS
    SUCCESS
    <governance_contract_id>
    ```
4. To initialize the **certs_governance** contract, we can either send the addresses of the receivers who will receive the contract or specify the number of certificates that can be distributed through this contract. If both values are null, the contract will be initialized with a distribution limit of 10.
    > **Note** The arguments of type `Bytes` must be in hexadecimal format.
    >
    > * **name**: "senior developer"
    > * **name converted**: 73656e696f7220646576656c6f706572
    > ---
    > * **file_storage**: "Filebase"
    > * **file_storage converted**: 46696c6562617365
    > ---
    > * **organization id**: "org_id_123"
    > * **organization id converted**: 6f72675f69645f313233

    **Initialize without expiration_date**

    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- initialize \
        --file_storage 46696c6562617365 \
        --name 73656e696f7220646576656c6f706572 \
        --receivers null \
        --distribution_limit 5 \
        --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "None" }] }] }' \
        --organization '{"id": "6f72675f69645f313233" , "admin": "<org-account-public-key>"}'
    ```

    > **Note** The `governance_rules` argument is a tuple with two fields. The first field is a `bool` value that indicates whether the contract is `revocable` or not. The second field is the `expiration_time`, a field of type `OptionU64` that represents the duration of validity of the issued certificate, which is managed in `Epoch Time` format. For this example, we will use the value of `31556926`, which equals one year. This means that the certificate will only be valid for one year after distribution. For more information on this date format, please visit the following website: https://www.unixtimestamp.com/

    **initialize with distribution_limit and expiration_date**

    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- initialize \
        --file_storage 4669726562617365 \
        --name 73656e696f7220646576656c6f706572 \
        --receivers null \
        --distribution_limit 5 \
        --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "Some" }, {"u64": 31556926}] }] }' \
        --organization '{"id": "6f72675f69645f313233" , "admin": "<org-account-public-key>"}'
    ```

    **Initialize with receivers and expiration_date**

    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- initialize \
        --file_storage 4669726562617365 \
        --name 73656e696f7220646576656c6f706572 \
        --receivers '["<user-account-public-key-1>","<user-account-public-key-2>"]' \
        --distribution_limit null \
        --governance_rules '{ "vec": [{ "bool": true }, { "vec": [{ "symbol": "Some" }, {"u64": 31556926}] }] }' \
        --organization '{"id": "6f72675f69645f313233" , "admin": "<org-account-public-key>"}'
    ```

5. Create user account with the [Friendbot](https://laboratory.stellar.org/#account-creator?network=futurenet)
6. Deploy **certs_wallet** contract
    ```
    soroban contract deploy \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --wasm target/wasm32-unknown-unknown/release/certs_wallet.wasm

    SUCCESS
    SUCCESS
    <wallet_contract_id>
    ```
7. Initialize **certs_wallet** contract
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- initialize \
        --owner <user-account-public-key>
    ```
8. Include the organization on the wallet **Access Control List**.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- add_organization \
        --org_id 6f72675f69645f313233
    ```
9. Verify organization was succesfully included.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- get_access_control_list
    ```
10. Verify there aren't chaincerts on the wallet.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- get_chaincerts
    ```
11. Once the organization has been added to the **Access Control List** of the wallet, we can proceed with distributing a certificate from the **certs_governance** contract.
    > **Note** The cid argument refers to the IPFS content id, it must be in hexadecimal format.
    > * **CID**: QmerTm8dYitCQQSGd33saPFyMWgd2de8KV63KhYUL9hC7S
    > * **CID converted**: 516d6572546d3864596974435151534764333373615046794d576764326465384b5636334b6859554c3968433753
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- distribute \
        --admin <org-account-public-key> \
        --receiver <user-account-public-key> \
        --wallet_contract_id <wallet_contract_id> \
        --cid 516d6572546d3864596974435151534764333373615046794d576764326465384b5636334b6859554c3968433753 \
        --distribution_date 1681414979
    ```
12. Verify that the supply increase one
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- supply
    ```
13. Verify the wallet now contains a certificate.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- get_chaincerts
    ```
14. Revoke the certificate using the Governance contract.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- revoke \
        --admin <org-account-public-key> \
        --holder <user-account-public-key> \
        --wallet_contract_id <wallet_contract_id>
    ```
15. Verify certificate is marked as revoked on the wallet.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <wallet_contract_id> \
        -- get_chaincerts
    ```

# Additional Functions:

## Certs Governane
1. This function returns the name of the certificate
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- name
    ```
2. This function returns whether the certificate is revocable or not.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- is_revocable
    ```
3. This function returns the expiration time used to calculate the expiration_date.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- expiration_time
    ```
4. This function returns the number of certificates the contract allows to distribute.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- distribution_limit
    ```
5. This function return then name of decentralized storage service used for storing the certificates data.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- file_storage
    ```
7. This function returns the receivers stored in the contract.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- receivers
    ```
8. This function returns the contract information provided during the initialization.
    ```
    soroban contract invoke \
        --source-account <source-account-secret-key> \
        --rpc-url https://rpc-futurenet.stellar.org:443 \
        --network-passphrase 'Test SDF Future Network ; October 2022' \
        --id <governance_contract_id> \
        -- info
    ```

## Certs wallet
This function allows us to remove an organization from the **Access Control List**.
```
	soroban contract invoke \
		--source-account <source-account-secret-key> \
		--rpc-url https://rpc-futurenet.stellar.org:443 \
		--network-passphrase 'Test SDF Future Network ; October 2022' \
		--id <wallet_contract_id> \
		-- remove_organization \
		--org_id 6f72675f69645f313233
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
