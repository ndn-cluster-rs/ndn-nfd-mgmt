use bytes::Bytes;
use ndn_protocol::{GenericNameComponent, Interest, Name};
use ndn_tlv::{NonNegativeInteger, Tlv, TlvEncode};

#[derive(Default, Debug, Tlv, Clone)]
#[tlv(104)]
pub struct ControlParameters {
    name: Option<Name>,
}

#[derive(Debug, Tlv, Clone, Copy)]
#[tlv(102)]
pub struct StatusCode {
    code: NonNegativeInteger,
}

#[derive(Debug, Tlv, Clone)]
#[tlv(103)]
pub struct StatusText {
    text: Bytes,
}

#[derive(Debug, Tlv, Clone)]
#[tlv(101)]
pub struct ControlResponse<T> {
    status_code: StatusCode,
    status_text: StatusText,
    body: T,
}

pub fn make_command(module: &str, method: &str, params: ControlParameters) -> Option<Interest<()>> {
    let mut name = Name::from_str(&format!("ndn:/localhost/nfd/{module}/{method}")).ok()?;
    name.components
        .push(GenericNameComponent::new(params.encode()).into());

    let mut ret = Interest::<()>::new(name);
    ret.set_must_be_fresh(true);

    Some(ret)
}

impl ControlParameters {
    pub fn new() -> Self {
        ControlParameters { name: None }
    }

    pub fn set_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
}

impl<T> ControlResponse<T> {
    pub fn status_code(&self) -> NonNegativeInteger {
        self.status_code.code
    }

    pub fn status_text(&self) -> &Bytes {
        &self.status_text.text
    }

    pub fn body(&self) -> &T {
        &self.body
    }
}
