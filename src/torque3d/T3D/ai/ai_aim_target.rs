pub trait AIAimTarget: AIInfo
{
    type Parent: AIInfo;
    type mAimOffset: Point3F;
}