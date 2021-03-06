///! 用于封装基本的寄存器操作指令
/// 向CR0寄存器写入64位数据
#[inline]
pub unsafe fn write_cr0(value: u64) {
    asm!("mov $0, %cr0" ::"r"(value):"memory")
}

/// 向CR0寄存器读取64位数据
#[inline]
pub unsafe fn read_cr0() -> u64 {
    let mut value: u64 = 0;
    asm!("mov %cr0, $0" :"=r"(value));
    value
}

/// 向CR2寄存器写入64位数据
#[inline]
pub unsafe fn write_cr2(value: u64) {
    asm!("mov $0, %cr2" ::"r"(value):"memory")
}

/// 向CR2寄存器读取64位数据
#[inline]
pub unsafe fn read_cr2() -> u64 {
    let mut value: u64 = 0;
    asm!("mov %cr2, $0" :"=r"(value));
    value
}

/// 向CR3寄存器写入64位数据
#[inline]
pub unsafe fn write_cr3(value: u64) {
    asm!("mov $0, %cr3" ::"r"(value):"memory")
}

/// 向CR3寄存器读取64位数据
#[inline]
pub unsafe fn read_cr3() -> u64 {
    let mut value: u64 = 0;
    asm!("mov %cr3, $0" :"=r"(value));
    value
}

/// 从msr寄存器中读取64位数据
pub unsafe fn read(data: u32) -> u64 {
    let (high, low): (u32, u32);
    asm!("rdmsr" : "={eax}" (low), "={edx}" (high) : "{ecx}" (data) : "memory" : "volatile");
    ((high as u64) << 32) | (low as u64)
}

/// 从msr寄存器中写入64位数据
pub unsafe fn write(data: &mut u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!("wrmsr" :: "{ecx}" (data), "{eax}" (low), "{edx}" (high) : "memory" : "volatile" );
}


/// 从当前RFLAGS 寄存器中读取64位数据
pub unsafe fn read_rflags() -> u64 {
    let mut r: u64 = 0;
    asm!("pushfq; popq $0" : "=r"(r) :: "memory");
    r
}

/// 将`val`值写入rflags寄存器
pub unsafe fn write_raw(val: u64) {
    asm!("pushq $0; popfq" :: "r"(val) : "memory" "flags");
}

/// 获取当前的RIP指针的值
#[inline(always)]
pub fn read_rip() -> u64 {
    let mut rip: u64 = 0;
    unsafe {
        asm!(
            "lea (%rip), $0"
            : "=r"(rip) ::: "volatile"
        );
    }
    rip
}
