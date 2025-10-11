# Architecture

"Premature optimization is the root of all evils"

Current crates:
- `crates/node`: P2P Implementation details
- `crates/client`: client side front-end 
- `crates/registry`: centralized RESTful kv database for peer discovery

## Project organization

- Package (main Cargo.toml directory)
	- 1 library, n executable crates
- Crate (library | executable)
	- executable has main() entry point
- Module
	- organize items ex. structs, functions etc..
	- c++ namespace

This project should be:
- A package (TODO: move registry into a seperate repository)

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


- Maybe have a group table in the registry database that maps group id's to
  usernames / public keys
- Use MVC (Model, View, Controller) design pattern?

## Design problems

- Scenario:

	Assume a group of nodes: {A, B, C}

	1. A is online, B and C offline
	2. A sends a message then goes offline
	3. B and/or C go online, how do they get the message?

	Decision: DHT running on entire juus network with encrypted messages
	mapped under hashed group id?

	Every time a nodes goes online it first queries the DHT network for an
	messages that may have been published while offline.

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
