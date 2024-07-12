mod model;

pub use model::*;

#[cfg(test)]
mod smoke_test {
    use crate::model;

    static JSON: &str = include_str!("../test.json");

    #[test]
    fn parse_json() {
        serde_json::from_str::<model::Iperf3>(&JSON).expect("failed to parse");
    }
}