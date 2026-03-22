use riscv::register::sstatus::{self, SPP, Sstatus};
/// Trap Context
#[repr(C)]
pub struct TrapContext {
    //保存通用寄存器 x0-x31
    pub x: [usize; 32],
    //保存特权级状态寄存器
    pub sstatus: Sstatus,
    //保存异常/终断程序计数器
    pub sepc: usize,

}

impl TrapContext {
    //x[2]是sp
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    //初始化应用上下文
    pub fn app_init_context(entry: usize, sp: usize)-> Self {
        let mut sstatus = sstatus::read();
    
        sstatus.set_spp(SPP::User); //设置特权级为用户态
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            //pc设为程序入口地址
            sepc: entry,
        
        };
        cx.set_sp(sp); //设置sp
        cx//返回初始化好的应用上下文
    }

}