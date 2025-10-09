# Architecture

"Premature optimization is the root of all evils"

Current crates:
- `crates/node`: P2P Implementation details
- `crates/client`: client side front-end 
- `crates/registry`: centralized RESTful kv database for peer discovery

- What should the main ui look like?
- Figure out base functionality then add more features
- Backend vs frontend interaction?
- P2p nodes

## P2P

- Iroh Pkarr DHT (Distributed Hash Table) for address resolution
- Custom, centralized key value db to map unique juus usernames to node public keys

  ex. { "Gabriel_2008" : "ab78900sadgkaajjss" }

## UI

- Chatting page

## State

- Peer information
- Chat history
