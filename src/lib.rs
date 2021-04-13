//! Mappings for APIs of *The Things Network*.

pub mod v2;

#[cfg(test)]
mod test {
    use crate::v2::Uplink;
    use serde_json::json;

    pub fn parse(json: &[u8]) -> Uplink {
        let uplink = serde_json::from_slice(json);

        println!("{:?}", uplink);

        assert!(uplink.is_ok());

        uplink.unwrap()
    }

    #[test]
    pub fn data1() {
        let uplink: Uplink = parse(include_bytes!("../test/v2/data1.json"));

        assert_eq!("foo", uplink.app_id);
        assert_eq!("device-01", uplink.dev_id);
        assert_eq!(
            json!({
              "luminosity_21": 5
            }),
            uplink.payload_fields
        )
    }

    #[test]
    pub fn simulation() {
        let uplink: Uplink = parse(include_bytes!("../test/v2/simulation.json"));

        assert_eq!("foo", uplink.app_id);
        assert_eq!("device_id", uplink.dev_id);
    }
}
