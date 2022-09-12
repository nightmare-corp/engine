
pub struct FPSData {
    low:f32, //1%
    index:f32,
    // lowest:u32, //.1%
}
impl Default for FPSData
{
    fn default() -> Self {
        Self { low: 100_000_000.0, 
            index: Default::default() }
    }
}
impl FPSData
{
    pub fn get_lowest(&mut self, fps:f32) -> f32
    {
        self.index+=1.0;
        //reset every 100+ frames
        if self.index>=100.0
        {
            self.index = 0.0;
            self.low = 100_000_000.0;
        }
        //set if lower
        if fps<self.low
        {
            self.low=fps;
        }
        self.low
    }
}
