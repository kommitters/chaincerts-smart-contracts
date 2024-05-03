[![CHAINCERTS_LOGO](https://github.com/kommitters/chaincerts-smart-contracts/assets/39246879/5c7c3c50-f435-43ad-87e5-dad223eaa12a)][chaincerts.co]

# Vault Smart Contract
The Vault smart contract is a secure repository for safeguarding verifiable credentials (VCs).

## Features
With this smart contract, you will be able to:

- Initialize the vault by deploying a DID and storing the corresponding DID URI.
- Authorize a list of issuers to store verifiable credentials in the vault.
- Authorize an issuer to store verifiable credentials in the vault.
- Revoke an issuer for a specific vault.
- Store a verifiable credential in the recipient's vault.
- Revoke the vault.
- Migrate the VCs key for contracts older than version v0.20.0.
- Set the contract admin.
- Upgrade the contract.
- Get the contract version.

## Types
### VerifiableCredential
Represents a digitally signed statement made by an issuer about a DID subject.

#### Attributes

| Name                | Type      | Values                                                                         |
| ------------------- | --------- | ------------------------------------------------------------------------------ |
| `id`                | `String`  | Unique identifier (e.g., `t5iwuct2njbbcdu2nfwr32ib`).                          |
| `data`              | `String`  | VC data encrypted utilizing a key agreement algorithm for heightened security. |
| `issuance_contract` | `Address` | Smart contract address responsible for verifiable credential issuance.         |
| `issuer_did`        | `String`  | DID of the verifiable credential issuer.                                       |

#### Example

```bash
{
  "id": "t5iwuct2njbbcdu2nfwr32ib",
  "data": "eoZXggNeVDW2g5GeA0G2s0QJBn3SZWzWSE3fXM9V6IB5wWIfFJRxPrTLQRMHulCF62bVQNmZkj7zbSa39fVjAUTtfm6JMio75uMxoDlAN/Y",
  "issuance_contract": "CBWDZIBI5NZ77EPSZLJDS3RTM57D3CIBKAIIOFER2TZEZATUYBASYF65",
  "issuer_did": "did:chaincerts:7dotwpyzo2weqj6oto6liic6"
}
```

## Functions

The following functions define the behavior of the Vault smart contract.

### Initialize

Initializes the contract by setting the admin, deploying a DID and storing it within the contract. An error will be triggered if the contract has already been initialized.

```rust
fn initialize(
    e: Env,
    admin: Address,
    did_wasm_hash: BytesN<32>,
    did_init_args: Vec<Val>,
    salt: BytesN<32>,
) -> (Address, Val);
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
  --did_wasm_hash e48a9b26734cff6b2e36117784d4474b5f91f9c50044341811816d8d7e7b63a0 \
  --salt 8752b75c946477e1ef5613d594e2cb25433c886b45117792f00d4c84e6362cec \
  --did_init_args '[{"address":"GC6RRIN6XUZ7NBQS3AYWS6OOWFRLNBOHAYKX3IBYLPKGRODWEANTWJDA"},{"string":"chaincerts"},{"vec":[{"string":"https://www.w3.org/ns/did/v1"},{"string":"https://w3id.org/security/suites/ed25519-2020/v1"},{"string":"https://w3id.org/security/suites/x25519-2020/v1"}]},{"vec":[{"map":{"id":{"string":"keys-1"},"type_":{"vec":[{"symbol":"Ed25519VerificationKey2020"}]},"controller":{"string":""},"public_key_multibase":{"string":"z6MkgpAN9rsVPXJ6DrrvxcsGzKwjdkVdvjNtbQsRiLfsqmuQ"},"verification_relationships":{"vec":[{"symbol":"Authentication"},{"symbol":"AssertionMethod"}]}}}]},{"vec":[{"map":{"id":{"string":"chaincerts"},"service_endpoint":{"string":"https://chaincerts.co"},"type_":{"vec":[{"symbol":"LinkedDomains"}]}}}]}]'

```

### Output
Returns a tuple containing the following values:
- `Address`: DID Contract address that was deployed.
- `Val`: [DID Document](https://github.com/kommitters/soroban-did-contract?tab=readme-ov-file#diddocument) parsed as a `Val` type.

### Authorize Issuers

Set a list of issuers as authorized issuers to store verifiable credentials in the vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is revoked.


```rust
fn authorize_issuers(e: Env, issuers: Vec<Address>);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  authorize_issuers \
  --issuers '["GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC", "GAH6Q4PBWCW2WZAGTEWAL3GUY3YZ2ISGBHGKG44BPFADUQNW6HOWL3GC"]'

```

### Authorize Issuer

Authorizes an issuer to store verifiable credentials in the vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Issuer is already authorized.
- Vault is revoked.


```rust
fn authorize_issuer(e: Env, issuer: Address);
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
  --issuer GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC

```

### Revoke Issuer
Revokes an issuer for the vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
- Invoker is not the contract admin.
- Vault is revoked.

```rust
fn revoke_issuer(e: Env, issuer: Address);
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
  --issuer GCPGQ32D7OTELJWJ7G2YBCM5DDXXWKDWFJYRQLOJ4HQCXYFSVXVEBLN3
```

### Store VC
Stores a verifiable credential into the vault. An authorized issuer must invoke this function.

A contract error will be triggered if:

- Issuer is not authorized.
- Vault is revoked.

```rust
fn store_vc(
    e: Env,
    vc_id: String,
    vc_data: String,
    issuer: Address,
    issuer_did: String,
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
  --issuer GDSOFBSZMFIY5BMZT3R5FCQK6MJAR2PGDSWHOMHZFGFFGKUO32DBNJKC \
  --issuer_did "did:chaincerts:7dotwpyzo2weqj6oto6liic6" \
  --issuance_contract CAVN6QFZP2WMB5WIF5EVBBW3LUDDJ62BWLP23EBCX56AS2HGXFIJXK7R
```

### Revoke Vault
Revokes the vault. The admin account is the only party authorized to invoke this function.

A contract error will be triggered if:
 - Invoker is not the contract admin.
 - Vault is revoked.

```rust
fn revoke_vault(e: Env);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  revoke_vault
```

### Migrate VCs
Migrates the VCs from being stored in a single vector to multiple vectors.

```rust
fn migrate(e: Env);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  migrate
```

### Set contract admin
Replaces the current contract admin with a new one.

```rust
fn set_admin(e: Env, new_admin: Address);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  set_admin \
  --new_admin GCWZBFEKWUGQKYLCLI5ULI4DTXLEA7LPC5QVB55NZPC7FY2NGMLP4YMC

```

### Upgrade contract
Replaces the current contract code with a new one.

```rust
fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
```

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  upgrade \
  --new_wasm_hash 4e3e2a3e6286149775c308c8420fd87c9e5f655549073506f72b917577ef1e33

```

### Get contract version
Returns the contract version.

```rust
fn version(e: Env) -> String;
```

#### Output
Returns the contract version as a string.

#### Example

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source SOURCE_ACCOUNT_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015' \
  -- \
  version

# Output: CONTRACT VERSION
"0.18.0"
```

## Contract Errors

| Code | Error                     | Description                                                   |
| ---- | ------------------------- | ------------------------------------------------------------- |
| 1    | `AlreadyInitialized`      | Contract has already been initialized                         |
| 2    | `IssuerNotAuthorized`     | Specified issuer is not authorized                            |
| 3    | `IssuerAlreadyAuthorized` | Specified issuer is already authorized                        |
| 4    | `VaultRevoked`            | Action cannot be performed because the vault has been revoked |


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
