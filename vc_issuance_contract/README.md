# Verifiable Credentials Issuance Smart Contract


This smart contract prioritizes security and privacy by avoiding the persistence of structured data or personal information. Data is handled exclusively in an encrypted form, with access granted only to owners through cryptographic mechanisms. This approach is particularly critical in insecure communication channels such as blockchain ledgers. For more details, refer to the [W3C KeyAgreement specification](https://www.w3.org/TR/did-core/#dfn-keyagreement).

[![Release Badge](https://img.shields.io/github/v/release/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/releases)
[![License Badge](https://img.shields.io/github/license/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/chaincerts-smart-contracts/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts-smart-contracts)
[![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts-smart-contracts?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts-smart-contracts)

> [!IMPORTANT]
>  🤝
> In line with our commitment to contribute to the [Stellar community][stellar], we have developed this verifiable credential issuance contract that serves as an interface. This contract can be utilized by anyone seeking to innovate with a solution that follows the W3C specification.

## Features
The Verifiable Credentials (VCs) Issuance smart contract establishes the rules for issuing, transferring, and revoking verifiable credentials. It acts as the governing framework, ensuring the secure and standardized management of on-chain verifiable credentials. With this smart contract, you will be able to:

- Issue a verifiable credential.
- Verify a verifiable credential.
- Revoke a verifiable credential.

## Development

### Pre-requirements

Before getting started with the development of the Verifiable Credentials Issuance smart contract, ensure the following pre-requirements are met:

- [Soroban setup][soroban-setup].

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/chaincerts-smart-contracts.git
    ```

2. Build the project and install dependencies:
    ```
    cd chaincerts-smart-contracts
    soroban contract build
    ```

3. Run tests:
    ```
    cargo test
    ```

## Functions

### Initialize
Initializes the VC Issuance Contract by setting the admin.

```rust
fn initialize(e: Env, admin: Address);
```

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  initialize \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA 
```

### Issue:
 Issues a new verifiable credential and returns the Verifiable Credential id as String. The admin account is the only party authorized to invoke this function.

```rust
fn issue(
    e: Env,
    admin: Address,
    recipient_did: String,
    vc_data: String,
    storage_address: Address,
) -> String;
```

#### Output

Returns the verifiable credential id.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --storage_address GR2RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA

# Ouput: VC ID

"t5iwuct2njbbcdu2nfwr32ib"
```

### Verify
Verifies if the Verifiable Credential has been revoked, it returns a Map with the respective status.

```rust
fn verify(e: Env, vc_id: String) -> Map<String, String>;
```

#### Output
```bash
# Valid VC
{
    "status": "valid"
}

# Revoked VC

{
    "status": "revoked", 
    "since": ""
}
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  verify \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib"

# Output: VC Status

{
    "status": "valid"
}
```

### Revoke
Revokes a verifiable credential in a specific date. The admin account is the only party authorized to invoke this function.

```rust
fn revoke(e: Env, admin: Address, vc_id: String, date: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib" \
  --date "01/01/2010 14:10:10"
```

## Contract Errors

| Code | Error | Description |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | Contract has already been initialized |
| 2 | `NotAuthorized` | Invoker lacks the necessary authorization as the contract administrator |
| 3 | `AmountLimitExceeded` | The amount exceeds the issuance contract's capacity for certificates |
| 4 | `VCNotFound` | The Verifiable Credential (VC) was not found |

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/chaincerts-smart-contracts.git
    ```

2. Build the project and install dependencies:
    ```
    cd chaincerts-smart-contracts
    soroban contract build
    ```

3. Run tests:
    ```
    cargo test
    ```

### Deployment

1. Build the contract:
    ```
    soroban contract build
    ```

    This will generate a WASM file for the contract in the `target/wasm32-unknown-unknown/release/` directory.

2. Deploy using Soroban CLI:
    ```
    soroban contract deploy \
        --source SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://soroban-testnet.stellar.org:443 \
        --network-passphrase 'Test SDF Network ; September 2015' \
        --wasm target/wasm32-unknown-unknown/release/vc_issuance_contract.wasm

    CONTRACT_ID
    ```

## Changelog
Features and bug fixes are listed in the [CHANGELOG][changelog] file.

## Code of conduct
We welcome everyone to contribute. Make sure you have read the [CODE OF CONDUCT][coc] before.

## Contributing
For information on how to contribute, please refer to our [CONTRIBUTING][contributing] guide.

## License
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
[kommit-website]: https://kommit.co
[kommit-github]: https://github.com/kommitters
[kommit-x]: https://twitter.com/kommitco
[kommit-linkedin]: https://www.linkedin.com/company/kommit-co
[stellar]: https://stellar.org
