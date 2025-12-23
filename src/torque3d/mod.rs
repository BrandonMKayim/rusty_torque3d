pub mod platform;

use platform::ProcessorType;

pub struct TorqueEngine
{
    memory: Option<TorqueMemory>,
    processor_manager: ProcessorType
}

pub struct TorqueMemory;