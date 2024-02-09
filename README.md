[![CHAINCERTS_LOGO](https://github.com/kommitters/chaincerts-smart-contracts/assets/39246879/5c7c3c50-f435-43ad-87e5-dad223eaa12a)][chaincerts.co]

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
As part of our commitment to transparency, we promote the adoption of a **"build in the open"** approach to developing these contracts. This approach allows anyone to use or inspect the underlying code of smart contracts, ensuring that it works as intended and does not contain any hidden vulnerabilities or malicious code. We aim to foster trust within the ecosystem and community, promoting greater confidence in using Chaincerts.

For more details, please refer to the open source [LICENSE][license].

<br/>

# Smart Contracts
Chaincerts utilizes [Decentralized Identifiers (DIDs)][w3c-did-core-spec] and [Verifiable Credentials (VCs)][w3c-vcs-spec] to establish a secure and interoperable infrastructure for managing digital identities and credentials within the Stellar and Soroban ecosystems.
<br/>

![smart-contracts-workflow](https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/230a708b-7270-4f0c-bf91-53b397a8771a)

<br/>

## Decentralized Identifiers (DIDs)
The DID smart contract strictly adheres to the [W3C DID specification][w3c-did-core-spec] and is a foundational component within the verifiable credential ecosystem. It empowers individuals and issuers to establish and manage their self-sovereign identities on the Stellar blockchain.

[**DID smart contract code**][did-repo]

<br/>

## Verifiable Credentials Issuance
The Verifiable Credentials (VCs) Issuance smart contract establishes the rules for issuing, transferring, and revoking verifiable credentials. It acts as the governing framework, ensuring the secure and standardized management of on-chain verifiable credentials.

This smart contract prioritizes security and privacy by avoiding the persistence of structured data or personal information. Data is handled exclusively in an encrypted form, with access granted only to owners through cryptographic mechanisms. This approach is particularly critical in insecure communication channels such as blockchain ledgers. For more details, refer to the [W3C KeyAgreement specification](https://www.w3.org/TR/did-core/#dfn-keyagreement).

[**Issuance smart contract code**][vcs-issuance-repo]

<br/>

## Vault: Verifiable Credentials Storage
The Vault smart contract is a secure repository for safeguarding Verifiable Credentials (VCs).

Through the implementation of control access mechanisms, the smart contract authorizes issuers to deposit credentials through issuance contracts. VCs stored within the Vault utilize an encryption mechanism that prioritizes security and data privacy.

[**Vault smart contract code**][vcs-storage-repo]

### Deployment

1. To build and optimize all smart contracts, run:

  ```
  chmod +x build.sh
  ```

  ```
  sh build.sh
  ```

2. To install and deploy the respective smart contracts, execute:

  ```
  chmod +x release.sh
  ```

  ```
  sh release.sh
  ```

The output will display the respective WASM IDs and the deployer contract address:

  ```
  Vault contract WASM ID:
  1b2f3c9e5d807a1f2b319c4d6e98f87d4b6a0fc325a1707c08e5f7d1ae2c9b3a
  Issuance contract WASM ID:
  8d4f1e9b6a7032dab92f56c81eaf094d5e1b0a72f8c3ed5706cb08193c4e2f7a
  DID contract WASM ID:
  4fe937d2b8e1a0c6f312dbaf6987c3c0d2d1ae7364c87e0a95e047fd8fb2c1e3
  Deployer contract Address:
  CGKFDN2X8VU4AJHRMZK35YMSYRWBSWTPVJ4I32EHDRRJK9YKU4W9CD9X
  ```

<br/>

# License
This software is licensed under the [Apache License 2.0][license] © kommit.

<br/>

<hr/>

[<img src="https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/d60d775f-166b-4968-89b6-8be847993f8c" width="80px" alt="kommit"/>](https://kommit.co)

<sub>

[Website][kommit-website] •
[Github][kommit-github] •
[X][kommit-x] •
[LinkedIn][kommit-linkedin]

</sub>

[chaincerts.co]: https://chaincerts.co
[soroban-setup]: https://soroban.stellar.org/docs/getting-started/setup
[license]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE
[coc]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CODE_OF_CONDUCT.md
[changelog]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CHANGELOG.md
[contributing]: https://github.com/kommitters/chaincerts-smart-contracts/blob/main/CONTRIBUTING.md
[w3c-did-core-spec]: https://www.w3.org/TR/did-core/
[w3c-vcs-spec]:https://www.w3.org/TR/vc-data-model
[did-repo]: https://github.com/kommitters/soroban-did-contract
[vcs-issuance-repo]: https://github.com/kommitters/chaincerts-smart-contracts/tree/main/vc_issuance_contract
[vcs-storage-repo]: https://github.com/kommitters/chaincerts-smart-contracts/tree/main/vault_contract
[kommit-website]: https://kommit.co
[kommit-github]: https://github.com/kommitters
[kommit-x]: https://twitter.com/kommitco
[kommit-linkedin]: https://www.linkedin.com/company/kommit-co
