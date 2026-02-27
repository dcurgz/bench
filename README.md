## BENCH

> [!CAUTION]
> Until further notice, bench has not been audited by security researchers.
> This will happen eventually, but until then you should assume the client code
> contains critical vulnerabilities and cannot facilitate secure message
> channels.

Bench is an E2EE peer-to-peer chat application that lets clients communicate
over secure message relays. This is how it works:

1. benchd is the client daemon and contains all crytographic logic, a double
rachet state machine, a message store, a wire protocol, chat/channel/community
management, and message forwarding logic. It exposes a local IPC socket for UI
clients. It is written in Rust.
2. bench-relay is a relay server implementation; it is a stateless message
router. Clients talk to each other with encrypted payloads over message relays
that they partially trust. Eventually, relay servers will be federated, so you
don't need to share a relay in order to talk to a friend. It is also written in
Rust.
3. bench-identity, bench-crypto and bench-protocol are Rust libraries that
expose encryption and message primitives to enable higher-order functionality
in benchd and bench-relay
4. park is the reference client UI that connects to the local benchd socket. It
is a tauri application i.e. Rust-backend, web-frontend. Users can elect to use
any client UI that they want, given it connects to the local benchd socket. It
is also possible to use an alternative spec-adherent benchd implementation, if
one ever comes to exist.

I don't know how mobile will work, but we'll cross that bridge when we come to
it.

### ROADMAP

- [ ] bench-identity
    - [ ] client identity creation.
    - [ ] client identity metadata.
- [ ] bench-identity (unit tests)
- [ ] bench-crypto
    - [ ] initial p2p key agreement.
    - [ ] double ratchet algorithm (of signal fame).
    - [ ] double ratchet state.
    - [ ] session management.
- [ ] bench-crypto (unit tests)
- [ ] bench-protocol (protobuf)
    - [ ] outer message frame.
    - [ ] client message payloads:
        - [ ] Hello, Ack
        - [ ] PreKeyRequest, PreKeyResponse
        - [ ] RequestIdentity, GiveIdentity
        - [ ] IdentityUpdate
        - [ ] TextMessage
        - [ ] GetMessages, GiveMessages
        - [ ] CreateChannel, DeleteCommunity
        - [ ] ChannelInvite
        - [ ] JoinChannel, LeaveChannel
        - [ ] CreateCommunity, DeleteCommunity
        - [ ] CommunityInvite
        - [ ] JoinCommunity, LeaveCommunity
        - [ ] AdminAction
    - [ ] relay message payloads:
        - [ ] RelayHello, RelayAck
        - [ ] RelayIdentity
        - [ ] RelayMessage, RelayResponse
- [ ] bench-protocol (unit tests)
- [ ] poc: client transport (integration test)
- [ ] bench-relay
    - [ ] accept tls connections.
    - [ ] identify clients.
    - [ ] in-memory connection map.
    - [ ] forward messages to recipients.
- [ ] poc: relay transport (integration test)
- [ ] benchd
    - [ ] lifecycle.
    - [ ] local database (sqlite).
    - [ ] relay configuration.
    - [ ] relay connectivity.
    - [ ] identity awareness.
    - [ ] identity table.
    - [ ] channels.
    - [ ] channel table.
    - [ ] channel administration. (not blockchain)
    - [ ] channel action table.
    - [ ] message sending.
    - [ ] message retry.
    - [ ] message table.
    - [ ] communities.
    - [ ] community administration. (not blockchain)
    - [ ] community action table.
    - [ ] community table.
    - [ ] community fork.
    - [ ] forwarding: offers (opt-in).
    - [ ] forwarding: trusted forwarders (part of identity).
    - [ ] forwarding: message retry.
    - [ ] forwarding: message TTL.
- [ ] benchd: ipc socket
- [ ] poc: benchd client communication (integration test)
- [ ] park (reference ui)
    - [ ] connect rust backend to benchd IPC socket.
    - [ ] send and receive ipc messages.
    - [ ] ui:  window.
    - [ ] ui:  create identity.
    - [ ] ui:  restore identity (from file).
    - [ ] ipc: identity creation.
    - [ ] ui:  chat application.
    - [ ] ui:  channel rendering.
    - [ ] ipc: fetch and listen for channel creation.
    - [ ] ui:  add friend.
    - [ ] ipc: add friend / connect to client.
    - [ ] ui:  accept channel invite.
    - [ ] ipc: join channel.
    - [ ] ui:  message rendering.
    - [ ] ipc: fetch and listen for message creation.
    - [ ] ui:  send message.
    - [ ] ipc: send message.
    - [ ] ui:  user profiles.
    - [ ] ipc: user metadata.
    - [ ] ipc: listen for user identity updates.
    - [ ] ui:  my profile.
    - [ ] ipc: send identity updates.
    - [ ] ui:  join community.
    - [ ] ipc: join community.
    - [ ] ui:  community rendering.
