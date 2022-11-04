use std::env;
use http_client::isahc::IsahcClient;
use isahc::{HttpClient, prelude::Configurable, config::{VersionNegotiation, SslOption}};
use opentelemetry::{
    sdk::{
        trace::{self},
        Resource,
    },
    KeyValue,
};

mod shared;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[async_std::main]
async fn main() -> std::result::Result<(), http_types::Error>{
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    femme::with_level(femme::LevelFilter::Info);
    shared::init_global_propagator();
    let _tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("http2_client")
        .with_trace_config(trace::config().with_resource(Resource::new(tags())))
        .install_batch(opentelemetry::runtime::AsyncStd);
    let otl_mw = opentelemetry_surf::OpenTelemetryTracingMiddleware::default();
    
    //VersionNegotiation::http3(); Connect via HTTP/3
    let http_client = HttpClient::builder()
        .version_negotiation(VersionNegotiation::http2())
        .ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS | SslOption::DANGER_ACCEPT_REVOKED_CERTS | SslOption::DANGER_ACCEPT_INVALID_HOSTS)
        .metrics(true)
        .build().unwrap();
    let isahc_client = IsahcClient::from_client(http_client);
    let client = surf::Client::with_http_client(isahc_client).with(otl_mw);


    let response = client.get(url).recv_string().await?;
    println!("{}", response);

    opentelemetry::global::shutdown_tracer_provider();
    Ok(())
}

fn tags() -> Vec<KeyValue> {
    use opentelemetry_semantic_conventions::resource;

    vec![
        resource::SERVICE_VERSION.string(VERSION),
        resource::SERVICE_INSTANCE_ID.string("client-42"),
        resource::PROCESS_EXECUTABLE_PATH.string(std::env::current_exe().unwrap().display().to_string()),
        resource::PROCESS_PID.string(std::process::id().to_string()),
        // KeyValue::new("process.executable.profile", PROFILE),
    ]
}
