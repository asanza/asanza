#[doc = "Reader of register STATUS"]
pub type R = crate::R<u16, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u16, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0x08"]
impl crate::ResetValue for super::STATUS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Invalidate CPU read/write access bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVAL_BIT_A {
    #[doc = "0: Time /Date Counters can be read/written. Time /Date is valid."]
    _0 = 0,
    #[doc = "1: Time /Date Counter values are changing or Time /Date is invalid and cannot be read or written."]
    _1 = 1,
}
impl From<INVAL_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: INVAL_BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVAL_BIT`"]
pub type INVAL_BIT_R = crate::R<bool, INVAL_BIT_A>;
impl INVAL_BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVAL_BIT_A {
        match self.bits {
            false => INVAL_BIT_A::_0,
            true => INVAL_BIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INVAL_BIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INVAL_BIT_A::_1
    }
}
#[doc = "Write Protect Enable status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_PROT_EN_A {
    #[doc = "0: Registers are unlocked and can be accessed."]
    _0 = 0,
    #[doc = "1: Registers are locked and in read-only mode."]
    _1 = 1,
}
impl From<WRITE_PROT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_PROT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRITE_PROT_EN`"]
pub type WRITE_PROT_EN_R = crate::R<bool, WRITE_PROT_EN_A>;
impl WRITE_PROT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_PROT_EN_A {
        match self.bits {
            false => WRITE_PROT_EN_A::_0,
            true => WRITE_PROT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRITE_PROT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRITE_PROT_EN_A::_1
    }
}
#[doc = "CPU Low Voltage Warning status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_LOW_VOLT_A {
    #[doc = "0: CPU in Normal Operating Voltage."]
    _0 = 0,
    #[doc = "1: CPU Voltage is below Normal Operating Voltage. RTC Registers in read-only mode."]
    _1 = 1,
}
impl From<CPU_LOW_VOLT_A> for bool {
    #[inline(always)]
    fn from(variant: CPU_LOW_VOLT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPU_LOW_VOLT`"]
pub type CPU_LOW_VOLT_R = crate::R<bool, CPU_LOW_VOLT_A>;
impl CPU_LOW_VOLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU_LOW_VOLT_A {
        match self.bits {
            false => CPU_LOW_VOLT_A::_0,
            true => CPU_LOW_VOLT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPU_LOW_VOLT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPU_LOW_VOLT_A::_1
    }
}
#[doc = "Reset Source bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_SRC_A {
    #[doc = "0: Part was reset due to Standby Mode Exit (that is when VDD is powered up and VBAT was not powered down at all)."]
    _0 = 0,
    #[doc = "1: Part was reset due to Power-On Reset (that is Power On Reset when both VBAT and VDD are powered up)."]
    _1 = 1,
}
impl From<RST_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: RST_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RST_SRC`"]
pub type RST_SRC_R = crate::R<bool, RST_SRC_A>;
impl RST_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_SRC_A {
        match self.bits {
            false => RST_SRC_A::_0,
            true => RST_SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RST_SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RST_SRC_A::_1
    }
}
#[doc = "Reader of field `CMP_INT`"]
pub type CMP_INT_R = crate::R<bool, bool>;
#[doc = "Write Enable bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WE_AW {
    #[doc = "2: Enable Write Protection - Registers are locked."]
    _10 = 2,
}
impl From<WE_AW> for u8 {
    #[inline(always)]
    fn from(variant: WE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `WE`"]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable Write Protection - Registers are locked."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WE_AW::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Bus Error bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERR_A {
    #[doc = "0: Read and Write accesses are normal."]
    _0 = 0,
    #[doc = "1: Read or Write accesses occurred when INVAL_BIT was asserted."]
    _1 = 1,
}
impl From<BUS_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUS_ERR`"]
pub type BUS_ERR_R = crate::R<bool, BUS_ERR_A>;
impl BUS_ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_ERR_A {
        match self.bits {
            false => BUS_ERR_A::_0,
            true => BUS_ERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUS_ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUS_ERR_A::_1
    }
}
#[doc = "Write proxy for field `BUS_ERR`"]
pub struct BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUS_ERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read and Write accesses are normal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUS_ERR_A::_0)
    }
    #[doc = "Read or Write accesses occurred when INVAL_BIT was asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUS_ERR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Compensation Done bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_DONE_A {
    #[doc = "0: Compensation busy or not enabled."]
    _0 = 0,
    #[doc = "1: Compensation completed."]
    _1 = 1,
}
impl From<CMP_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP_DONE`"]
pub type CMP_DONE_R = crate::R<bool, CMP_DONE_A>;
impl CMP_DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_DONE_A {
        match self.bits {
            false => CMP_DONE_A::_0,
            true => CMP_DONE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMP_DONE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMP_DONE_A::_1
    }
}
#[doc = "Write proxy for field `CMP_DONE`"]
pub struct CMP_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_DONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compensation busy or not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMP_DONE_A::_0)
    }
    #[doc = "Compensation completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMP_DONE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Invalidate CPU read/write access bit."]
    #[inline(always)]
    pub fn inval_bit(&self) -> INVAL_BIT_R {
        INVAL_BIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protect Enable status bit."]
    #[inline(always)]
    pub fn write_prot_en(&self) -> WRITE_PROT_EN_R {
        WRITE_PROT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU Low Voltage Warning status bit."]
    #[inline(always)]
    pub fn cpu_low_volt(&self) -> CPU_LOW_VOLT_R {
        CPU_LOW_VOLT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Source bit."]
    #[inline(always)]
    pub fn rst_src(&self) -> RST_SRC_R {
        RST_SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Compensation Interval bit."]
    #[inline(always)]
    pub fn cmp_int(&self) -> CMP_INT_R {
        CMP_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bus Error bit."]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Compensation Done bit."]
    #[inline(always)]
    pub fn cmp_done(&self) -> CMP_DONE_R {
        CMP_DONE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Write Enable bits."]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Bit 8 - Bus Error bit."]
    #[inline(always)]
    pub fn bus_err(&mut self) -> BUS_ERR_W {
        BUS_ERR_W { w: self }
    }
    #[doc = "Bit 11 - Compensation Done bit."]
    #[inline(always)]
    pub fn cmp_done(&mut self) -> CMP_DONE_W {
        CMP_DONE_W { w: self }
    }
}
