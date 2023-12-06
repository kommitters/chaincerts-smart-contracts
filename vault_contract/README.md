# Vault Smart Contract

## Features
The vault smart contract is a secure repository for safeguarding verifiable credentials (VCs). With this smart contract, you will be able to:

- Authorize issuers to emit certificates for specific DIDs(Decentralized Identifiers) in the vault.
- Revoke an issuer's authority for a particular DID.
- Store a verifiable credential.
- Retrieve a specific verifiable credential using its identifier.
- Retrieve a list of verifiable credentials organized by DID.
- Register new DIDs in the vault.
- Revoke DIDs in the vault.

## Types

### VerifiableCredential
Represents a verifiable credential with essential attributes for comprehensive identification and validation.

#### Attributes

| Name                   | Type                 | Description                                                |
| ---------------------- | -------------------- | ---------------------------------------------------------- |
| `id`                   | `String`             | Unique identifier for the verifiable credential (e.g., `t5iwuct2njbbcdu2nfwr32ib`). |
| `data`                 | `String`             | The encrypted payload encapsulating the actual data within the credential, utilizing the X25519KeyAgreementKey2020 algorithm for heightened security.|
| `issuance_contract`    | `Address`            | The address of the smart contract responsible for credential issuance. |

#### Example 

```bash
{
  "id": "t5iwuct2njbbcdu2nfwr32ib",
  "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
  "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
}
```

### Vault
Represents a structured entity that encapsulates a DID along with its corresponding verifiable credentials.

#### Attributes

| Name            | Type                           | Description                                                |
| --------------- | ------------------------------ | ---------------------------------------------------------- |
| `did`           | `String`                       | The DID associated with the vault. |
| `revoked`    | `bool`                         | Indicates whether the vault has been revoked (`true` if revoked, `false` otherwise). |
| `vcs`           | `Vec<VerifiableCredential>`    | List of Verifiable Credentials associated with the given DID. |


#### Example
```bash
{
  "did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
  "revoked": false,
  "vcs": [
    {
      "id": "t5iwuct2njbbcdu2nfwr32ib",
      "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
      "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
    }
  ]
}
```

## Functions

The following functions define the behavior of the Vault smart contract.

### Initialize
Initializes the vault contract by configuring the administrator and initial Decentralized Identifiers (DIDs).

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

Authorizes an issuer to issue verifiable credentials for a specific DID. If the DID is already registered or revoked, a specific error will be triggered. This function is exclusively accessible to the admin account for invocation.

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

### Revoke Issuer
Revokes an issuer to prevent the issuance of verifiable credentials to a specific DID in the vault. This function is exclusively accessible to the admin account for invocation.

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
Stores a verifiable credential related to a holder's DID. This function is invoked by the issuer within the vc_issuance_contract smart contract.

```rust
fn store_vc(
    e: Env,
    vc_id: String,
    vc_data: String,
    recipient_did: String,
    issuer: Address,
    issuance_contract: Address,
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

### Register Vault
Registers a vault given its DID. The admin account is the only party authorized to invoke this function.

```rust
fn register_vault(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  register_vault \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" 
```

### Revoke Vault
Revokes a vault based on its DID to prevent the issuance of verifiable credentials. The admin account is the only party authorized to invoke this function.

```rust
fn revoke_vault(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  revoke_vault \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" 
```

### Get Vault
Retrieves a vault given its DID.

```rust
fn get_vault(e: Env, did: String) -> Vault;
```

#### Output
Returns a vault.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  get_vault \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" 

# Output: VAULT
{
  "did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
  "revoked": false,
  "vcs": [
    {
      "id": "t5iwuct2njbbcdu2nfwr32ib",
      "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
      "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
    }
  ]
}
```

### List Vaults
Retrieves all the vaults

```rust
fn list_vaults(e: Env) -> Vec<Vault>;
```

#### Output
Returns a list of vault.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  list_vaults

# Output: LIST OF VAULTS
[
  {
    "did": "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h",
    "revoked": false,
    "vcs": [
      {
        "id": "t5iwuct2njbbcdu2nfwr32ib",
        "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
        "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
      }
    ]
  }
]
```

## Contract Errors

| Code | Error | Description |
| --- | --- | --- |
| 1 | `AlreadyInitialized` | Contract has already been initialized |
| 2 | `NotAuthorized` | Invoker lacks the necessary authorization as the contract administrator |
| 3 | `EmptyDIDs` | The array of DIDs is empty |
| 4 | `IssuerNotFound` | The specified issuer was not found |
| 5 | `IssuerRevoked` | The issuer cannot perform the action because it has been revoked |
| 6 | `VaultNotFound` | The specified DID was not found |
| 5 | `VaultRevoked` | The DID cannot perform the action because it has been revoked |
| 8 | `VaultAlreadyRegistered` | The vault was already registered |

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
        --wasm target/wasm32-unknown-unknown/release/soroban_did_contract.wasm

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
