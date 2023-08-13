use proc_macros::IncrementalEnum;

pub const MJ_INTERFACE_VERSION: u32 = 12;

/* #[macro_export]
macro_rules! define_incremental_enum {
    ($name:ident, $start:expr, $($variant:ident),+ $(,)?) => {
        #[repr(u32)]
        #[allow(non_camel_case_types)]
        enum $name {
            $(
                $variant = $start + {const _: u32 = $start; _} ,
                $start += 1,
            )+
        }

        impl $name {
            pub fn from_value(value: u32) -> Option<Self> {
                match value {
                    $(
                        x if x == $name::$variant as u32 => Some($name::$variant),
                    )+
                    _ => None,
                }
            }
        }
    };
}

define_incremental_enum!(
    MJPI,
    1,
    MJPI_INITIALIZE,
    MJPI_SUTEHAI,
    MJPI_ONACTION,
    MJPI_STARTGAME,
    MJPI_STARTKYOKU,
    MJPI_ENDKYOKU,
    MJPI_ENDGAME,
    MJPI_DESTROY,
    MJPI_YOURNAME,
    MJPI_CREATEINSTANCE,
    MJPI_BASHOGIME,
    MJPI_ISEXCHANGEABLE,
    MJPI_ONEXCHANGE,
); */

/* Macro */
pub const MJPIR_SUTEHAI: u32 = 0x00000100;
pub const MJPIR_REACH: u32 = 0x00000200;
pub const MJPIR_KAN: u32 = 0x00000400;
pub const MJPIR_TSUMO: u32 = 0x00000800;
pub const MJPIR_NAGASHI: u32 = 0x00001000;
pub const MJPIR_PON: u32 = 0x00002000;
pub const MJPIR_CHII1: u32 = 0x00004000;
pub const MJPIR_CHII2: u32 = 0x00008000;
pub const MJPIR_CHII3: u32 = 0x00010000;
pub const MJPIR_MINKAN: u32 = 0x00020000;
pub const MJPIR_ANKAN: u32 = 0x00040000;
pub const MJPIR_RON: u32 = 0x00080000;

pub const MJMIR_ERROR: u32 = 0x80000000;
pub const MJR_NOTCARED: u32 = 0xffffffff;

/* RULE Macro */
#[derive(IncrementalEnum, PartialEq, Clone, Copy, Debug)]
#[base(1)]
#[incr(1)]
pub enum RULE {
    MJRL_KUITAN = 1,
    MJRL_KANSAKI,
    MJRL_PAO,
    MJRL_RON,
    MJRL_MOCHITEN,
    MJRL_BUTTOBI,
    MJRL_WAREME,
    MJRL_AKA5,
    MJRL_SHANYU,
    MJRL_SHANYU_SCORE,
    MJRL_KUINAOSHI,
    MJRL_AKA5S,
    MJRL_URADORA,
    MJRL_SCORE0REACH,
    MJRL_RYANSHIBA,
    MJRL_DORAPLUS,
    MJRL_FURITENREACH,
    MJRL_NANNYU,
    MJRL_NANNYU_SCORE,
    MJRL_KARATEN,
    MJRL_PINZUMO,
    MJRL_NOTENOYANAGARE,
    MJRL_KANINREACH,
    MJRL_TOPOYAAGARIEND,
    MJRL_77MANGAN,
    MJRL_DBLRONCHONBO,
}
