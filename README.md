# Chaincerts Governance

![Version Badge](https://img.shields.io/badge/VERSION-v0.1.0-yellow?style=for-the-badge)
[![License badge](https://img.shields.io/github/license/kommitters/chaincerts_governance?style=for-the-badge)](https://github.com/kommitters/chaincerts_governance/blob/main/LICENSE)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts_governance?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts_governance)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts_governance?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts_governance)

**Chaincerts Governance** is a set of smart contracts designed for the creation, distribution, and revocation of digital certificates on the blockchain. The goal of this project is to provide a decentralized and secure way to issue and manage certificates using Soroban smart contracts for [**Chaincerts**](https://chaincerts.co).

This repository contains two smart contracts:

- [**Cert Governance Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_governance):
  The `cert_governance` contract is responsible for defining the governance rules for Chaincerts. With this contract, users can set the revocability and expiration of certificates, define which users can receive a certificate, and restrict the distribution of certificates. Additionally, the `cert_governance` contract is responsible for executing the distribution and revocation actions for certificates to different users.

- [**Cert Wallet Contract**](https://github.com/kommitters/chaincerts_governance/tree/main/certs_wallet):
  The `cert_wallet` contract stores the Chaincerts of a specific user, and each user must have their own `cert_wallet`. This contract allows users to view their Chaincerts, add organizations authorized to issue certificates to them, and execute distribution and revocation actions on their certificates. Only functions that can be executed by the `cert_governance` contract can modify certificates in a `cert_wallet`.

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
