use opentelemetry::global;
use opentelemetry_jaeger::Propagator;
use opentelemetry::sdk::propagation::{BaggagePropagator, TextMapCompositePropagator, TraceContextPropagator};

pub fn init_global_propagator(){
    global::set_text_map_propagator(composite_propagator());
}

pub fn composite_propagator() -> TextMapCompositePropagator{
    let jaeger_propagator = Propagator::new();
    let baggage_propagator = BaggagePropagator::new();
    let trace_context_propagator = TraceContextPropagator::new();
    TextMapCompositePropagator::new(vec![
        Box::new(jaeger_propagator),
        Box::new(baggage_propagator),
        Box::new(trace_context_propagator)
    ])
}