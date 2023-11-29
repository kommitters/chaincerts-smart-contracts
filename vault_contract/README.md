[<img src="https://github.com/kommitters/chaincerts-smart-contracts/assets/1649973/a43a4a8b-932b-47e5-af63-470e35ab9330" width="300px" />][chaincerts.co]
<br/><br/>

# Vault Smart Contract
The Vault smart contract is a secure repository for safeguarding Verifiable Credentials (VCs).

Through the implementation of control access mechanisms, the smart contract authorizes issuers to deposit credentials through issuance contracts. VCs stored within the Vault utilize an encryption mechanism that prioritizes security and data privacy.

## Development

### Pre-requirements

Before getting started with the development of the Vault smart contract, ensure the following pre-requirements are met:

- [Soroban setup][soroban-setup].

### Setup

1. Clone the repository:
    ```
    git clone git@github.com:kommitters/soroban-did-contract.git
    ```

2. Build the project and install dependencies:
    ```
    cd soroban-did-contract
    soroban contract build
    ```

3. Run tests:
    ```
    cargo test
    ```


## Vault Contract Functions

The following functions define the behavior of the Vault smart contract.

### `initialize`
Initializes the Vault Contract by setting the admin and the initial DIDs.

```rust
fn initialize(e: Env, admin: Address, dids: Vec<String>);
```

#### Parameters:

- e: Environment object.
- admin: Address of the smart contract administrator.
- dids: Vector of strings representing the initial DIDs to be stored.

#### Example:

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

### `authorize_issuer`

Authorizes an issuer to issue verifiable credentials to a specific DID. If the DID is already registered or revoked, a specific error will be returned. The admin account is the only party authorized to invoke this function.

```rust
fn authorize_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

#### Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `issuer`: Address of the issuer to be authorized.
- `did`: String representing the DID associated with the issuer.

#### Example:

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

### `revoke_issuer`
Revokes an issuer to prevent the issuance of verifiable credentials to a specific DID in the vault. The admin account is the only party authorized to invoke this function.

```rust
fn revoke_issuer(e: Env, admin: Address, issuer: Address, did: String);
```

Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `issuer`: Address of the issuer to be revoked.
- `did`: String representing the DID associated with the issuer.

#### Example:

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

### `store_vc`:
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

#### Parameters:

- `e`: Environment object.
- `vc_id`: String representing the unique identifier of the verifiable credential.
- `vc_data`: String containing the encrypted verifiable credential data.
- `recipient_did`: String representing the DID of the credential recipient.
- `issuer_pk`: Address of the issuer's public key.
- `issuance_contract_address`: Address of the contract responsible for credential issuance.

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  store_vc \
  --vc_id "vc_id" \
  --vc_data "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y" \
  --recipient_did "did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h" \
  --issuer_pk GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --issuance_contract_address CBRM3HA7GLEI6QQ3O55RUKVRDSQASARUPKK6NXKXKKPWEYLE533GDYQD
```
### `get_vc`:
Retrieves a verifiable credential using its unique identifier.

```rust
fn get_vc(e: Env, vc_id: String) -> VerifiableCredential;
```

#### Parameters:

- `e`: Environment object.
- `vc_id`: String representing the unique identifier of the verifiable credential.

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  get_vc \
  --vc_id "t5iwuct2njbbcdu2nfwr32ib"

# Response: VerifiableCredential

{"data":"eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y","holder_did":"did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h","id":"t5iwuct2njbbcdu2nfwr32ib","issuance_contract":"CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"}
```

### `list_vcs`:
Retrieves the list of verifiable credentials from the storage grouped by DID. The admin account is the only party authorized to invoke this function.

```rust
fn list_vcs(e: Env) -> Map<String, DidWithVCs>;
```

#### Parameters:

- `e`: Environment object.

#### Example:

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --network testnet \
  -- \
  list_vcs

#Response: Map<String, DidWithVCs>
{"\"did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h\"":{"did":"did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h","is_revoked":false,"vcs":[{"data":"eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y","holder_did":"did:chaincerts:3mtjfbxad3wzh7qa4w5f7q4h","id":"t5iwuct2njbbcdu2nfwr32ib","issuance_contract":"CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65"}]}}
```

### `revoke_did`:
Revokes a DID based on its DID URI to prevent the issuance of verifiable credentials to the specific DID. The admin account is the only party authorized to invoke this function.

```rust
fn revoke_did(e: Env, admin: Address, did: String);
```

#### Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `did`: String representing the DID to be revoked.

#### Example:

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

### `register_did`:
Registers a new DID in the vault given a DID URI. The admin account is the only party authorized to invoke this function.

```rust
fn register_did(e: Env, admin: Address, did: String);
```

#### Parameters:

- `e`: Environment object.
- `admin`: Address of the smart contract administrator.
- `did`: String representing the new DID to be registered.

#### Example:

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

## Deployment

1. Build the contract:
    ```
    soroban contract build
    ```

    This will generate a WASM file for the contract in the `target/wasm32-unknown-unknown/release/` directory.

2. Deploy using Soroban CLI:
    ```bash
    soroban contract deploy \
        --source-account SOURCE_ACCOUNT_SECRET_KEY \
        --rpc-url https://rpc-futurenet.stellar.org \
        --network-passphrase 'Test SDF Network ; October 2022' \
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
