JSON RPC API
===

Morgan nodes accept HTTP requests using the [JSON-RPC 2.0](https://www.jsonrpc.org/specification) specification.

To interact with a Morgan node inside a JavaScript application, use the [morgan-web3.js](https://github.com/morgan-labs/morgan-web3.js) library, which gives a convenient interface for the RPC methods.

RPC HTTP Endpoint
---

**Default port:** 10099
eg. http://localhost:10099, http://192.168.1.88:10099

RPC PubSub WebSocket Endpoint
---

**Default port:** 10100
eg. ws://localhost:10100, http://192.168.1.88:10100


Methods
---

* [confirmTransaction](#confirmtransaction)
* [getAccountInfo](#getaccountinfo)
* [getBalance](#getbalance)
* [getClusterNodes](#getclusternodes)
* [getRecentBlockhash](#getrecentblockhash)
* [getSignatureStatus](#getsignaturestatus)
* [getSlotLeader](#getslotleader)
* [getNumBlocksSinceSignatureConfirmation](#getnumblockssincesignatureconfirmation)
* [getTransactionCount](#gettransactioncount)
* [getEpochVoteAccounts](#getepochvoteaccounts)
* [requestAirdrop](#requestairdrop)
* [sendTransaction](#sendtransaction)
* [startSubscriptionChannel](#startsubscriptionchannel)

* [Subscription Websocket](#subscription-websocket)
  * [accountSubscribe](#accountsubscribe)
  * [accountUnsubscribe](#accountunsubscribe)
  * [programSubscribe](#programsubscribe)
  * [programUnsubscribe](#programunsubscribe)
  * [signatureSubscribe](#signaturesubscribe)
  * [signatureUnsubscribe](#signatureunsubscribe)

Request Formatting
---

To make a JSON-RPC request, send an HTTP POST request with a `Content-Type: application/json` header. The JSON request data should contain 4 fields:

* `jsonrpc`, set to `"2.0"`
* `id`, a unique client-generated identifying integer
* `method`, a string containing the method to be invoked
* `params`, a JSON array of ordered parameter values

Example using curl:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getBalance", "params":["83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri"]}' 192.168.1.88:10099
```

The response output will be a JSON object with the following fields:

* `jsonrpc`, matching the request specification
* `id`, matching the request identifier
* `result`, requested data or success confirmation

Requests can be sent in batches by sending an array of JSON-RPC request objects as the data for a single POST.

Definitions
---

* Hash: A SHA-256 hash of a chunk of data.
* Pubkey: The public key of a Ed25519 key-pair.
* Signature: An Ed25519 signature of a chunk of data.
* Transaction: A Morgan instruction signed by a client key-pair.

JSON RPC API Reference
---

### confirmTransaction
Returns a transaction receipt

##### Parameters:
* `string` - Signature of Transaction to confirm, as base-58 encoded string

##### Results:
* `boolean` - Transaction status, true if Transaction is confirmed

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"confirmTransaction", "params":["5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":true,"id":1}
```

---

### getBalance
Returns the balance of the account of provided Pubkey

##### Parameters:
* `string` - Pubkey of account to query, as base-58 encoded string

##### Results:
* `integer` - quantity, as a signed 64-bit integer

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getBalance", "params":["83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri"]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":0,"id":1}
```

---

### getClusterNodes
Returns information about all the nodes participating in the cluster

##### Parameters:
None

##### Results:
The result field will be an array of JSON objects, each with the following sub fields:
* `id` - Node identifier, as base-58 encoded string
* `gossip` - Gossip network address for the node
* `tpu` - TPU network address for the node
* `rpc` - JSON RPC network address for the node, or `null` if the JSON RPC service is not enabled

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getClusterNodes"}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":[{"gossip":"10.239.6.48:10001","id":"9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ","rpc":"10.239.6.48:10099","tpu":"10.239.6.48:8856"}],"id":1}
```

---

### getAccountInfo
Returns all information associated with the account of provided Pubkey

##### Parameters:
* `string` - Pubkey of account to query, as base-58 encoded string

##### Results:
The result field will be a JSON object with the following sub fields:

* `difs`, number of difs assigned to this account, as a signed 64-bit integer
* `owner`, array of 32 bytes representing the program this account has been assigned to
* `data`, array of bytes representing any data associated with the account
* `executable`, boolean indicating if the account contains a program (and is strictly read-only)
* `loader`, array of 32 bytes representing the loader for this program (if `executable`), otherwise all

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getAccountInfo", "params":["2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpdST"]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":{"executable":false,"loader":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"owner":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"difs":1,"data":[3,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,20,0,0,0,0,0,0,0,50,48,53,48,45,48,49,45,48,49,84,48,48,58,48,48,58,48,48,90,252,10,7,28,246,140,88,177,98,82,10,227,89,81,18,30,194,101,199,16,11,73,133,20,246,62,114,39,20,113,189,32,50,0,0,0,0,0,0,0,247,15,36,102,167,83,225,42,133,127,82,34,36,224,207,130,109,230,224,188,163,33,213,13,5,117,211,251,65,159,197,51,0,0,0,0,0,0]},"id":1}
```

---

### getRecentBlockhash
Returns a recent block hash from the ledger, and a fee schedule that can be used
to compute the cost of submitting a transaction using it.

##### Parameters:
None

##### Results:
An array consisting of
* `string` - a Hash as base-58 encoded string
* `FeeCalculator object` - the fee schedule for this block hash

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"getRecentBlockhash"}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":["GH7ome3EiwEr7tu9JuTh2dpYWBJK3z69Xm1ZE3MEE6JC",{"difsPerSignature": 0}],"id":1}
```

---

### getSignatureStatus
Returns the status of a given signature.  This method is similar to
[confirmTransaction](#confirmtransaction) but provides more resolution for error
events.

##### Parameters:
* `string` - Signature of Transaction to confirm, as base-58 encoded string

##### Results:
* `null` - Unknown transaction
* `object` - Transaction status:
    * `"Ok": null` - Transaction was successful
    * `"Err": <ERR>` - Transaction failed with TransactionError <ERR> [TransactionError definitions](https://github.com/morgan-labs/morgan/blob/master/sdk/src/transaction.rs#L14)

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getSignatureStatus", "params":["5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":"SignatureNotFound","id":1}
```

-----

### getSlotLeader
Returns the current slot leader

##### Parameters:
None

##### Results:
* `string` - Node Id as base-58 encoded string

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"getSlotLeader"}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":"ENvAW7JScgYq6o4zKZwewtkzzJgDzuJAFxYasvmEQdpS","id":1}
```

-----

### getNumBlocksSinceSignatureConfirmation
Returns the current number of blocks since signature has been confirmed.

##### Parameters:
* `string` - Signature of Transaction to confirm, as base-58 encoded string

##### Results:
* `integer` - count, as unsigned 64-bit integer

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0", "id":1, "method":"getNumBlocksSinceSignatureConfirmation", "params":["5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":8,"id":1}
```

---

### getTransactionCount
Returns the current Transaction count from the ledger

##### Parameters:
None

##### Results:
* `integer` - count, as unsigned 64-bit integer

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"getTransactionCount"}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":268,"id":1}
```

---

### getEpochVoteAccounts
Returns the account info and associated stake for all the voting accounts in the current epoch.

##### Parameters:
None

##### Results:
An array consisting of vote accounts:
* `string` - the vote account's Pubkey as base-58 encoded string
* `integer` - the stake, in difs, delegated to this vote account
* `VoteState` - the vote account's state

Each VoteState will be a JSON object with the following sub fields:

* `votes`, array of most recent vote lockouts
* `node_pubkey`, the pubkey of the node that votes using this account
* `authorized_voter_pubkey`, the pubkey of the authorized vote signer for this account
* `commission`, a 32-bit integer used as a fraction (commission/MAX_U32) for rewards payout
* `root_slot`, the most recent slot this account has achieved maximum lockout
* `credits`, credits accrued by this account for reaching lockouts

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"getEpochVoteAccounts"}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":[[[84,115,89,23,41,83,221,72,58,23,53,245,195,188,140,161,242,189,200,164,139,214,12,180,84,161,28,151,24,243,159,125],10000000,{"authorized_voter_pubkey":[84,115,89,23,41,83,221,72,58,23,53,245,195,188,140,161,242,189,200,164,139,214,12,180,84,161,28,151,24,243,159,125],"commission":0,"credits":0,"node_pubkey":[49,139,227,211,47,39,69,86,131,244,160,144,228,169,84,143,142,253,83,81,212,110,254,12,242,71,219,135,30,60,157,213],"root_slot":null,"votes":[{"confirmation_count":1,"slot":0}]}]],"id":1}
```

---


### requestAirdrop
Requests an airdrop of difs to a Pubkey

##### Parameters:
* `string` - Pubkey of account to receive difs, as base-58 encoded string
* `integer` - difs, as a signed 64-bit integer

##### Results:
* `string` - Transaction Signature of airdrop, as base-58 encoded string

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"requestAirdrop", "params":["83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri", 50]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":"5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW","id":1}
```

---

### sendTransaction
Creates new transaction

##### Parameters:
* `array` - array of octets containing a fully-signed Transaction

##### Results:
* `string` - Transaction Signature, as base-58 encoded string

##### Example:
```bash
// Request
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1, "method":"sendTransaction", "params":[[61, 98, 55, 49, 15, 187, 41, 215, 176, 49, 234, 229, 228, 77, 129, 221, 239, 88, 145, 227, 81, 158, 223, 123, 14, 229, 235, 247, 191, 115, 199, 71, 121, 17, 32, 67, 63, 209, 239, 160, 161, 2, 94, 105, 48, 159, 235, 235, 93, 98, 172, 97, 63, 197, 160, 164, 192, 20, 92, 111, 57, 145, 251, 6, 40, 240, 124, 194, 149, 155, 16, 138, 31, 113, 119, 101, 212, 128, 103, 78, 191, 80, 182, 234, 216, 21, 121, 243, 35, 100, 122, 68, 47, 57, 13, 39, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 40, 240, 124, 194, 149, 155, 16, 138, 31, 113, 119, 101, 212, 128, 103, 78, 191, 80, 182, 234, 216, 21, 121, 243, 35, 100, 122, 68, 47, 57, 11, 12, 106, 49, 74, 226, 201, 16, 161, 192, 28, 84, 124, 97, 190, 201, 171, 186, 6, 18, 70, 142, 89, 185, 176, 154, 115, 61, 26, 163, 77, 1, 88, 98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]}' http://localhost:10099

// Result
{"jsonrpc":"2.0","result":"2EBVM6cB8vAAD93Ktr6Vd8p67XPbQzCJX47MpReuiCXJAtcjaxpvWpcg9Ege1Nr5Tk3a2GFrByT7WPBjdsTycY9b","id":1}
```

---

### Subscription Websocket
After connect to the RPC PubSub websocket at `ws://<ADDRESS>/`:
- Submit subscription requests to the websocket using the methods below
- Multiple subscriptions may be active at once
- All subscriptions take an optional `confirmations` parameter, which defines
  how many confirmed blocks the node should wait before sending a notification.
  The greater the number, the more likely the notification is to represent
  consensus across the cluster, and the less likely it is to be affected by
  forking or rollbacks. If unspecified, the default value is 0; the node will
  send a notification as soon as it witnesses the event. The maximum
  `confirmations` wait length is the cluster's `MAX_LOCKOUT_HISTORY`, which
  represents the economic finality of the chain.

---

### accountSubscribe
Subscribe to an account to receive notifications when the difs or data
for a given account public key changes

##### Parameters:
* `string` - account Pubkey, as base-58 encoded string
* `integer` - optional, number of confirmed blocks to wait before notification.
  Default: 0, Max: `MAX_LOCKOUT_HISTORY` (greater integers rounded down)

##### Results:
* `integer` - Subscription id (needed to unsubscribe)

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"accountSubscribe", "params":["CM78CPUeXjn8o3yroDHxUtKsZZgoy4GPkPPXfouKNH12"]}

{"jsonrpc":"2.0", "id":1, "method":"accountSubscribe", "params":["CM78CPUeXjn8o3yroDHxUtKsZZgoy4GPkPPXfouKNH12", 15]}

// Result
{"jsonrpc": "2.0","result": 0,"id": 1}
```

##### Notification Format:
```bash
{"jsonrpc": "2.0","method": "accountNotification", "params": {"result": {"executable":false,"loader":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"owner":[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"difs":1,"data":[3,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,20,0,0,0,0,0,0,0,50,48,53,48,45,48,49,45,48,49,84,48,48,58,48,48,58,48,48,90,252,10,7,28,246,140,88,177,98,82,10,227,89,81,18,30,194,101,199,16,11,73,133,20,246,62,114,39,20,113,189,32,50,0,0,0,0,0,0,0,247,15,36,102,167,83,225,42,133,127,82,34,36,224,207,130,109,230,224,188,163,33,213,13,5,117,211,251,65,159,197,51,0,0,0,0,0,0]},"subscription":0}}
```

---

### accountUnsubscribe
Unsubscribe from account change notifications

##### Parameters:
* `integer` - id of account Subscription to cancel

##### Results:
* `bool` - unsubscribe success message

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"accountUnsubscribe", "params":[0]}

// Result
{"jsonrpc": "2.0","result": true,"id": 1}
```

---

### programSubscribe
Subscribe to a program to receive notifications when the difs or data
for a given account owned by the program changes

##### Parameters:
* `string` - program_id Pubkey, as base-58 encoded string
* `integer` - optional, number of confirmed blocks to wait before notification.
  Default: 0, Max: `MAX_LOCKOUT_HISTORY` (greater integers rounded down)

##### Results:
* `integer` - Subscription id (needed to unsubscribe)

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"programSubscribe", "params":["9gZbPtbtHrs6hEWgd6MbVY9VPFtS5Z8xKtnYwA2NynHV"]}

{"jsonrpc":"2.0", "id":1, "method":"programSubscribe", "params":["9gZbPtbtHrs6hEWgd6MbVY9VPFtS5Z8xKtnYwA2NynHV", 15]}

// Result
{"jsonrpc": "2.0","result": 0,"id": 1}
```

##### Notification Format:
* `string` - account Pubkey, as base-58 encoded string
* `object` - account info JSON object (see [getAccountInfo](#getaccountinfo) for field details)
```bash
{"jsonrpc":"2.0","method":"programNotification","params":{{"result":["8Rshv2oMkPu5E4opXTRyuyBeZBqQ4S477VG26wUTFxUM",{"executable":false,"difs":1,"owner":[129,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"data":[1,1,1,0,0,0,0,0,0,0,20,0,0,0,0,0,0,0,50,48,49,56,45,49,50,45,50,52,84,50,51,58,53,57,58,48,48,90,235,233,39,152,15,44,117,176,41,89,100,86,45,61,2,44,251,46,212,37,35,118,163,189,247,84,27,235,178,62,55,89,0,0,0,0,50,0,0,0,0,0,0,0,235,233,39,152,15,44,117,176,41,89,100,86,45,61,2,44,251,46,212,37,35,118,163,189,247,84,27,235,178,62,45,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}],"subscription":0}}
```

---

### programUnsubscribe
Unsubscribe from program-owned account change notifications

##### Parameters:
* `integer` - id of account Subscription to cancel

##### Results:
* `bool` - unsubscribe success message

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"programUnsubscribe", "params":[0]}

// Result
{"jsonrpc": "2.0","result": true,"id": 1}
```

---

### signatureSubscribe
Subscribe to a transaction signature to receive notification when the transaction is confirmed
On `signatureNotification`, the subscription is automatically cancelled

##### Parameters:
* `string` - Transaction Signature, as base-58 encoded string
* `integer` - optional, number of confirmed blocks to wait before notification.
  Default: 0, Max: `MAX_LOCKOUT_HISTORY` (greater integers rounded down)

##### Results:
* `integer` - subscription id (needed to unsubscribe)

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"signatureSubscribe", "params":["2EBVM6cB8vAAD93Ktr6Vd8p67XPbQzCJX47MpReuiCXJAtcjaxpvWpcg9Ege1Nr5Tk3a2GFrByT7WPBjdsTycY9b"]}

{"jsonrpc":"2.0", "id":1, "method":"signatureSubscribe", "params":["2EBVM6cB8vAAD93Ktr6Vd8p67XPbQzCJX47MpReuiCXJAtcjaxpvWpcg9Ege1Nr5Tk3a2GFrByT7WPBjdsTycY9b", 15]}

// Result
{"jsonrpc": "2.0","result": 0,"id": 1}
```

##### Notification Format:
```bash
{"jsonrpc": "2.0","method": "signatureNotification", "params": {"result": "Confirmed","subscription":0}}
```

---

### signatureUnsubscribe
Unsubscribe from signature confirmation notification

##### Parameters:
* `integer` - subscription id to cancel

##### Results:
* `bool` - unsubscribe success message

##### Example:
```bash
// Request
{"jsonrpc":"2.0", "id":1, "method":"signatureUnsubscribe", "params":[0]}

// Result
{"jsonrpc": "2.0","result": true,"id": 1}
```
