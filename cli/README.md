# Testing

Create participants accounts:
```bash
trezoa-keygen new --outfile ~/.config/trezoa/id_owner.json --force
trezoa-keygen new --outfile ~/.config/trezoa/id_dest.json --force
trezoa-keygen new --outfile ~/.config/trezoa/id_new_dest.json --force
```

Owner would do all operations, so put some TRZ to his account:
```bash
trezoa airdrop 2 --url https://api.devnet.trezoa.com ~/.config/trezoa/id_owner.json
```

Build program:
```bash
( cd ../program ; cargo build-bpf;  )
```

Deploy program and copy `PROGRAM_ID`.
```bash
trezoa deploy ../program/target/deploy/token_vesting.so --url https://api.devnet.trezoa.com --keypair  ~/.config/trezoa/id_owner.json
```

Create mint and get its public key(`MINT`):
```bash
tpl-token create-token --url https://api.devnet.trezoa.com --fee-payer  ~/.config/trezoa/id_owner.json
```

Create source token account(`TOKEN_ACCOUNT_SOURCE`)
```bash
tpl-token create-account $MINT --url https://api.devnet.trezoa.com --owner ~/.config/trezoa/id_owner.json --fee-payer  ~/.config/trezoa/id_owner.json
```

Mint test source token:
```bash
tpl-token mint $MINT 100000 --url https://api.devnet.trezoa.com $TOKEN_ACCOUNT_SOURCE --fee-payer  ~/.config/trezoa/id_owner.json
```

Create vesting destination token account(`ACCOUNT_TOKEN_DEST`):
```bash
tpl-token create-account $MINT --url https://api.devnet.trezoa.com --owner ~/.config/trezoa/id_dest.json --fee-payer  ~/.config/trezoa/id_owner.json
```

And new one(`ACCOUNT_TOKEN_NEW_DEST`):
```bash
tpl-token create-account $MINT --url https://api.devnet.trezoa.com --owner ~/.config/trezoa/id_new_dest.json --fee-payer  ~/.config/trezoa/id_owner.json
```

Build CLI:

```bash
cargo build
```

Create vesting instance and store its SEED value
```bash
echo "RUST_BACKTRACE=1 ./target/debug/vesting-contract-cli      \
--url https://api.devnet.trezoa.com                             \
--program_id $PROGRAM_ID                                        \
create                                                          \
--mint_address $MINT                                            \
--source_owner ~/.config/trezoa/id_owner.json                   \
--source_token_address $TOKEN_ACCOUNT_SOURCE                    \
--destination_token_address $ACCOUNT_TOKEN_DEST                 \
--amounts 2,1,3,!                                               \
--release-times 1,28504431,2850600000000000,!                   \
--payer ~/.config/trezoa/id_owner.json"                         \
--verbose | bash              
```

To use [Associated Token Account](https://tpl.trezoa.com/associated-token-account) as destination use `--destination_address`(with public key of `id_dest`) instead of `--destination_token_address`.

Observe contract state:
```bash
echo "RUST_BACKTRACE=1 ./target/debug/vesting-contract-cli      \
--url https://api.devnet.trezoa.com                             \
--program_id $PROGRAM_ID                                        \
info                                                            \
--seed $SEED " | bash                                          
```

Change owner:
```bash
echo "RUST_BACKTRACE=1 ./target/debug/vesting-contract-cli      \
--url https://api.devnet.trezoa.com                             \
--program_id $PROGRAM_ID                                        \
change-destination                                              \
--seed $SEED                                                    \
--current_destination_owner ~/.config/trezoa/id_dest.json       \
--new_destination_token_address $ACCOUNT_TOKEN_NEW_DEST         \
--payer ~/.config/trezoa/id_owner.json" | bash                           
```

And unlock tokens according schedule:
```bash
echo "RUST_BACKTRACE=1 ./target/debug/vesting-contract-cli      \
--url https://api.devnet.trezoa.com                             \
--program_id $PROGRAM_ID                                        \
unlock                                                          \
--seed $SEED                                                    \
--payer ~/.config/trezoa/id_owner.json" | bash
```

Create linear vesting:
```bash
echo "RUST_BACKTRACE=1 ./target/debug/vesting-contract-cli      \
--url https://api.devnet.trezoa.com                             \
--program_id $PROGRAM_ID                                        \
create                                                          \
--mint_address $MINT                                            \
--source_owner ~/.config/trezoa/id_owner.json                   \
--source_token_address $TOKEN_ACCOUNT_SOURCE                    \
--destination_token_address $ACCOUNT_TOKEN_DEST                 \
--amounts 42,!                                                  \
--release-frequency 'P1D'                                       \
--start-date-time '2022-01-06T20:11:18Z'                        \
--end-date-time '2022-01-12T20:11:18Z'                          \
--payer ~/.config/trezoa/id_owner.json"                         \
--verbose | bash 
```

## Links

https://tpl.trezoa.com/token
