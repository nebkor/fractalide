extern crate capnp;

#[macro_use]
extern crate rustfbp;

mod contracts {
    include!("net_ndn_interest.rs");
}
use self::contracts::net_ndn_interest;

component! {
    ForwardingInformationBase,
    inputs( lookup_interest: net_ndn_interest),
    inputs_array(),
    outputs(interest_hit: net_ndn_interest),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {
        Ok(())
    }
}