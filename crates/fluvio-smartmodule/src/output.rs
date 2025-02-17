use fluvio_protocol::{Encoder, Decoder, record::Record, link::smartmodule::SmartModuleRuntimeError};

/// A type used to return processed records and/or an error from a SmartModule
#[derive(Debug, Default, Encoder, Decoder)]
pub struct SmartModuleOutput {
    /// The successfully processed output Records
    pub successes: Vec<Record>,
    /// Any runtime error if one was encountered
    pub error: Option<SmartModuleRuntimeError>,
}

/// A type used to return processed records and/or an error from an Aggregate SmartModule
#[derive(Debug, Default, Encoder, Decoder)]
pub struct SmartModuleAggregateOutput {
    /// The base output required by all SmartModules
    pub base: SmartModuleOutput,
    #[fluvio(min_version = 16)]
    pub accumulator: Vec<u8>,
}
