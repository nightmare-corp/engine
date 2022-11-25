//TODO this uses println! replace by ne::log!()  .... someday
//only works on x86 and x86_64
#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;
#[cfg(target_arch = "x86_64")]

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!(
    "ne_bench::timer doesn't work on your architecture, only on x86_64 and x86. Sorry!~"
);

/// warning: only works on x86 and x86_64
/// minimalistic tool to measure how many cpu cycles have passed
///  
/// it is reccomened you measure the cost of this tool itself:
///     ``let timer = Timer::new();``
///     ``timer.end(); // prints 340? ``
pub struct Timer(u64);
impl Timer { 
    pub fn new() -> Self {Self {0: now()}}
    ///simply returns the duration and lives
    pub fn duration(&self) -> u64 {
        now() - self.0
    }
    ///simply returns the duration and dies.
    pub fn duration_end(self) -> u64 {
        self.duration()
    }
    ///simply prints the number and lives.
    pub fn no_end(&self) {
        println!("{}", self.duration());
    }
    ///simply prints the number and dies.
    pub fn end(self) {
        self.no_end()
    }
}
///get current cycles
fn now() -> u64 {
    let now;
    unsafe {
        now = arch::_rdtsc();
    }
    now
}