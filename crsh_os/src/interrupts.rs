use x86_64::structures::idt::InterruptDescriptorTable;
use crate::lnprintr;

pub fn init_idt() { //initialize interrupt descriptor table
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpointHandler);
}

extern "x86-interrupt" fn breakpointHandler(stackFrame: InterruptStackFrame) {
    lnprintr!("EXCEPTION: BREAKPOINT\n{:#?}", stackFrame);
}