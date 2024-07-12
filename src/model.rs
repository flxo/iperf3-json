use std::net::IpAddr;
use std::time::Duration;

use serde::Deserialize;
use serde_with::serde_as;
use serde_with::DurationSecondsWithFrac;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iperf3 {
    pub start: Start,
    pub intervals: Vec<Interval>,
    pub end: End,
    pub error: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Start {
    pub connected: Vec<Connected>,
    pub version: String,
    #[serde(rename = "system_info")]
    pub system_info: String,
    pub timestamp: Timestamp,
    #[serde(rename = "connecting_to")]
    pub connecting_to: ConnectingTo,
    pub cookie: String,
    #[serde(rename = "tcp_mss_default")]
    pub tcp_mss_default: u64,
    #[serde(rename = "target_bitrate")]
    pub target_bitrate: u64,
    #[serde(rename = "sock_bufsize")]
    pub sock_bufsize: u64,
    #[serde(rename = "sndbuf_actual")]
    pub sndbuf_actual: u64,
    #[serde(rename = "rcvbuf_actual")]
    pub rcvbuf_actual: u64,
    #[serde(rename = "test_start")]
    pub test_start: TestStart,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connected {
    pub socket: i32,
    #[serde(rename = "local_host")]
    pub local_host: IpAddr,
    #[serde(rename = "local_port")]
    pub local_port: u16,
    #[serde(rename = "remote_host")]
    pub remote_host: IpAddr,
    #[serde(rename = "remote_port")]
    pub remote_port: u16,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    pub time: String,
    pub timesecs: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectingTo {
    // TODO: Slit off the interface in case of ipv6
    pub host: String,
    pub port: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStart {
    pub protocol: String,
    #[serde(rename = "num_streams")]
    pub num_streams: i64,
    pub blksize: i64,
    pub omit: i64,
    pub duration: i64,
    pub bytes: i64,
    pub blocks: i64,
    pub reverse: i64,
    pub tos: i64,
    #[serde(rename = "target_bitrate")]
    pub target_bitrate: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub streams: Vec<Stream>,
    pub sum: Sum,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub socket: i32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub retransmits: u64,
    #[serde(rename = "snd_cwnd")]
    pub snd_cwnd: u64,
    #[serde(rename = "snd_wnd")]
    pub snd_wnd: u64,
    pub rtt: u64,
    pub rttvar: u64,
    pub pmtu: u64,
    pub omitted: bool,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sum {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: u64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub retransmits: u64,
    pub omitted: bool,
    pub sender: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct End {
    pub streams: Vec<EndStream>,
    #[serde(rename = "sum_sent")]
    pub sum_sent: SumSent,
    #[serde(rename = "sum_received")]
    pub sum_received: SumReceived,
    #[serde(rename = "cpu_utilization_percent")]
    pub cpu_utilization_percent: CpuUtilizationPercent,
    #[serde(rename = "sender_tcp_congestion")]
    pub sender_tcp_congestion: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndStream {
    pub sender: Sender,
    pub receiver: Receiver,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub socket: i32,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub retransmits: u64,
    #[serde(rename = "max_snd_cwnd")]
    pub max_snd_cwnd: u64,
    #[serde(rename = "max_snd_wnd")]
    pub max_snd_wnd: u64,
    #[serde(rename = "max_rtt")]
    pub max_rtt: u64,
    #[serde(rename = "min_rtt")]
    pub min_rtt: u64,
    #[serde(rename = "mean_rtt")]
    pub mean_rtt: u64,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receiver {
    pub socket: i64,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: i64,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumSent {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: f64,
    pub retransmits: i64,
    pub sender: bool,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SumReceived {
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub start: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub end: Duration,
    #[serde_as(as = "DurationSecondsWithFrac<f64>")]
    pub seconds: Duration,
    pub bytes: i64,
    #[serde(rename = "bits_per_second")]
    pub bits_per_second: i64,
    pub sender: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuUtilizationPercent {
    #[serde(rename = "host_total")]
    pub host_total: f64,
    #[serde(rename = "host_user")]
    pub host_user: f64,
    #[serde(rename = "host_system")]
    pub host_system: f64,
    #[serde(rename = "remote_total")]
    pub remote_total: i64,
    #[serde(rename = "remote_user")]
    pub remote_user: i64,
    #[serde(rename = "remote_system")]
    pub remote_system: i64,
}
