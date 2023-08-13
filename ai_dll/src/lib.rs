use std::ptr::null;

// use consts::MJPI;

mod consts;

pub struct MahjongAIState {
    te_cnt: [u32; 34],
    sute_cnt: [u32; 34],
    kyoku: u32,
    cha: u32,
    kaze: u32,
    tsumohai: i32,
}

#[cfg(windows)]
type MJSendMessage = extern "stdcall" fn(*const MahjongAIState, u32, u32, u32);

static mut message_func: Option<MJSendMessage> = None;

#[cfg(windows)]
#[no_mangle]
pub extern "stdcall" fn MJPInterfaceFunc(
    inst: *mut MahjongAIState,
    message: isize,
    param1: u32,
    param2: u32,
) -> u32 {
    // use consts::MJMI;

    let name: &'static str = "test\0";
    let name_ptr = name.as_ptr();
    match MJPI::from_value(message) {
        Some(MJPI::MJPI_CREATEINSTANCE) => std::mem::size_of::<MahjongAIState>() as u32,
        Some(MJPI::MJPI_INITIALIZE) => {
            unsafe {
                message_func = Some(std::mem::transmute(param2));
            }
            0
        }
        Some(MJPI::MJPI_SUTEHAI) => {
            /*             unsafe {
                           if let Some(ptr) = message_func {
                               ptr(
                                   inst,
                                   MJMI::MJMI_FUKIDASHI as u32,
                                   "testes\0".as_ptr() as u32,
                                   0u32,
                               );
                           }
                       }
            */
            consts::MJPIR_SUTEHAI | 13
        }
        Some(MJPI::MJPI_YOURNAME) => name_ptr as u32,
        _ => 0,
    }
}
