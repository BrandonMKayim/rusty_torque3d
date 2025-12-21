pub mod platform;

use platform::ProcessorType;

pub struct TorqueEngine
{
    memory: Option<TorqueMemory>,
    processor_manager: ProcessorType
}

impl TorqueEngine
{
    pub fn init() -> Self
    {
        TorqueEngine {  }
    }
}

pub struct TorqueMemory;