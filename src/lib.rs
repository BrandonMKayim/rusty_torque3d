// Visit: Torque3D.org for more.
// All rights reserved to Torque3D.org | Project created by Brandon "Visolator" Kayim.
//  A full Torque3D port from C++
//    What does this entitle?
//    - This entire project includes Torque3D's features and TorqueScript. These are tools to help you build what you need to make a game in this engine.

pub mod torque3d;

use torque3d::*;

pub type S32 = i32;

pub fn torque3d_main(argc: S32, argv: String) -> S32
{
    let engine = TorqueEngine::init();

    if(TorqueEngine::handleCommandLine(argc, argv))
    {
        return 1;
    }
    
    while TorqueEngine::tick() != -1
    {

    }
    
    // in the clear i guess
    return 0;
}