/* Global processor identifiers. */
pub enum ProcessorType
{
    CPUx86compatible,
    CPUArmCompatible,
    CPUIntel,
    CPUAMD,
    CPUAPPLE
}

/* Properties for CPU. */
pub enum ProcessorProperties
{
    /// We should use C fallback math functions.
    CPUPropC         = (1<<0), 
    /// Has an FPU. (It better!)
    CPUPropFPU       = (1<<1),  
    /// Supports MMX instruction set extension.
    CPUPropMMX       = (1<<2),  
    /// Supports SSE instruction set extension.
    CPUPropSSE       = (1<<3),  
    /// Supports SSE2 instruction set extension.
    CPUPropSSE2      = (1<<4),  
    /// Supports SSE3 instruction set extension.  
    CPUPropSSE3      = (1<<5),  
    /// Supports Supplemental SSE3 instruction set  
    CPUPropSSE3ex    = (1<<6),  
    /// Supports SSE4_1 instruction set extension.  
    CPUPropSSE4_1    = (1<<7),  
    /// Supports SSE4_2 instruction set extension.
    CPUPropSSE4_2    = (1<<8),  
    /// Supports AVX256 instruction set extension.
    CPUPropAVX       = (1<<9),  
    /// This is a multi-processor system.
    CPUPropMP        = (1<<10), 
    /// This processor is LITTLE ENDIAN.
    CPUPropLE        = (1<<11), 
    /// This processor is 64-bit capable
    CPUProp64bit     = (1<<12), 
    /// Supports the Arm Neon instruction set extension.
    CPUPropNEON      = (1<<13)
} 

pub trait Processor
{
    fn init();
}