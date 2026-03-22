//! Batch subsystem: load apps one-by-one and run them in user mode.

use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use core::arch::asm;
use lazy_static::*;


//constants
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
// User program will be loaded here
const APP_BASE_ADDRESS: usize = 0x8040_0000;
const APP_SIZE_LIMIT: usize = 0x20_000;
//stacks
#[repr(align(4096))]
struct KernelStack([u8; KERNEL_STACK_SIZE]);

#[repr(align(4096))]
struct UserStack([u8; USER_STACK_SIZE]);
static KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);
static USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

impl KernelStack {
    //返回栈最高地址,self.0.as_ptr()为栈底地址
    //os只需要根据sp即可查出栈顶，不需要用函数来查
    fn top(&self) -> usize {
        self.0.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    /// Push a TrapContext onto the kernel stack 
    /// and return its address.
    pub fn push_context(&self, cx: TrapContext) -> *mut TrapContext {
        //当前系统只需要推入一个上下文（单任务）
        let ptr = (self.top() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe { *ptr = cx };
        ptr
    }
}

impl UserStack {
    fn top(&self) -> usize {
        self.0.as_ptr() as usize + USER_STACK_SIZE
    }
}

// AppManager
struct AppManager {
    num_app: usize,
    //当前活跃的app id
    current: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
impl AppManager {
    //打印调试信息
    pub fn print_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i, self.app_start[i], self.app_start[i + 1]
            
            );
        }
    }
    /// Load the app into memory at APP_BASE_ADDRESS.
    fn load_app(&self, id: usize){
        if id >= self.num_app {
            println!("All applications completed!");
            shutdown(false);
        }
        println!("[kernel] Loading app_{}", id);
        unsafe{
            //  Clear the user program area(内存中)
            core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT)
               .fill(0);
            //  找到内核镜像里嵌入的 app 二进制
            let start = self.app_start[id];
            let end = self.app_start[id + 1];
            let size = end - start;
            let src = core::slice::from_raw_parts(start as *const u8, size);
            //  将 app 二进制拷贝到用户内存
            let dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, size);
            dst.copy_from_slice(src);
            //  确保 CPU 看到更新的指令
            //让 CPU 重新从内存加载指令，
            //而不是继续执行旧的指令缓存（I-cache）里的内容
            asm!("fence.i");
        }
    }

    fn current_app(&self) -> usize {
        self.current
    }

    fn advance(&mut self) {
        self.current += 1;
    }
}
// Global App Manager
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            // _num_app is defined in link_app.S
            extern "C" {
                static _num_app: usize;
            }
            let base = &_num_app as *const usize;
            // 读取应用程序数量
            let num_app = base.read_volatile();
            // Next num_app + 1 entries = app start addresses
            let raw = core::slice::from_raw_parts(base.add(1), num_app + 1);
            //为了计算每个app的大小，最后一个地址是app_end，所以需要+1
            let mut app_start = [0usize; MAX_APP_NUM + 1];
            app_start[..=num_app].copy_from_slice(raw);
            
            AppManager {
                num_app,
                current: 0,
                app_start,
            }
        })
    };
}

// Public API
pub fn init() {
    APP_MANAGER.exclusive_access().print_info();
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_info();
}

// Load next app and switch to user mode.
pub fn run_next_app() -> ! {
    let mut mgr = APP_MANAGER.exclusive_access();
    let id = mgr.current_app();
    mgr.load_app(id);
    mgr.advance();
    drop(mgr); // Release borrow before switching to user mode
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    // Build initial user context
    //“伪造 trap 保存结果”
    let cx = TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.top());
    // Push context onto kernel stack
    let cx_ptr = KERNEL_STACK.push_context(cx);
    //从伪造的 TrapContext 恢复寄存器，然后执行 sret，直接跳到用户态
    unsafe {
        __restore(cx_ptr as usize);
    }
    unreachable!("__restore should never return");
}