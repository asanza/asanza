#[doc = "Reader of register CH1_CFR"]
pub type R = crate::R<u32, super::CH1_CFR>;
#[doc = "Writer for register CH1_CFR"]
pub type W = crate::W<u32, super::CH1_CFR>;
#[doc = "Register CH1_CFR `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::CH1_CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Hardware Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HW_TRG_A {
    #[doc = "0: Software trigger select"]
    _0 = 0,
    #[doc = "1: Hardware trigger select"]
    _1 = 1,
}
impl From<HW_TRG_A> for bool {
    #[inline(always)]
    fn from(variant: HW_TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HW_TRG`"]
pub type HW_TRG_R = crate::R<bool, HW_TRG_A>;
impl HW_TRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HW_TRG_A {
        match self.bits {
            false => HW_TRG_A::_0,
            true => HW_TRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HW_TRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HW_TRG_A::_1
    }
}
#[doc = "Write proxy for field `HW_TRG`"]
pub struct HW_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HW_TRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger select"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HW_TRG_A::_0)
    }
    #[doc = "Hardware trigger select"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HW_TRG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Decimator Clock Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEC_CLK_INP_SEL_A {
    #[doc = "0: On the chip modulator clock will be used"]
    _0 = 0,
    #[doc = "1: External clock will be used."]
    _1 = 1,
}
impl From<DEC_CLK_INP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEC_CLK_INP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEC_CLK_INP_SEL`"]
pub type DEC_CLK_INP_SEL_R = crate::R<bool, DEC_CLK_INP_SEL_A>;
impl DEC_CLK_INP_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEC_CLK_INP_SEL_A {
        match self.bits {
            false => DEC_CLK_INP_SEL_A::_0,
            true => DEC_CLK_INP_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEC_CLK_INP_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEC_CLK_INP_SEL_A::_1
    }
}
#[doc = "Write proxy for field `DEC_CLK_INP_SEL`"]
pub struct DEC_CLK_INP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_CLK_INP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEC_CLK_INP_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On the chip modulator clock will be used"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEC_CLK_INP_SEL_A::_0)
    }
    #[doc = "External clock will be used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEC_CLK_INP_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Decimator Clock Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEC_CLK_EDGE_SEL_A {
    #[doc = "0: Posedge will be used."]
    _0 = 0,
    #[doc = "1: Negedge will be used."]
    _1 = 1,
}
impl From<DEC_CLK_EDGE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DEC_CLK_EDGE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEC_CLK_EDGE_SEL`"]
pub type DEC_CLK_EDGE_SEL_R = crate::R<bool, DEC_CLK_EDGE_SEL_A>;
impl DEC_CLK_EDGE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEC_CLK_EDGE_SEL_A {
        match self.bits {
            false => DEC_CLK_EDGE_SEL_A::_0,
            true => DEC_CLK_EDGE_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEC_CLK_EDGE_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEC_CLK_EDGE_SEL_A::_1
    }
}
#[doc = "Write proxy for field `DEC_CLK_EDGE_SEL`"]
pub struct DEC_CLK_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_CLK_EDGE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEC_CLK_EDGE_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Posedge will be used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEC_CLK_EDGE_SEL_A::_0)
    }
    #[doc = "Negedge will be used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEC_CLK_EDGE_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Continuous Conversion/Single Conversion Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC_A {
    #[doc = "0: One conversion following a triggering event"]
    _0 = 0,
    #[doc = "1: Continuous conversions following a triggering event."]
    _1 = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<bool, CC_A>;
impl CC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::_0,
            true => CC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CC_A::_1
    }
}
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One conversion following a triggering event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CC_A::_0)
    }
    #[doc = "Continuous conversions following a triggering event."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Decimation Filter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEC_EN_A {
    #[doc = "0: Decimation filter is disabled."]
    _0 = 0,
    #[doc = "1: Decimation filter is enabled."]
    _1 = 1,
}
impl From<DEC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEC_EN`"]
pub type DEC_EN_R = crate::R<bool, DEC_EN_A>;
impl DEC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEC_EN_A {
        match self.bits {
            false => DEC_EN_A::_0,
            true => DEC_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEC_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEC_EN_A::_1
    }
}
#[doc = "Write proxy for field `DEC_EN`"]
pub struct DEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Decimation filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEC_EN_A::_0)
    }
    #[doc = "Decimation filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEC_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Sigma Delta Modulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_MOD_EN_A {
    #[doc = "0: SD ADC1 is disabled"]
    _0 = 0,
    #[doc = "1: SD ADC1 is enabled"]
    _1 = 1,
}
impl From<SD_MOD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SD_MOD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SD_MOD_EN`"]
pub type SD_MOD_EN_R = crate::R<bool, SD_MOD_EN_A>;
impl SD_MOD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD_MOD_EN_A {
        match self.bits {
            false => SD_MOD_EN_A::_0,
            true => SD_MOD_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SD_MOD_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SD_MOD_EN_A::_1
    }
}
#[doc = "Write proxy for field `SD_MOD_EN`"]
pub struct SD_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_MOD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_MOD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SD ADC1 is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SD_MOD_EN_A::_0)
    }
    #[doc = "SD ADC1 is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SD_MOD_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "AFE Channel1 bypass mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYP_MODE_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Bypass mode where ADC and PGA of channel1 are disabled."]
    _1 = 1,
}
impl From<BYP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BYP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYP_MODE`"]
pub type BYP_MODE_R = crate::R<bool, BYP_MODE_A>;
impl BYP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYP_MODE_A {
        match self.bits {
            false => BYP_MODE_A::_0,
            true => BYP_MODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BYP_MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BYP_MODE_A::_1
    }
}
#[doc = "Write proxy for field `BYP_MODE`"]
pub struct BYP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYP_MODE_A::_0)
    }
    #[doc = "Bypass mode where ADC and PGA of channel1 are disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYP_MODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "PGA Gain Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGA_GAIN_SEL_A {
    #[doc = "1: 1x (default)"]
    _001 = 1,
    #[doc = "2: 2x"]
    _010 = 2,
    #[doc = "3: 4x"]
    _011 = 3,
    #[doc = "4: 8x"]
    _100 = 4,
    #[doc = "5: 16x"]
    _101 = 5,
    #[doc = "6: 32x"]
    _110 = 6,
}
impl From<PGA_GAIN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PGA_GAIN_SEL`"]
pub type PGA_GAIN_SEL_R = crate::R<u8, PGA_GAIN_SEL_A>;
impl PGA_GAIN_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PGA_GAIN_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PGA_GAIN_SEL_A::_001),
            2 => Val(PGA_GAIN_SEL_A::_010),
            3 => Val(PGA_GAIN_SEL_A::_011),
            4 => Val(PGA_GAIN_SEL_A::_100),
            5 => Val(PGA_GAIN_SEL_A::_101),
            6 => Val(PGA_GAIN_SEL_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PGA_GAIN_SEL_A::_110
    }
}
#[doc = "Write proxy for field `PGA_GAIN_SEL`"]
pub struct PGA_GAIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_GAIN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGA_GAIN_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1x (default)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_001)
    }
    #[doc = "2x"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_010)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_011)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_100)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_101)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PGA_GAIN_SEL_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "PGA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGA_EN_A {
    #[doc = "0: PGA disabled"]
    _0 = 0,
    #[doc = "1: PGA enabled"]
    _1 = 1,
}
impl From<PGA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PGA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PGA_EN`"]
pub type PGA_EN_R = crate::R<bool, PGA_EN_A>;
impl PGA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGA_EN_A {
        match self.bits {
            false => PGA_EN_A::_0,
            true => PGA_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGA_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGA_EN_A::_1
    }
}
#[doc = "Write proxy for field `PGA_EN`"]
pub struct PGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PGA disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGA_EN_A::_0)
    }
    #[doc = "PGA enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGA_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Decimator OverSampling Ratio select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEC_OSR_A {
    #[doc = "0: 64"]
    _000 = 0,
    #[doc = "1: 128"]
    _001 = 1,
    #[doc = "2: 256"]
    _010 = 2,
    #[doc = "3: 512"]
    _011 = 3,
    #[doc = "4: 1024"]
    _100 = 4,
    #[doc = "5: 2048"]
    _101 = 5,
}
impl From<DEC_OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: DEC_OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEC_OSR`"]
pub type DEC_OSR_R = crate::R<u8, DEC_OSR_A>;
impl DEC_OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEC_OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEC_OSR_A::_000),
            1 => Val(DEC_OSR_A::_001),
            2 => Val(DEC_OSR_A::_010),
            3 => Val(DEC_OSR_A::_011),
            4 => Val(DEC_OSR_A::_100),
            5 => Val(DEC_OSR_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DEC_OSR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DEC_OSR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DEC_OSR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DEC_OSR_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DEC_OSR_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DEC_OSR_A::_101
    }
}
#[doc = "Write proxy for field `DEC_OSR`"]
pub struct DEC_OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEC_OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_000)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_001)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_010)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_011)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_100)
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DEC_OSR_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Hardware Trigger Select"]
    #[inline(always)]
    pub fn hw_trg(&self) -> HW_TRG_R {
        HW_TRG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Decimator Clock Input Select"]
    #[inline(always)]
    pub fn dec_clk_inp_sel(&self) -> DEC_CLK_INP_SEL_R {
        DEC_CLK_INP_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Decimator Clock Edge Select"]
    #[inline(always)]
    pub fn dec_clk_edge_sel(&self) -> DEC_CLK_EDGE_SEL_R {
        DEC_CLK_EDGE_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Continuous Conversion/Single Conversion Mode Select"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Decimation Filter enable"]
    #[inline(always)]
    pub fn dec_en(&self) -> DEC_EN_R {
        DEC_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Sigma Delta Modulator enable"]
    #[inline(always)]
    pub fn sd_mod_en(&self) -> SD_MOD_EN_R {
        SD_MOD_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AFE Channel1 bypass mode"]
    #[inline(always)]
    pub fn byp_mode(&self) -> BYP_MODE_R {
        BYP_MODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - PGA Gain Select"]
    #[inline(always)]
    pub fn pga_gain_sel(&self) -> PGA_GAIN_SEL_R {
        PGA_GAIN_SEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 24 - PGA enable"]
    #[inline(always)]
    pub fn pga_en(&self) -> PGA_EN_R {
        PGA_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - Decimator OverSampling Ratio select"]
    #[inline(always)]
    pub fn dec_osr(&self) -> DEC_OSR_R {
        DEC_OSR_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - Hardware Trigger Select"]
    #[inline(always)]
    pub fn hw_trg(&mut self) -> HW_TRG_W {
        HW_TRG_W { w: self }
    }
    #[doc = "Bit 10 - Decimator Clock Input Select"]
    #[inline(always)]
    pub fn dec_clk_inp_sel(&mut self) -> DEC_CLK_INP_SEL_W {
        DEC_CLK_INP_SEL_W { w: self }
    }
    #[doc = "Bit 11 - Decimator Clock Edge Select"]
    #[inline(always)]
    pub fn dec_clk_edge_sel(&mut self) -> DEC_CLK_EDGE_SEL_W {
        DEC_CLK_EDGE_SEL_W { w: self }
    }
    #[doc = "Bit 12 - Continuous Conversion/Single Conversion Mode Select"]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    #[doc = "Bit 13 - Decimation Filter enable"]
    #[inline(always)]
    pub fn dec_en(&mut self) -> DEC_EN_W {
        DEC_EN_W { w: self }
    }
    #[doc = "Bit 14 - Sigma Delta Modulator enable"]
    #[inline(always)]
    pub fn sd_mod_en(&mut self) -> SD_MOD_EN_W {
        SD_MOD_EN_W { w: self }
    }
    #[doc = "Bit 17 - AFE Channel1 bypass mode"]
    #[inline(always)]
    pub fn byp_mode(&mut self) -> BYP_MODE_W {
        BYP_MODE_W { w: self }
    }
    #[doc = "Bits 19:21 - PGA Gain Select"]
    #[inline(always)]
    pub fn pga_gain_sel(&mut self) -> PGA_GAIN_SEL_W {
        PGA_GAIN_SEL_W { w: self }
    }
    #[doc = "Bit 24 - PGA enable"]
    #[inline(always)]
    pub fn pga_en(&mut self) -> PGA_EN_W {
        PGA_EN_W { w: self }
    }
    #[doc = "Bits 29:31 - Decimator OverSampling Ratio select"]
    #[inline(always)]
    pub fn dec_osr(&mut self) -> DEC_OSR_W {
        DEC_OSR_W { w: self }
    }
}
