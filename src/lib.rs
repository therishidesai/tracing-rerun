use rerun::{RecordingStream, TextLogLevel};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

fn to_rerun_log_level(tracing_level: &Level) -> TextLogLevel {
    match *tracing_level {
        Level::ERROR => TextLogLevel::ERROR.into(),
        Level::WARN => TextLogLevel::WARN.into(),
        Level::INFO => TextLogLevel::INFO.into(),
        Level::DEBUG => TextLogLevel::DEBUG.into(),
        Level::TRACE => TextLogLevel::TRACE.into(),
    }
}

/// Send every tracing event to Rerun as a TextLog.
pub struct RerunLayer {
    pub rec: RecordingStream,
    pub path: String, // e.g. "logs/tracing"
}

impl<S> Layer<S> for RerunLayer
where
    S: Subscriber + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        use std::fmt::Write;
        // Collect span stack (innermost -> root)
        let spans = ctx
            .lookup_current()
            .map(|s| {
                s.scope()
                    .map(|s| s.metadata().name().to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            })
            .unwrap_or_default();

        // Collect fields
        struct Visitor<'a> { buf: &'a mut String }
        impl<'a> tracing::field::Visit for Visitor<'a> {
            fn record_debug(
                &mut self,
                _field: &tracing::field::Field,
                value: &dyn std::fmt::Debug
            ) {
                let _ = write!(self.buf, "{:?} ", value);
            }
        }

        let meta = event.metadata();
        let mut msg = String::new();
        let span_prefix = if spans.is_empty() { 
            String::new() 
        } else { 
            format!("{spans}: ") 
        };
        let _ = write!(
            msg,
            "{}{}: ",
            span_prefix,
            meta.target()
        );
        let mut v = Visitor { buf: &mut msg };
        event.record(&mut v);

        // Ship to Rerun
        let text = rerun::archetypes::TextLog::new(msg).with_level(to_rerun_log_level(meta.level()));
        let _ = self.rec.log(self.path.as_str(), &text);
    }
}
