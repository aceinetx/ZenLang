pub trait U64BitsControl {
    // get low/high bits functions
    fn get_high(&self) -> u32;
    fn get_low(&self) -> u32;

    // set low/high bits functions
    fn set_high(&mut self, n: u32);
    fn set_low(&mut self, n: u32);
    fn set_high_low(&mut self, high: u32, low: u32);

    // low/high bits operation functions
    fn add_low(&mut self, n: u32);
    fn add_high(&mut self, n: u32);

    fn sub_low(&mut self, n: u32);
    fn sub_high(&mut self, n: u32);
}

impl U64BitsControl for u64 {
    // get low/high bits functions
    fn get_high(&self) -> u32 {
        return (*self >> 32) as u32;
    }
    fn get_low(&self) -> u32 {
        return *self as u32;
    }

    // set low/high bits functions
    fn set_high(&mut self, n: u32) {
        self.set_high_low(n, self.get_low());
    }
    fn set_low(&mut self, n: u32) {
        self.set_high_low(self.get_high(), n);
    }
    fn set_high_low(&mut self, high: u32, low: u32) {
        *self = ((high as u64) << 32) | (low as u64);
    }

    // low/high bits operation functions
    fn add_low(&mut self, n: u32) {
        self.set_low(self.get_low().wrapping_add(n));
    }
    fn add_high(&mut self, n: u32) {
        self.set_high(self.get_high().wrapping_add(n));
    }

    fn sub_low(&mut self, n: u32) {
        self.set_low(self.get_low().wrapping_sub(n));
    }
    fn sub_high(&mut self, n: u32) {
        self.set_high(self.get_high().wrapping_sub(n));
    }
}
