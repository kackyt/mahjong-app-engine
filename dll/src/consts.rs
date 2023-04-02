use proc_macros::IncrementalEnum;

pub const MJ_INTERFACE_VERSION: u32 = 12;

/* Messages for player's interface */
#[derive(IncrementalEnum, PartialEq, Clone, Copy, Debug)]
#[base(1)]
#[incr(1)]
pub enum MJPI {
    MJPI_INITIALIZE = 1,
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
}

/* Messages for system interface */
#[derive(IncrementalEnum, PartialEq, Clone, Copy, Debug)]
#[base(1)]
#[incr(1)]
pub enum MJMI {
    MJMI_GETTEHAI = 1,
    MJMI_GETKAWA,
    MJMI_GETDORA,
    MJMI_GETSCORE,
    MJMI_GETHONBA,
    MJMI_GETREACHBOU,
    MJMI_GETRULE,
    MJMI_GETVERSION,
    MJMI_GETMACHI,
    MJMI_GETAGARITEN,
    MJMI_GETHAIREMAIN,
    MJMI_GETVISIBLEHAIS,
    MJMI_FUKIDASHI,
    MJMI_KKHAIABILITY,
    MJMI_GETWAREME,
    MJMI_SETSTRUCTTYPE,
    MJMI_SETAUTOFUKIDASHI,
    MJMI_LASTTSUMOGIRI,
    MJMI_SSPUTOABILITY,
    MJMI_GETYAKUHAN,
    MJMI_GETKYOKU,
    MJMI_GETKAWAEX,
    MJMI_ANKANABILITY,
}

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
