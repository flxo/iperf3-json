use std::net::IpAddr;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DurationSecondsWithFrac;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Iperf3 {
    pub start: Start,
    pub intervals: Vec<Interval>,
    pub end: End,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Start {
    pub connected: Vec<Connected>,
    pub version: String,
    pub system_info: String,
    pub timestamp: Timestamp,
    pub connecting_to: ConnectingTo,
    pub cookie: String,
    pub tcp_mss_default: Option<u64>,
    pub target_bitrate: u64,
    pub test_start: TestStart,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Connected {
    pub socket: u32,
    pub local_host: IpAddr,
    pub local_port: u16,
    pub remote_host: IpAddr,
    pub remote_port: u16,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Timestamp {
    pub time: String,
    pub timesecs: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectingTo {
    // TODO: Slit off the interface in case of ipv6
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TestStart {
    pub protocol: String,
    pub num_streams: u64,
    pub blksize: u64,
    pub omit: u64,
    pub duration: i64,
    pub bytes: u64,
    pub blocks: u64,
    pub reverse: u64,
    pub tos: u64,
    pub target_bitrate: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Interval {
    pub streams: Vec<Stream>,
    pub sum: Sum,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Stream {
    pub socket: u32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: Option<u64>,
    pub snd_cwnd: Option<u64>,
    pub snd_wnd: Option<u64>,
    pub rtt: Option<u64>,
    pub rttvar: Option<u64>,
    pub pmtu: Option<u64>,
    pub omitted: bool,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sum {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: Option<u64>,
    pub omitted: bool,
    pub sender: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct End {
    pub streams: Vec<EndStream>,
    pub sum_sent: SumSent,
    pub sum_received: SumReceived,
    pub cpu_utilization_percent: CpuUtilizationPercent,
    pub sender_tcp_congestion: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndStream {
    pub sender: Option<Sender>,
    pub receiver: Option<Receiver>,
    pub udp: Option<Udp>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Sender {
    pub socket: u32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: i64,
    pub bits_per_second: f64,
    pub retransmits: Option<u64>,
    pub max_snd_cwnd: u64,
    pub max_snd_wnd: u64,
    pub max_rtt: u64,
    pub min_rtt: u64,
    pub mean_rtt: u64,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Receiver {
    pub socket: u32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub sender: bool,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Udp {
    pub socket: u32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    #[serde(rename = "jitter_ms")]
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub jitter: Duration,
    pub lost_packets: u64,
    pub packets: u64,
    pub lost_percent: u32,
    pub out_of_order: u64,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SumSent {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub retransmits: Option<u64>,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SumReceived {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    pub bits_per_second: f64,
    pub sender: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CpuUtilizationPercent {
    pub host_total: f64,
    pub host_user: f64,
    pub host_system: f64,
    pub remote_total: f64,
    pub remote_user: f64,
    pub remote_system: f64,
}
