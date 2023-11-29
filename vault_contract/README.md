# Vault Smart Contract
[![Release Badge](https://img.shields.io/github/v/release/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/releases)
[![License Badge](https://img.shields.io/github/license/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://github.com/kommitters/chaincerts-smart-contracts/blob/main/LICENSE)
![Build Badge](https://img.shields.io/github/actions/workflow/status/kommitters/chaincerts-smart-contracts/ci.yml?branch=main&style=for-the-badge)
[![Coverage Status](https://img.shields.io/coveralls/github/kommitters/chaincerts-smart-contracts?style=for-the-badge)](https://coveralls.io/github/kommitters/chaincerts-smart-contracts)
[![OSSF-Scorecard Score](https://img.shields.io/ossf-scorecard/github.com/kommitters/chaincerts-smart-contracts?label=openssf%20scorecard&style=for-the-badge)](https://api.securityscorecards.dev/projects/github.com/kommitters/chaincerts-smart-contracts)

> [!IMPORTANT]
>  🤝
> In line with our commitment to contribute to the [Stellar community][stellar], we have developed this vault smart contract that serves as an interface. This contract can be utilized by anyone seeking to innovate with a solution that follows the W3C specification.

## Features
The vault smart contract is a secure repository for safeguarding Verifiable Credentials (VCs). With this smart contract, you will be able to:

- Empower issuers to emit certificates for specific Decentralized Identifiers (DIDs)
- Revoke an issuer's authority for a particular DID.
- Store a verifiable credential.
- Get a verifiable credential by id.
- List verifiable credentials grouped by DID.
- Register new DIDs in the vault.
- Revoke DIDs in the vault.

## Types

### VerifiableCredential
Represents Verifiable Credential.

#### Attributes

| Name                   | Type                 | Description                                                |
| ---------------------- | -------------------- | ---------------------------------------------------------- |
| `id`                   | `String`             | Unique identifier for the verifiable credential (e.g., `vc-1`). |
| `data`                 | `String`             | The actual data contained within the credential encrypted using the X25519KeyAgreementKey2020 algorithm.|
| `holder_did`          | `String`             | The Decentralized Identifier (DID) of the credential holder. |
| `issuance_contract`    | `Address`            | The address of the smart contract responsible for credential issuance. |

#### Example 

```bash
{
  "id": "t5iwuct2njbbcdu2nfwr32ib",
  "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
  "holder_did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
  "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
}
```

### DidWithVCs
Represents a structure mapping to a DID (Decentralized Identifier) along with associated Verifiable Credentials.

#### Attributes

| Name            | Type                           | Description                                                |
| --------------- | ------------------------------ | ---------------------------------------------------------- |
| `did`           | `String`                       | The Decentralized Identifier (DID) associated with the structure. |
| `is_revoked`    | `bool`                         | Indicates whether the DID has been revoked (`true` if revoked, `false` otherwise). |
| `vcs`           | `Vec<VerifiableCredential>`    | List of Verifiable Credentials associated with the given DID. |


#### Example
```bash
{
  "did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
  "is_revoked": false,
  "vcs": [
    {
      "id": "t5iwuct2njbbcdu2nfwr32ib",
      "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
      "holder_did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
      "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
    }
  ]
}
```


## Functions

The following functions define the behavior of the Vault smart contract.

### Initialize
Initializes the Vault Contract by setting the admin and the initial DIDs.

```rust
fn initialize(e: Env, admin: Address, dids: Vec<String>);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  initialize \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --dids '["did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"]'
```

### Authorize Issuer

Authorizes an issuer to issue verifiable credentials to a specific DID. If the DID is already registered or revoked, a specific error will be returned. The admin account is the only party authorized to invoke this function.

```rust
fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  authorize_issuer \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --issuer GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"
```

### Revoke issuer
Revokes an issuer to prevent the issuance of verifiable credentials to a specific DID in the vault. The admin account is the only party authorized to invoke this function.

```rust
fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke_issuer \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --issuer GCPGQ32D7OTELJWJ7G2YBCM5DDXXWKDWFJYRQLOJ4HQCXYFSVXVEBLN3 \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"
```

### Store VC
Stores a verifiable credential related to a holder DID. This function is invoked by the issuer from the vc_issuance_contract smart contract.

```rust
fn store_vc(
    e: Env,
    vc_id: String,
    vc_data: String,
    recipient_did: String,
    issuer_pk: Address,
    issuance_contract_address: Address,
);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  store_vc \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib" \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --recipient_did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" \
  --issuer_pk GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --issuance_contract_address CBRM3HA7GLEI6QQ3O55RUKVRDSQASARUPKK6NXKXKKPWEYLE533GDYQD
```

### Get VC
Retrieves a verifiable credential using its unique identifier.

```rust
fn get_vc(e: Env, vc_id: String) -> VerifiableCredential;
```

#### Output
```bash
{
  "id": "",
  "data": "", 
  "holder_did": "",  
  "issuance_contract": ""
}
```
#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  get_vc \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib"

# Output: VerifiableCredential

{
  "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
  "holder_did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
  "id": "t5iwuct2njbbcdu2nfwr32ib",
  "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
}
```

### List verifiable credentials
Retrieves the list of verifiable credentials from the storage grouped by DID. The admin account is the only party authorized to invoke this function.

```rust
fn list_vcs(e: Env) -> Map<String, DidWithVCs>;
```

#### Output

```bash
{
  "DID.did": {
    "did": "",
    "is_revoked": bool,
    "vcs": [
      {
        "data": "",
        "holder_did": "",
        "id": "",
        "issuance_contract": ""
      }
    ]
  }
}
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  list_vcs

#Output: Map with DIDWithVcs grouped by DID

{
  "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h": {
    "did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
    "is_revoked": false,
    "vcs": [
      {
        "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
        "holder_did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
        "id": "t5iwuct2njbbcdu2nfwr32ib",
        "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
      }
    ]
  }
}
```

### Revoke DID
Revokes a DID based on its DID URI to prevent the issuance of verifiable credentials to the specific DID. The admin account is the only party authorized to invoke this function.

```rust
fn revoke_did(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke_did \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" 
```

### Register DID
Registers a new DID in the vault given a DID URI. The admin account is the only party authorized to invoke this function.

```rust
fn register_did(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  register_did \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" 
```

## Development

### Pre-requirements

In order to develop and test the smart contract, you need to install Rust and Soroban CLI. The process is outlined in the Soroban setup documentation, which can be accessed at [Soroban setup][soroban-setup].

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
        --wasm target/wasm32-unknown-unknown/release/vault_contract.wasm

    CONTRACT_ID
    ```

## Contract Errors

| Code | Error | Description |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | Contract has already been initialized |
| 2 | `NotAuthorized` | Invoker lacks the necessary authorization as the contract administrator |
| 3 | `EmptyDIDs` | The array of DIDs is empty |
| 4 | `IssuerNotFound` | The specified issuer was not found |
| 5 | `DidRevoked` | The DID cannot perform the action because it has been revoked |
| 6 | `DidNotFound` | The specified DID was not found |
| 7 | `IssuerRevoked` | The issuer cannot perform the action because it has been revoked |
| 8 | `VCNotFound` | The Verifiable Credential (VC) was not found |
| 9 | `DuplicatedDID` | The DID is already registered |

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
