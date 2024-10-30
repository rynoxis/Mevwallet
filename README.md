## MevWallet

MevWallet is a smart contract wallet that allows the user to capture MEV from
Searchers, or create MEV on purpose.

This repo contains the solidity contracts in `contracts/`, deployment tooling
in `script/`, and a Rust library for interacting with the contracts in `src/`.

THIS IS UNAUDITED CODE. USE AT YOUR OWN RISK

### Wait what?

MevWallet is a smart contract wallet that allows the user to capture MEV from
Searchers, or create MEV on purpose.

### Ok yeah I heard you the first time, what does that mean?

User transactions pay fees, and may generate MEV. Searchers take the MEV from
the user, and pay it to block proposers. This means that users are effectively
paying block creators twice. Once via MEV, and once via the regular transaction
fee.

MevWallet changes this relationship. Instead of paying block proposers via tx
fees, users make transactions via the MevWallet. These transaction pay no tx
fee. Instead, MevWallet transactions create MEV, and allow Searchers to pay tx
fees on the user's behalf. This allows users to more effectively and efficiently
price their transactions.

It also allows one more (**really cool** thing). It lets the user give the
Searcher _negative_ MEV. This means that the Searcher's bundle must _pay the
user_ for the right to broadcast the transaction. If the user is creating a
large amount of MEV, they can force the Searcher to give them a cut of that
MEV. The user gets to use this to cover their tx fees (cool!) and potentially
end up paying **less than 0** tx fees (coooler!).

### Addresses

- MevWeth: `0x00000000008C43efC014746c230049e330039Cb3`
- MevWalletV1 Implementation: `0x0000000000c08718718B974D644B098C19bd0064`
- MevWalletV1 ProxyFactory: `0x9248B5e672e1880af34068C0FaE18D30c26D05Fb`

Didn't bother to grind an address for the proxy factory :)

### Repo Setup

`$ git submodule update --init --recursive`
