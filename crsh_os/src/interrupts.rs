use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::lnprintr;

pub fn init_idt() { //initialize interrupt descriptor table
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpointHandler);
    idt.load(); //load interrupt descriptor table
}

extern "x86-interrupt" fn breakpointHandler(stackFrame: InterruptStackFrame) {
    lnprintr!("EXCEPTION: BREAKPOINT\n{:#?}", stackFrame);
}