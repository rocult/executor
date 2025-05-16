use vtable_rs::vtable;

#[vtable]
pub trait BytecodeEncoderVmt {
    fn destructor(&mut self) {}
    fn encode(&self, data: *mut u32, count: usize);
}
