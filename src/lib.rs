mod model;

pub use model::*;

#[cfg(test)]
mod smoke_test {
    use crate::model;

    #[test]
    fn tcp() {
        static JSON: &str = include_str!("../tcp.json");
        serde_json::from_str::<model::Iperf3>(&JSON).expect("failed to parse");
    }

    #[test]
    fn tcp_parallel() {
        static JSON: &str = include_str!("../tcp-parallel.json");
        serde_json::from_str::<model::Iperf3>(&JSON).expect("failed to parse");
    }

    #[test]
    fn udp() {
        const JSON: &str = include_str!("../udp.json");
        serde_json::from_str::<model::Iperf3>(JSON).expect("failed to deserialize");
    }
}
