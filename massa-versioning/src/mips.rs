#[allow(unused_imports)]
use std::collections::BTreeMap;

#[allow(unused_imports)]
use massa_time::MassaTime;

#[allow(unused_imports)]
use crate::versioning::{MipComponent, MipInfo, MipState};

pub fn get_mip_list() -> [(MipInfo, MipState); 1] {
    // placeholder
    let mip_list = [
        (MipInfo {
            name: "MIP-0005".to_string(),
            version: 2,
            components: BTreeMap::from([
                (MipComponent::Print, 1),
            ]),
            start: MassaTime::from_millis(1708425000000), // 10h30 UTC
            timeout: MassaTime::from_millis(1708439400000), // 14h30 UTC
            activation_delay: MassaTime::from_millis(600000), // 10 mins
        },
        MipState::new(MassaTime::from_millis(1708423073000))) // 10h UTC
    ];

    // debug!("MIP list: {:?}", mip_list);
    #[allow(clippy::let_and_return)]
    mip_list
}
