
pub enum Command{
    OneTimeHResolutionMode = 0b0010_0000,
    OneTimeHResolutionMode2 = 0b0010_0001,
    OneTimeLResolutionMode = 0b0010_0011,
    ContinuoslyHResolutionMode = 0b0001_0000,
    ContinuoslyHResolutionMode2 = 0b0001_0001,
    ContinuoslyLResolutionMode = 0b0001_0011
}