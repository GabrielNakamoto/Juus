# Architecture

"Premature optimization is the root of all evils"

Current crates:
- `crates/node`: P2P Implementation details
- `crates/client`: client side front-end 
- `crates/registry`: centralized RESTful kv database for peer discovery


## To work on
- Safe and informative error handling
- Seperate related logic

## Design questions
- What should the main ui look like?
- Figure out base functionality then add more features
- Backend vs frontend interaction?
- P2p nodes
- Sqlite database distributed across group peers?
- need some way to store what peers you are in a group with

## Main screen (in layers)
1. OAuth signup, creates secret key and registers public key
2. Create / join group
3. Group screen

## P2P

- Iroh Pkarr DHT (Distributed Hash Table) for address resolution
- Custom, centralized key value db to map unique juus usernames to node public keys

  ex. { "Gabriel_2008" : "<pubkey>" }

## UI

- Chatting page

## State

- Peer information
- Chat history
