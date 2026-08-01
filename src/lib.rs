#![warn(missing_docs)]
//! A partial implementation of the NFD Management protocol, used to send
//! control commands -- like registering a route -- to a locally running
//! [NFD](https://docs.named-data.net/NFD/current/) forwarder and read its
//! response.
//!
//! [`make_command`] builds the command Interest for a given module/method
//! pair (e.g. `rib`/`register`) and [`ControlParameters`], following NFD's
//! `/localhost/nfd/<module>/<method>/<parameters>` naming convention; the
//! caller is responsible for signing and sending it, since NFD requires
//! command Interests to be signed by a trusted key. [`ControlResponse`]
//! decodes NFD's reply.
//!
//! This crate only implements the parts of the protocol that
//! [`ndn-app`](https://crates.io/crates/ndn-app) needs -- notably,
//! [`ControlParameters`] only carries a name, not the full set of fields
//! (FaceId, Cost, Flags, ...) the real protocol defines.

use bytes::Bytes;
use ndn_protocol::{GenericNameComponent, Interest, Name};
use ndn_tlv::{NonNegativeInteger, Tlv, TlvEncode};

/// The parameters of an NFD control command, encoded as one name
/// component of the command Interest (see [`make_command`]).
///
/// Only carries a name; NFD's control commands support several more
/// optional fields (FaceId, Cost, Flags, ...) this crate doesn't implement.
#[derive(Default, Debug, Tlv, Clone)]
#[tlv(104)]
pub struct ControlParameters {
    name: Option<Name>,
}

/// NFD's numeric status code for a control command, e.g. `200` for success.
#[derive(Debug, Tlv, Clone, Copy)]
#[tlv(102)]
pub struct StatusCode {
    code: NonNegativeInteger,
}

/// NFD's human-readable status message accompanying a [`StatusCode`].
#[derive(Debug, Tlv, Clone)]
#[tlv(103)]
pub struct StatusText {
    text: Bytes,
}

/// NFD's reply to a control command, decoded from the responding Data
/// packet's content.
///
/// `T` is the type the response body decodes as.
#[derive(Debug, Tlv, Clone)]
#[tlv(101)]
pub struct ControlResponse<T> {
    status_code: StatusCode,
    status_text: StatusText,
    body: T,
}

/// Builds the command Interest for `module`/`method` (e.g. `"rib"`,
/// `"register"`), encoding `params` as the name's final component per
/// NFD's `/localhost/nfd/<module>/<method>/<parameters>` convention, and
/// setting `MustBeFresh`.
///
/// Returns `None` if `module`/`method` don't form a valid name. The
/// returned Interest is unsigned.
pub fn make_command(module: &str, method: &str, params: ControlParameters) -> Option<Interest<()>> {
    let mut name = Name::from_str(&format!("ndn:/localhost/nfd/{module}/{method}")).ok()?;
    name.components
        .push(GenericNameComponent::new(params.encode()).into());

    let mut ret = Interest::<()>::new(name);
    ret.set_must_be_fresh(true);

    Some(ret)
}

impl ControlParameters {
    /// Creates an empty `ControlParameters`.
    pub fn new() -> Self {
        ControlParameters { name: None }
    }

    /// Sets the command's target name, e.g. the prefix to register.
    pub fn set_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
}

impl<T> ControlResponse<T> {
    /// The command's status code.
    pub fn status_code(&self) -> NonNegativeInteger {
        self.status_code.code
    }

    /// The status message accompanying the status code.
    pub fn status_text(&self) -> &Bytes {
        &self.status_text.text
    }

    /// The response body.
    pub fn body(&self) -> &T {
        &self.body
    }
}
