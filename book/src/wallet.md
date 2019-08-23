## morgan-wallet CLI

The [morgan crate](https://crates.io/crates/morgan) is distributed with a command-line interface tool

### Examples

#### Get Pubkey

```sh
// Command
$ morgan-wallet address

// Return
<PUBKEY>
```

#### Airdrop Difs

```sh
// Command
$ morgan-wallet airdrop 123

// Return
"Your balance is: 123"
```

#### Get Balance

```sh
// Command
$ morgan-wallet balance

// Return
"Your balance is: 123"
```

#### Confirm Transaction

```sh
// Command
$ morgan-wallet confirm <TX_SIGNATURE>

// Return
"Confirmed" / "Not found" / "Transaction failed with error <ERR>"
```

#### Deploy program

```sh
// Command
$ morgan-wallet deploy <PATH>

// Return
<PROGRAM_ID>
```

#### Unconditional Immediate Transfer

```sh
// Command
$ morgan-wallet pay <PUBKEY> 123

// Return
<TX_SIGNATURE>
```

#### Post-Dated Transfer

```sh
// Command
$ morgan-wallet pay <PUBKEY> 123 \
    --after 2018-12-24T23:59:00 --require-timestamp-from <PUBKEY>

// Return
{signature: <TX_SIGNATURE>, processId: <PROCESS_ID>}
```
*`require-timestamp-from` is optional. If not provided, the transaction will expect a timestamp signed by this wallet's secret key*

#### Authorized Transfer

A third party must send a signature to unlock the difs.
```sh
// Command
$ morgan-wallet pay <PUBKEY> 123 \
    --require-signature-from <PUBKEY>

// Return
{signature: <TX_SIGNATURE>, processId: <PROCESS_ID>}
```

#### Post-Dated and Authorized Transfer

```sh
// Command
$ morgan-wallet pay <PUBKEY> 123 \
    --after 2018-12-24T23:59 --require-timestamp-from <PUBKEY> \
    --require-signature-from <PUBKEY>

// Return
{signature: <TX_SIGNATURE>, processId: <PROCESS_ID>}
```

#### Multiple Witnesses

```sh
// Command
$ morgan-wallet pay <PUBKEY> 123 \
    --require-signature-from <PUBKEY> \
    --require-signature-from <PUBKEY>

// Return
{signature: <TX_SIGNATURE>, processId: <PROCESS_ID>}
```

#### Cancelable Transfer

```sh
// Command
$ morgan-wallet pay <PUBKEY> 123 \
    --require-signature-from <PUBKEY> \
    --cancelable

// Return
{signature: <TX_SIGNATURE>, processId: <PROCESS_ID>}
```

#### Cancel Transfer

```sh
// Command
$ morgan-wallet cancel <PROCESS_ID>

// Return
<TX_SIGNATURE>
```

#### Send Signature

```sh
// Command
$ morgan-wallet send-signature <PUBKEY> <PROCESS_ID>

// Return
<TX_SIGNATURE>
```

#### Indicate Elapsed Time

Use the current system time:
```sh
// Command
$ morgan-wallet send-timestamp <PUBKEY> <PROCESS_ID>

// Return
<TX_SIGNATURE>
```

Or specify some other arbitrary timestamp:

```sh
// Command
$ morgan-wallet send-timestamp <PUBKEY> <PROCESS_ID> --date 2018-12-24T23:59:00

// Return
<TX_SIGNATURE>
```

### Usage

```manpage
morgan-wallet 0.12.0

USAGE:
    morgan-wallet [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
        --rpc-tls    Enable TLS for the RPC endpoint
    -V, --version    Prints version information

OPTIONS:
        --drone-host <IP ADDRESS>    Drone host to use [default: same as --host]
        --drone-port <PORT>          Drone port to use [default: 11100]
    -n, --host <IP ADDRESS>          Host to use for both RPC and drone [default: 127.0.0.1]
    -k, --keypair <PATH>             /path/to/id.json
        --rpc-host <IP ADDRESS>      RPC host to use [default: same as --host]
        --rpc-port <PORT>            RPC port to use [default: 10099]

SUBCOMMANDS:
    address                  Get your public key
    airdrop                  Request a batch of difs
    balance                  Get your balance
    cancel                   Cancel a transfer
    confirm                  Confirm transaction by signature
    deploy                   Deploy a program
    get-transaction-count    Get current transaction count
    help                     Prints this message or the help of the given subcommand(s)
    pay                      Send a payment
    send-signature           Send a signature to authorize a transfer
    send-timestamp           Send a timestamp to unlock a transfer
```

```manpage
morgan-wallet-address
Get your public key

USAGE:
    morgan-wallet address

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```manpage
morgan-wallet-airdrop
Request a batch of difs

USAGE:
    morgan-wallet airdrop <NUM>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <NUM>    The number of difs to request
```

```manpage
morgan-wallet-balance
Get your balance

USAGE:
    morgan-wallet balance

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```manpage
morgan-wallet-cancel
Cancel a transfer

USAGE:
    morgan-wallet cancel <PROCESS_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PROCESS_ID>    The process id of the transfer to cancel
```

```manpage
morgan-wallet-confirm
Confirm transaction by signature

USAGE:
    morgan-wallet confirm <SIGNATURE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SIGNATURE>    The transaction signature to confirm
```

```manpage
morgan-wallet-deploy
Deploy a program

USAGE:
    morgan-wallet deploy <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATH>    /path/to/program.o
```

```manpage
morgan-wallet-get-transaction-count
Get current transaction count

USAGE:
    morgan-wallet get-transaction-count

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```manpage
morgan-wallet-pay
Send a payment

USAGE:
    morgan-wallet pay [FLAGS] [OPTIONS] <PUBKEY> <NUM>

FLAGS:
        --cancelable
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
        --after <DATETIME>                      A timestamp after which transaction will execute
        --require-timestamp-from <PUBKEY>       Require timestamp from this third party
        --require-signature-from <PUBKEY>...    Any third party signatures required to unlock the difs

ARGS:
    <PUBKEY>    The pubkey of recipient
    <NUM>       The number of difs to send
```

```manpage
morgan-wallet-send-signature
Send a signature to authorize a transfer

USAGE:
    morgan-wallet send-signature <PUBKEY> <PROCESS_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PUBKEY>        The pubkey of recipient
    <PROCESS_ID>    The process id of the transfer to authorize
```

```manpage
morgan-wallet-send-timestamp
Send a timestamp to unlock a transfer

USAGE:
    morgan-wallet send-timestamp [OPTIONS] <PUBKEY> <PROCESS_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --date <DATETIME>    Optional arbitrary timestamp to apply

ARGS:
    <PUBKEY>        The pubkey of recipient
    <PROCESS_ID>    The process id of the transfer to unlock
```
