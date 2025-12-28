use super::ai_info::AIInfo;
use crate::util::Point3F;

pub trait AIAimTarget: AIInfo
{
    type Parent: AIInfo;

    fn get_aim_offset() -> Point3F;
    fn set_aim_offset();
    fn is_target_los()  -> bool;
    fn get_position()   -> Point3F;

}