//! Mappings for APIs of *The Things Network*.

pub mod http;

#[cfg(test)]
mod test {
    use crate::http::Uplink;

    #[test]
    pub fn data1() {
        let json = include_bytes!("../test/data1.json");
        let uplink = serde_json::from_slice(json);

        println!("{:?}", uplink);

        assert!(uplink.is_ok());

        let uplink: Uplink = uplink.unwrap();

        assert_eq!("foo", uplink.app_id);
        assert_eq!("device-01", uplink.dev_id);
    }
}
