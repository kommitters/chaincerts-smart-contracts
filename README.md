<img src="https://user-images.githubusercontent.com/1649973/235795202-02794303-b462-43bd-bb06-2e2dbf783d14.png" width="400">

[**Chaincerts**][chaincerts.co] is a cutting-edge platform that revolutionizes the way digital credentials are issued, verified, and showcased.

**Chaincerts** issues secure, non-transferable, and verifiable digital credentials on the Stellar network. Managed through Soroban smart contracts, serve as unique identity credentials that showcase an individual or entity's characteristics, features, traits, and accomplishments in a visually appealing format.

# Chaincerts Smart Contracts

![Version Badge](https://img.shields.io/github/v/release/kommitters/chaincerts-smart-contracts?style=for-the-badge)
[![License badge](https://img.shields.io/github/license/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts-smart-contracts)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts-smart-contracts?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts-smart-contracts)

This repository hosts a collection of smart contracts, specifically developed for the purpose of creating, distributing, and revoking digital credentials using Decentralized Identifiers (DIDs) on the blockchain. The objective of this project is to provide a decentralized, secure, and tamper-proof solution for issuing and managing digital credentials, utilizing the power of Soroban smart contracts.

📖 As part of our commitment to transparency in the Chaincerts system, we encourage the use of a **"build in the open"** approach when developing these smart contracts. This approach allows anyone to easily review and examine the underlying code of the smart contracts, ensuring that they function as intended and do not contain any hidden vulnerabilities or malicious code. By using this approach, we can build trust in the system and promote greater confidence in the use of Chaincerts. Check the Open Source [LICENSE][license] for details.

### Issuance Contract
The Issuance Contract defines the rules for issuing and managing digital credentials within the Chaincerts ecosystem. With this contract, issuing entities can set the revocability and expiration of credentials, define the eligible recipients, and restrict the distribution of credentials. Additionally, the Issuance Contract is responsible for executing the distribution action to different recipients, and it can revoke credentials making them invalid.

### DID Contract
The DID Contract allows to manage a digital identity within the Chaincerts ecosystem. By following the guidelines set forth in the [W3C DID Core specification][w3c-did-core-spec], this contract empowers users to take control of their digital credentials, enabling self-revocation, customizable access capabilities, and streamlined credential issuance authorization.

## Contracts Workflow
The diagram bellow illustrates how the smart contracts manage the credential issuance, revocation, display, and verification processes.

![Chaincerts - Contracts Workflow](https://i.imgur.com/NpaBjsT.jpg)

## Development

### Pre-requirements
In order to utilize the contracts, it is necessary to have Rust and Soroban installed. The installation process is outlined in the Soroban setup documentation, which can be accessed at [Soroban setup][soroban-setup].

### Setup
1. Clone the repository:
```
git clone git@github.com:kommitters/chaincerts-smart-contracts.git
```

2. Build the project and install the dependencies:
```
cd chaincerts-smart-contracts
cargo build
```

3. Run tests:
```
cargo test -- --show-output
```

### Deployment
1. Build the smart contracts:
```
cargo build --target wasm32-unknown-unknown --release
```

2. Deploy using the Soroban CLI:
```
soroban contract deploy \
    --source-account SOURCE_ACCOUNT_SECRET_KEY \
    --rpc-url https://rpc-futurenet.stellar.org \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/issuance_contract.wasm
SUCCESS
SUCCESS

ISSUANCE_CONTRACT_ID
```

```
soroban contract deploy \
    --source-account SOURCE_ACCOUNT_SECRET_KEY \
    --rpc-url https://rpc-futurenet.stellar.org \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --wasm target/wasm32-unknown-unknown/release/did_contract.wasm
SUCCESS
SUCCESS

WALLET_CONTRACT_ID
```

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

[chaincerts.co]: https://chaincerts.co
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[license]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE
[coc]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CONTRIBUTING.md
[w3c-did-core-spec]: https://www.w3.org/TR/did-core/
