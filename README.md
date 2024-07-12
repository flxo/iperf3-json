# iperf3 json model

This is a simple model for the [iperf3](https://github.com/esnet/iperf) json
output. It can be used to parse the json output of
[iperf3](https://github.com/esnet/iperf).

```sh
iperf3 -J -c <server>
...
```

```rust
    let json: &str = ...;
    let iperf = serde_json::from_str::<iperf3_json::Iperf3>(json).expect("failed to parse");
```
