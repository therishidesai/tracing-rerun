# tracing-rerun

A simple [tracing](https://github.com/tokio-rs/tracing) `Layer` that
will send logs to a rerun `RecordingStream`. This allows you to view
tracing logs in the rerun visualizer and also save to a `.rrd` file.