#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Year and Month Counters Register"]
    pub yearmon: YEARMON,
    #[doc = "0x02 - RTC Days and Day-of-Week Counters Register"]
    pub days: DAYS,
    #[doc = "0x04 - RTC Hours and Minutes Counters Register"]
    pub hourmin: HOURMIN,
    #[doc = "0x06 - RTC Seconds Counters Register"]
    pub seconds: SECONDS,
    #[doc = "0x08 - RTC Year and Months Alarm Register"]
    pub alm_yearmon: ALM_YEARMON,
    #[doc = "0x0a - RTC Days Alarm Register"]
    pub alm_days: ALM_DAYS,
    #[doc = "0x0c - RTC Hours and Minutes Alarm Register"]
    pub alm_hourmin: ALM_HOURMIN,
    #[doc = "0x0e - RTC Seconds Alarm Register"]
    pub alm_seconds: ALM_SECONDS,
    #[doc = "0x10 - RTC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x12 - RTC Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - RTC Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x16 - RTC Interrupt Enable Register"]
    pub ier: IER,
    _reserved12: [u8; 8usize],
    #[doc = "0x20 - RTC General Purpose Data Register"]
    pub gp_data_reg: GP_DATA_REG,
    #[doc = "0x22 - RTC Daylight Saving Hour Register"]
    pub dst_hour: DST_HOUR,
    #[doc = "0x24 - RTC Daylight Saving Month Register"]
    pub dst_month: DST_MONTH,
    #[doc = "0x26 - RTC Daylight Saving Day Register"]
    pub dst_day: DST_DAY,
    #[doc = "0x28 - RTC Compensation Register"]
    pub compen: COMPEN,
    _reserved17: [u8; 8usize],
    #[doc = "0x32 - RTC Tamper Status and Control Register"]
    pub tamper_scr: TAMPER_SCR,
    #[doc = "0x34 - RTC Tamper 0 1 Filter Configuration Register"]
    pub filter01_cfg: FILTER01_CFG,
    #[doc = "0x36 - RTC Tamper 2 Filter Configuration Register"]
    pub filter2_cfg: FILTER2_CFG,
    _reserved20: [u8; 10usize],
    #[doc = "0x42 - RTC Control 2 Register"]
    pub ctrl2: CTRL2,
}
#[doc = "RTC Year and Month Counters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yearmon](yearmon) module"]
pub type YEARMON = crate::Reg<u16, _YEARMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YEARMON;
#[doc = "`read()` method returns [yearmon::R](yearmon::R) reader structure"]
impl crate::Readable for YEARMON {}
#[doc = "`write(|w| ..)` method takes [yearmon::W](yearmon::W) writer structure"]
impl crate::Writable for YEARMON {}
#[doc = "RTC Year and Month Counters Register"]
pub mod yearmon;
#[doc = "RTC Days and Day-of-Week Counters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [days](days) module"]
pub type DAYS = crate::Reg<u16, _DAYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAYS;
#[doc = "`read()` method returns [days::R](days::R) reader structure"]
impl crate::Readable for DAYS {}
#[doc = "`write(|w| ..)` method takes [days::W](days::W) writer structure"]
impl crate::Writable for DAYS {}
#[doc = "RTC Days and Day-of-Week Counters Register"]
pub mod days;
#[doc = "RTC Hours and Minutes Counters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hourmin](hourmin) module"]
pub type HOURMIN = crate::Reg<u16, _HOURMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOURMIN;
#[doc = "`read()` method returns [hourmin::R](hourmin::R) reader structure"]
impl crate::Readable for HOURMIN {}
#[doc = "`write(|w| ..)` method takes [hourmin::W](hourmin::W) writer structure"]
impl crate::Writable for HOURMIN {}
#[doc = "RTC Hours and Minutes Counters Register"]
pub mod hourmin;
#[doc = "RTC Seconds Counters Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seconds](seconds) module"]
pub type SECONDS = crate::Reg<u16, _SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECONDS;
#[doc = "`read()` method returns [seconds::R](seconds::R) reader structure"]
impl crate::Readable for SECONDS {}
#[doc = "`write(|w| ..)` method takes [seconds::W](seconds::W) writer structure"]
impl crate::Writable for SECONDS {}
#[doc = "RTC Seconds Counters Register"]
pub mod seconds;
#[doc = "RTC Year and Months Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm_yearmon](alm_yearmon) module"]
pub type ALM_YEARMON = crate::Reg<u16, _ALM_YEARMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM_YEARMON;
#[doc = "`read()` method returns [alm_yearmon::R](alm_yearmon::R) reader structure"]
impl crate::Readable for ALM_YEARMON {}
#[doc = "`write(|w| ..)` method takes [alm_yearmon::W](alm_yearmon::W) writer structure"]
impl crate::Writable for ALM_YEARMON {}
#[doc = "RTC Year and Months Alarm Register"]
pub mod alm_yearmon;
#[doc = "RTC Days Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm_days](alm_days) module"]
pub type ALM_DAYS = crate::Reg<u16, _ALM_DAYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM_DAYS;
#[doc = "`read()` method returns [alm_days::R](alm_days::R) reader structure"]
impl crate::Readable for ALM_DAYS {}
#[doc = "`write(|w| ..)` method takes [alm_days::W](alm_days::W) writer structure"]
impl crate::Writable for ALM_DAYS {}
#[doc = "RTC Days Alarm Register"]
pub mod alm_days;
#[doc = "RTC Hours and Minutes Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm_hourmin](alm_hourmin) module"]
pub type ALM_HOURMIN = crate::Reg<u16, _ALM_HOURMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM_HOURMIN;
#[doc = "`read()` method returns [alm_hourmin::R](alm_hourmin::R) reader structure"]
impl crate::Readable for ALM_HOURMIN {}
#[doc = "`write(|w| ..)` method takes [alm_hourmin::W](alm_hourmin::W) writer structure"]
impl crate::Writable for ALM_HOURMIN {}
#[doc = "RTC Hours and Minutes Alarm Register"]
pub mod alm_hourmin;
#[doc = "RTC Seconds Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm_seconds](alm_seconds) module"]
pub type ALM_SECONDS = crate::Reg<u16, _ALM_SECONDS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALM_SECONDS;
#[doc = "`read()` method returns [alm_seconds::R](alm_seconds::R) reader structure"]
impl crate::Readable for ALM_SECONDS {}
#[doc = "`write(|w| ..)` method takes [alm_seconds::W](alm_seconds::W) writer structure"]
impl crate::Writable for ALM_SECONDS {}
#[doc = "RTC Seconds Alarm Register"]
pub mod alm_seconds;
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "RTC Control Register"]
pub mod ctrl;
#[doc = "RTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u16, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "RTC Status Register"]
pub mod status;
#[doc = "RTC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u16, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "RTC Interrupt Status Register"]
pub mod isr;
#[doc = "RTC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u16, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "RTC Interrupt Enable Register"]
pub mod ier;
#[doc = "RTC General Purpose Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_data_reg](gp_data_reg) module"]
pub type GP_DATA_REG = crate::Reg<u16, _GP_DATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GP_DATA_REG;
#[doc = "`read()` method returns [gp_data_reg::R](gp_data_reg::R) reader structure"]
impl crate::Readable for GP_DATA_REG {}
#[doc = "`write(|w| ..)` method takes [gp_data_reg::W](gp_data_reg::W) writer structure"]
impl crate::Writable for GP_DATA_REG {}
#[doc = "RTC General Purpose Data Register"]
pub mod gp_data_reg;
#[doc = "RTC Daylight Saving Hour Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_hour](dst_hour) module"]
pub type DST_HOUR = crate::Reg<u16, _DST_HOUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST_HOUR;
#[doc = "`read()` method returns [dst_hour::R](dst_hour::R) reader structure"]
impl crate::Readable for DST_HOUR {}
#[doc = "`write(|w| ..)` method takes [dst_hour::W](dst_hour::W) writer structure"]
impl crate::Writable for DST_HOUR {}
#[doc = "RTC Daylight Saving Hour Register"]
pub mod dst_hour;
#[doc = "RTC Daylight Saving Month Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_month](dst_month) module"]
pub type DST_MONTH = crate::Reg<u16, _DST_MONTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST_MONTH;
#[doc = "`read()` method returns [dst_month::R](dst_month::R) reader structure"]
impl crate::Readable for DST_MONTH {}
#[doc = "`write(|w| ..)` method takes [dst_month::W](dst_month::W) writer structure"]
impl crate::Writable for DST_MONTH {}
#[doc = "RTC Daylight Saving Month Register"]
pub mod dst_month;
#[doc = "RTC Daylight Saving Day Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_day](dst_day) module"]
pub type DST_DAY = crate::Reg<u16, _DST_DAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DST_DAY;
#[doc = "`read()` method returns [dst_day::R](dst_day::R) reader structure"]
impl crate::Readable for DST_DAY {}
#[doc = "`write(|w| ..)` method takes [dst_day::W](dst_day::W) writer structure"]
impl crate::Writable for DST_DAY {}
#[doc = "RTC Daylight Saving Day Register"]
pub mod dst_day;
#[doc = "RTC Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compen](compen) module"]
pub type COMPEN = crate::Reg<u16, _COMPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPEN;
#[doc = "`read()` method returns [compen::R](compen::R) reader structure"]
impl crate::Readable for COMPEN {}
#[doc = "`write(|w| ..)` method takes [compen::W](compen::W) writer structure"]
impl crate::Writable for COMPEN {}
#[doc = "RTC Compensation Register"]
pub mod compen;
#[doc = "RTC Tamper Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamper_scr](tamper_scr) module"]
pub type TAMPER_SCR = crate::Reg<u16, _TAMPER_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMPER_SCR;
#[doc = "`read()` method returns [tamper_scr::R](tamper_scr::R) reader structure"]
impl crate::Readable for TAMPER_SCR {}
#[doc = "`write(|w| ..)` method takes [tamper_scr::W](tamper_scr::W) writer structure"]
impl crate::Writable for TAMPER_SCR {}
#[doc = "RTC Tamper Status and Control Register"]
pub mod tamper_scr;
#[doc = "RTC Tamper 0 1 Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter01_cfg](filter01_cfg) module"]
pub type FILTER01_CFG = crate::Reg<u16, _FILTER01_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER01_CFG;
#[doc = "`read()` method returns [filter01_cfg::R](filter01_cfg::R) reader structure"]
impl crate::Readable for FILTER01_CFG {}
#[doc = "`write(|w| ..)` method takes [filter01_cfg::W](filter01_cfg::W) writer structure"]
impl crate::Writable for FILTER01_CFG {}
#[doc = "RTC Tamper 0 1 Filter Configuration Register"]
pub mod filter01_cfg;
#[doc = "RTC Tamper 2 Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter2_cfg](filter2_cfg) module"]
pub type FILTER2_CFG = crate::Reg<u16, _FILTER2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER2_CFG;
#[doc = "`read()` method returns [filter2_cfg::R](filter2_cfg::R) reader structure"]
impl crate::Readable for FILTER2_CFG {}
#[doc = "`write(|w| ..)` method takes [filter2_cfg::W](filter2_cfg::W) writer structure"]
impl crate::Writable for FILTER2_CFG {}
#[doc = "RTC Tamper 2 Filter Configuration Register"]
pub mod filter2_cfg;
#[doc = "RTC Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u16, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "RTC Control 2 Register"]
pub mod ctrl2;
