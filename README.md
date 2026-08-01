# ndn-nfd-mgmt

[![docs.rs](https://img.shields.io/docsrs/ndn-nfd-mgmt)](https://docs.rs/ndn-nfd-mgmt)
[![crates.io](https://img.shields.io/crates/v/ndn-nfd-mgmt)](https://crates.io/crates/ndn-nfd-mgmt)
[![license](https://img.shields.io/crates/l/ndn-nfd-mgmt)](https://github.com/ndn-cluster-rs/ndn-nfd-mgmt/blob/master/LICENSE)

A partial implementation of the NFD Management protocol, used to send control commands -- like registering a route -- to a locally running [NFD](https://docs.named-data.net/NFD/current/) forwarder.

This crate only implements the parts of the protocol that [`ndn-app`](https://crates.io/crates/ndn-app) needs -- it is not a complete implementation of NFD Management.

## Installation

```
cargo add ndn-nfd-mgmt
```

## How it works

- `make_command` builds the command Interest for a module/method pair (e.g. `rib`/`register`) and `ControlParameters`, following NFD's `/localhost/nfd/<module>/<method>/<parameters>` naming convention. The caller is responsible for signing and sending it.
- `ControlParameters` only carries a name; NFD's control commands support several more optional fields (FaceId, Cost, Flags, ...) this crate doesn't implement.
- `ControlResponse` decodes NFD's reply: a status code, a status message, and a body.

## Related crates

- [`ndn-app`](https://crates.io/crates/ndn-app) is an application framework that uses this crate to register routes with a local forwarder.
- [`ndn-tlv`](https://crates.io/crates/ndn-tlv) provides the TLV encoding/decoding traits this crate's types are built on.
- [`ndn-protocol`](https://crates.io/crates/ndn-protocol) implements the Interest and Data packet types this crate's commands are built from.

## License

MIT

---

Produced as part of a Master's thesis in Computer Science.
