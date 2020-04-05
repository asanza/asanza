#[doc = "Reader of register CTRL"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fine compensation enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINEEN_A {
    #[doc = "1: Fine compensation is enabled."]
    _1 = 1,
    #[doc = "0: Fine compensation is disabled"]
    _0 = 0,
}
impl From<FINEEN_A> for bool {
    #[inline(always)]
    fn from(variant: FINEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FINEEN`"]
pub type FINEEN_R = crate::R<bool, FINEEN_A>;
impl FINEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINEEN_A {
        match self.bits {
            true => FINEEN_A::_1,
            false => FINEEN_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FINEEN_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FINEEN_A::_0
    }
}
#[doc = "Write proxy for field `FINEEN`"]
pub struct FINEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FINEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fine compensation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FINEEN_A::_1)
    }
    #[doc = "Fine compensation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FINEEN_A::_0)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COMP_EN`"]
pub type COMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP_EN`"]
pub struct COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Alarm Match bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALM_MATCH_A {
    #[doc = "0: Only Seconds, Minutes, and Hours matched."]
    _00 = 0,
    #[doc = "1: Only Seconds, Minutes, Hours, and Days matched."]
    _01 = 1,
    #[doc = "2: Only Seconds, Minutes, Hours, Days, and Months matched."]
    _10 = 2,
    #[doc = "3: Only Seconds, Minutes, Hours, Days, Months, and Year (offset) matched."]
    _11 = 3,
}
impl From<ALM_MATCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ALM_MATCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALM_MATCH`"]
pub type ALM_MATCH_R = crate::R<u8, ALM_MATCH_A>;
impl ALM_MATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALM_MATCH_A {
        match self.bits {
            0 => ALM_MATCH_A::_00,
            1 => ALM_MATCH_A::_01,
            2 => ALM_MATCH_A::_10,
            3 => ALM_MATCH_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ALM_MATCH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ALM_MATCH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ALM_MATCH_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ALM_MATCH_A::_11
    }
}
#[doc = "Write proxy for field `ALM_MATCH`"]
pub struct ALM_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALM_MATCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Only Seconds, Minutes, and Hours matched."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ALM_MATCH_A::_00)
    }
    #[doc = "Only Seconds, Minutes, Hours, and Days matched."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ALM_MATCH_A::_01)
    }
    #[doc = "Only Seconds, Minutes, Hours, Days, and Months matched."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ALM_MATCH_A::_10)
    }
    #[doc = "Only Seconds, Minutes, Hours, Days, Months, and Year (offset) matched."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ALM_MATCH_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Sampling timer clocks mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_STB_MASK_A {
    #[doc = "1: Sampling clocks are gated in standby mode"]
    _1 = 1,
    #[doc = "0: Sampling clocks are not gated when in standby mode"]
    _0 = 0,
}
impl From<TIMER_STB_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER_STB_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMER_STB_MASK`"]
pub type TIMER_STB_MASK_R = crate::R<bool, TIMER_STB_MASK_A>;
impl TIMER_STB_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER_STB_MASK_A {
        match self.bits {
            true => TIMER_STB_MASK_A::_1,
            false => TIMER_STB_MASK_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMER_STB_MASK_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMER_STB_MASK_A::_0
    }
}
#[doc = "Write proxy for field `TIMER_STB_MASK`"]
pub struct TIMER_STB_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_STB_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_STB_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling clocks are gated in standby mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMER_STB_MASK_A::_1)
    }
    #[doc = "Sampling clocks are not gated when in standby mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMER_STB_MASK_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Daylight Saving Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_EN_A {
    #[doc = "0: Disabled. Daylight saving changes are not applied. Daylight saving registers can be modified."]
    _0 = 0,
    #[doc = "1: Enabled. Daylight saving changes are applied."]
    _1 = 1,
}
impl From<DST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DST_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DST_EN`"]
pub type DST_EN_R = crate::R<bool, DST_EN_A>;
impl DST_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_EN_A {
        match self.bits {
            false => DST_EN_A::_0,
            true => DST_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DST_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DST_EN_A::_1
    }
}
#[doc = "Write proxy for field `DST_EN`"]
pub struct DST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Daylight saving changes are not applied. Daylight saving registers can be modified."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DST_EN_A::_0)
    }
    #[doc = "Enabled. Daylight saving changes are applied."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DST_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Software Reset bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_AW {
    #[doc = "0: Software Reset cleared."]
    _0 = 0,
    #[doc = "1: Software Reset asserted."]
    _1 = 1,
}
impl From<SWR_AW> for bool {
    #[inline(always)]
    fn from(variant: SWR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software Reset cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWR_AW::_0)
    }
    #[doc = "Software Reset asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWR_AW::_1)
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
#[doc = "RTC Clock Output Selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUT_A {
    #[doc = "0: No Output Clock"]
    _00 = 0,
    #[doc = "1: Fine 1 Hz Clock"]
    _01 = 1,
    #[doc = "2: 32.768 kHz Clock"]
    _10 = 2,
    #[doc = "3: Coarse 1 Hz Clock"]
    _11 = 3,
}
impl From<CLKOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUT`"]
pub type CLKOUT_R = crate::R<u8, CLKOUT_A>;
impl CLKOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUT_A {
        match self.bits {
            0 => CLKOUT_A::_00,
            1 => CLKOUT_A::_01,
            2 => CLKOUT_A::_10,
            3 => CLKOUT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKOUT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKOUT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKOUT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKOUT_A::_11
    }
}
#[doc = "Write proxy for field `CLKOUT`"]
pub struct CLKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Output Clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKOUT_A::_00)
    }
    #[doc = "Fine 1 Hz Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKOUT_A::_01)
    }
    #[doc = "32.768 kHz Clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKOUT_A::_10)
    }
    #[doc = "Coarse 1 Hz Clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKOUT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u16) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fine compensation enable bit"]
    #[inline(always)]
    pub fn fineen(&self) -> FINEEN_R {
        FINEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compensation enable bit 1'b0:- Coarse Compensation is disabled"]
    #[inline(always)]
    pub fn comp_en(&self) -> COMP_EN_R {
        COMP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Alarm Match bits."]
    #[inline(always)]
    pub fn alm_match(&self) -> ALM_MATCH_R {
        ALM_MATCH_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Sampling timer clocks mask"]
    #[inline(always)]
    pub fn timer_stb_mask(&self) -> TIMER_STB_MASK_R {
        TIMER_STB_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Daylight Saving Enable."]
    #[inline(always)]
    pub fn dst_en(&self) -> DST_EN_R {
        DST_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RTC Clock Output Selection."]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Fine compensation enable bit"]
    #[inline(always)]
    pub fn fineen(&mut self) -> FINEEN_W {
        FINEEN_W { w: self }
    }
    #[doc = "Bit 1 - Compensation enable bit 1'b0:- Coarse Compensation is disabled"]
    #[inline(always)]
    pub fn comp_en(&mut self) -> COMP_EN_W {
        COMP_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Alarm Match bits."]
    #[inline(always)]
    pub fn alm_match(&mut self) -> ALM_MATCH_W {
        ALM_MATCH_W { w: self }
    }
    #[doc = "Bit 4 - Sampling timer clocks mask"]
    #[inline(always)]
    pub fn timer_stb_mask(&mut self) -> TIMER_STB_MASK_W {
        TIMER_STB_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Daylight Saving Enable."]
    #[inline(always)]
    pub fn dst_en(&mut self) -> DST_EN_W {
        DST_EN_W { w: self }
    }
    #[doc = "Bit 8 - Software Reset bit."]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bits 13:14 - RTC Clock Output Selection."]
    #[inline(always)]
    pub fn clkout(&mut self) -> CLKOUT_W {
        CLKOUT_W { w: self }
    }
}
