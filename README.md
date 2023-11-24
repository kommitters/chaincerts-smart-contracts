![CHAINCERTS_LOGO](https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/a43a4a8b-932b-47e5-af63-470e35ab9330)

Issue, store, and verify your credentials backed by smart contracts on Soroban.

**[Chaincerts](https://chaincerts.co/)** are Web3 Verifiable Credentials ([as standardized by the W3C][w3c-vcs-spec]).

![Version Badge](https://img.shields.io/github/v/release/kommitters/chaincerts-smart-contracts?style=for-the-badge)
[![License badge](https://img.shields.io/github/license/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/chaincerts-smart-contracts/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts-smart-contracts)
[![OpenSSF Scorecard](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts-smart-contracts?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts-smart-contracts)
<br/>
<br/>

# Build in the Open ☝️
As part of our commitment to transparency, we promote the adoption of a _"build in the open"_ approach to developing these contracts. This approach allows anyone to review and inspect the underlying code of smart contracts, ensuring that they work as intended and do not harbor any hidden vulnerabilities or malicious code. We aim to foster trust within the ecosystem and community, promoting greater confidence in using Chaincerts.

For more details, please refer to the open source [LICENSE][license].

<br/>

# Smart Contracts
Chaincerts utilizes [Decentralized Identifiers (DIDs)][w3c-did-core-spec] and [Verifiable Credentials (VCs)][w3c-vcs-spec] to establish a secure and interoperable infrastructure for managing digital identities and credentials within the Stellar and Soroban ecosystems.
<br/>

![smart-contracts-workflow](https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/230a708b-7270-4f0c-bf91-53b397a8771a)

<br/>

## Decentralized Identifiers (DIDs)
The DID smart contract strictly adheres to the [W3C DID specification][w3c-did-core-spec] and is a foundational component within the verifiable credential ecosystem. It empowers individuals and issuers to establish and manage their self-sovereign identities on the Stellar blockchain.

>  🤝
> In line with our commitment to contribute to the **Stellar community**, we have developed a Soroban DID contract that serves as an interface. This contract can be utilized by anyone in the community seeking to innovate with a solution that follows the W3C specification.

[**DID smart contract code**][did-repo]

<br/>

## Verifiable Credentials Issuance
The Verifiable Credentials (VCs) Issuance smart contract establishes the foundational rules for issuing, transferring, and revoking verifiable credentials. It acts as the governing framework, ensuring the secure and standardized management of on-chain verifiable credentials.

This smart contract prioritizes security and privacy by avoiding the persistence of structured data or personal information. Data is handled exclusively in an encrypted form, with access granted only to owners through cryptographic mechanisms. This approach is particularly critical in insecure communication channels such as blockchain ledgers. For more details, refer to the [W3C KeyAgreement specification](https://www.w3.org/TR/did-core/#dfn-keyagreement).

[**Issuance smart contract code**][vcs-issuance-repo]

<br/>

## Vault: Verifiable Credentials Storage
The Vault smart contract is a secure repository for safeguarding Verifiable Credentials (VCs).

Through the implementation of control access mechanisms, the smart contract authorizes issuers to deposit credentials through issuance contracts. VCs stored within the Vault utilize an encryption mechanism that prioritizes security and data privacy, ensuring only credential owners can view the information.

[**Vault smart contract code**][vcs-storage-repo]

<br/>

# Changelog
Features and bug fixes are listed in the [CHANGELOG][changelog] file.

<br/>

# Code of conduct
We welcome everyone to contribute. Make sure you have read the [CODE_OF_CONDUCT][coc] before.

<br/>

# Contributing
For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

<br/>

# License
This library is licensed under a GNU AGPLv3 license. See [LICENSE][license] for details.

<br/>

# Acknowledgements
Made with 💙 by the [kommit OSPO](https://kommit.co)

[chaincerts.co]: https://chaincerts.co
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[license]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE
[coc]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CONTRIBUTING.md
[w3c-did-core-spec]: https://www.w3.org/TR/did-core/
[w3c-vcs-spec]:https://www.w3.org/TR/vc-data-model
[did-repo]: https://github.com/kommitters/soroban-did-contract
[vcs-issuance-repo]: https://github.com/kommitters/chaincerts-smart-contracts/tree/main/issuance_contract
[vcs-storage-repo]: https://github.com/kommitters/chaincerts-smart-contracts/tree/main/vault_contract
