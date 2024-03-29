## Method get_account_transaction

**Description**

Get the transaction sent by the account with the given sequence number


### Parameters

| Name           | Type           | Description                                                   |
|----------------|----------------|---------------------------------------------------------------|
| account        | string         | Hex-encoded account address                                   |
| sequence       | unsigned int64 | The account sequence number                                   |
| include_events | boolean        | Set to true to also fetch [events](type_event.md) generated by the transaction |

### Returns

[Transaction](type_transaction.md) - If transaction exists

Null - If transaction does not exist


### Example


```
// Request: fetches transaction for account address "1668f6be25668c1a17cd8caf6b8d2f25" and sequence number 0, with including events associated with this transaction
curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"get_account_transaction","params":["1668f6be25668c1a17cd8caf6b8d2f25", 0, true],"id":1}' https://testnet.mango.com/v1

// Response
{
  "id": 1,
  "jsonrpc": "2.0",
  "diem_chain_id": 2,
  "diem_ledger_timestampusec": 1596694618402871,
  "diem_ledger_version": 3309406,
  "result": {
    "events": [
      {
        "data": {
          "amount": {
            "amount": 1000000,
            "currency": "XDX"
          },
          "metadata": "",
          "receiver": "262e691ec8c7e3e23470d8c3ee26e1a7",
          "sender": "1668f6be25668c1a17cd8caf6b8d2f25",
          "type": "sentpayment"
        },
        "key": "01000000000000001668f6be25668c1a17cd8caf6b8d2f25",
        "sequence_number": 0,
        "transaction_version": 106548
      },
      {
        "data": {
          "amount": {
            "amount": 1000000,
            "currency": "XDX"
          },
          "metadata": "",
          "receiver": "262e691ec8c7e3e23470d8c3ee26e1a7",
          "sender": "1668f6be25668c1a17cd8caf6b8d2f25",
          "type": "receivedpayment"
        },
        "key": "0000000000000000262e691ec8c7e3e23470d8c3ee26e1a7",
        "sequence_number": 1,
        "transaction_version": 106548
      }
    ],
    "gas_used": 175,
    "hash": "0fa27a781a9086e80a870851ea4f1b14090fb8b5bd9933e27447ab806443e08e",
    "transaction": {
      "chain_id": 2,
      "expiration_timestamp_secs": 100000000000,
      "gas_currency": "XDX",
      "gas_unit_price": 0,
      "max_gas_amount": 1000000,
      "public_key": "f549a91fb9989883fb4d38b463308f3ea82074fb39ea74dae61f62e11bf55d25",
      "script": {
        "amount": 1000000,
        "currency": "XDX",
        "metadata": "",
        "metadata_signature": "",
        "receiver": "262e691ec8c7e3e23470d8c3ee26e1a7",
        "type": "peer_to_peer_transaction"
      },
      "script_hash": "61749d43d8f10940be6944df85ddf13f0f8fb830269c601f481cc5ee3de731c8",
      "sender": "1668f6be25668c1a17cd8caf6b8d2f25",
      "sequence_number": 0,
      "signature": "a181a036ba68fcd25a7ba9f3895caf720af7aee4bf86c4d798050a1101e75f71ccd891158c8fa0bf349bbb66fb0ba50b29b6fb29822dc04071aff831735e6402",
      "signature_scheme": "Scheme::Ed25519",
      "type": "user"
    },
    "version": 106548,
    "vm_status": { "type": "executed" }
  }
}

```
