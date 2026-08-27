use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::lnprintr;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() { //initialize interrupt descriptor table
    unsafe { //this is not a good way to do it but it works probably
        IDT.breakpoint.set_handler_fn(breakpointHandler);
        IDT.load(); //load interrupt descriptor table
    }
}

extern "x86-interrupt" fn breakpointHandler(stackFrame: InterruptStackFrame) {
    lnprintr!("EXCEPTION: BREAKPOINT\n{:#?}", stackFrame);
}