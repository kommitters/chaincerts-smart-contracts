# Vault Smart Contract
The Vault smart contract is a secure repository for safeguarding verifiable credentials (VCs).

## Features
With this smart contract, you will be able to:

- Authorize an issuer to store verifiable credentials in a vault.
- Revoke an issuer for a specific vault.
- Store a verifiable credential in the recipient's vault.
- Register a vault given its DID.
- Revoke a vault given its DID.
- Retrieve a vault given its DID.
- List all vaults.

## Types

### Issuer
Represents an authorized entity that issues verifiable credentials to a vault.

### Attributes

| Name         | Type      | Values                                            |
| ------------ | --------- | ------------------------------------------------- |
| `public_key` | `Address` | Issuer's public key address.                      |
| `revoked`    | `bool`    | Boolean indicating whether the issuer is revoked. |

### Example

```bash
{
  "public_key": "GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC",
  "revoked": false
}
```

### VerifiableCredential
Represents a digitally signed statement made by an issuer about a DID subject.

#### Attributes

| Name                | Type      | Values                                                                         |
| ------------------- | --------- | ------------------------------------------------------------------------------ |
| `id`                | `String`  | Unique identifier (e.g., `t5iwuct2njbbcdu2nfwr32ib`).                          |
| `data`              | `String`  | VC data encrypted utilizing a key agreement algorithm for heightened security. |
| `issuance_contract` | `Address` | Smart contract address responsible for verifiable credential issuance.         |

#### Example

```bash
{
  "id": "t5iwuct2njbbcdu2nfwr32ib",
  "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
  "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"
}
```

### Vault
Represents a secure container associated with a specific DID, capable of storing a collection of verifiable credentials.

#### Attributes

| Name      | Type                        | Values                                                     |
| --------- | --------------------------- | ---------------------------------------------------------- |
| `did`     | `String`                    | DID URI (e.g., `did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h`). |
| `revoked` | `bool`                      | Boolean indicating whether the vault is revoked.           |
| `vcs`     | `Vec<VerifiableCredential>` | List of [VerifiableCredentials](#verifiablecredential).    |


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

Initializes the contract by setting the admin and creating a vault for each DID. An error will be triggered if the contract has already been initialized.

```rust
fn initialize(e: Env, admin: Address, dids: Vec<String>);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  initialize \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --dids '["did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"]'

```

### Authorize Issuer

Authorizes an issuer to store verifiable credentials in a vault given its DID. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is not registered.
- Vault is registered but revoked.


```rust
fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  authorize_issuer \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --issuer GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"

```

### Revoke Issuer
Revokes an issuer for a specific vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is not registered.
- Vault is registered but revoked.

```rust
fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  revoke_issuer \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --issuer GCPGQ32D7OTELJWJ7G2YBCM5DDXXWKDWFJYRQLOJ4HQCXYFSVXVEBLN3 \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"
```

### Store VC
Stores a verifiable credential into a vault given the recipient DID. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:

- Issuer is not registered.
- Issuer is registered but revoked.
- Vault is not registered.
- Vault is registered but revoked.

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
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  store_vc \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib" \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --recipient_did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" \
  --issuer GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --issuance_contract CAVN6QFZP2WMB5WIF5EVBBW3LUDDJ62BWLP23EBCX56AS2HGXFIJXK7R
```

### Register Vault
Registers a vault given its DID. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is already registered.

```rust
fn register_vault(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  register_vault \
  --admin GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA \
  --did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h"
```

### Revoke Vault
Revokes a vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is not registered.

```rust
fn revoke_vault(e: Env, admin: Address, did: String);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
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
Retrieves all the vaults.

```rust
fn list_vaults(e: Env) -> Vec<Vault>;
```

#### Output
Returns a list of vaults.

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

| Code | Error                    | Description                                                             |
| ---- | ------------------------ | ----------------------------------------------------------------------- |
| 1    | `AlreadyInitialized`     | Contract has already been initialized                                   |
| 2    | `NotAuthorized`          | Invoker lacks the necessary authorization as the contract administrator |
| 3    | `EmptyDIDs`              | The array of DIDs is empty                                              |
| 4    | `IssuerNotFound`         | The specified issuer was not found                                      |
| 5    | `IssuerRevoked`          | The issuer cannot perform the action because it has been revoked        |
| 6    | `VaultNotFound`          | The specified Vault given its DID was not found                         |
| 5    | `VaultRevoked`           | The action cannot be performed because the vault has been revoked       |
| 8    | `VaultAlreadyRegistered` | The vault was already registered                                        |


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
