use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::lnprintr;
use lazy_static::lazy_static;

lazy_static! { //avoid direct unsafe code utilizing lazy_static abstractions
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpointHandler);
        idt
    };
}

pub fn init_idt() { //initialize interrupt descriptor table
    IDT.load(); //load interrupt descriptor table
}

extern "x86-interrupt" fn breakpointHandler(stackFrame: InterruptStackFrame) {
    lnprintr!("EXCEPTION: BREAKPOINT\n{:#?}", stackFrame);
}

#[test_case] //test breakpoint exception
fn testBreakpointException() {
    //invoke breakpoint exception
    x86_64::instructions::interrupts::int3();
}