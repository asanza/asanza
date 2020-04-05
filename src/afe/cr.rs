#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x0040_fa00"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0040_fa00
    }
}
#[doc = "Reader of field `STRTUP_CNT`"]
pub type STRTUP_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STRTUP_CNT`"]
pub struct STRTUP_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRTUP_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Result Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULT_FORMAT_A {
    #[doc = "0: Left justified 2's complement 32-bit : SVVVVVVVVVVVVVVVVVVVVVVV00000000 where (S= sign bit , V=valid result value, 0=zero)"]
    _0 = 0,
    #[doc = "1: Right justified 2's complement 32-bit : SSSSSSSSSVVVVVVVVVVVVVVVVVVVVVVV where (S= sign bit , V= valid result value, 0= zero)"]
    _1 = 1,
}
impl From<RESULT_FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: RESULT_FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESULT_FORMAT`"]
pub type RESULT_FORMAT_R = crate::R<bool, RESULT_FORMAT_A>;
impl RESULT_FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULT_FORMAT_A {
        match self.bits {
            false => RESULT_FORMAT_A::_0,
            true => RESULT_FORMAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESULT_FORMAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESULT_FORMAT_A::_1
    }
}
#[doc = "Write proxy for field `RESULT_FORMAT`"]
pub struct RESULT_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESULT_FORMAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Left justified 2's complement 32-bit : SVVVVVVVVVVVVVVVVVVVVVVV00000000 where (S= sign bit , V=valid result value, 0=zero)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESULT_FORMAT_A::_0)
    }
    #[doc = "Right justified 2's complement 32-bit : SSSSSSSSSVVVVVVVVVVVVVVVVVVVVVVV where (S= sign bit , V= valid result value, 0= zero)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESULT_FORMAT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `DLY_OK`"]
pub struct DLY_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_OK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Software Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_B_A {
    #[doc = "0: All ADCs, PGAs and Decimation filters are disabled. Clock Configuration bits will be reset."]
    _0 = 0,
    #[doc = "1: .= All ADCs, PGAs and Decimation filters are enabled."]
    _1 = 1,
}
impl From<RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: RST_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RST_B`"]
pub type RST_B_R = crate::R<bool, RST_B_A>;
impl RST_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_B_A {
        match self.bits {
            false => RST_B_A::_0,
            true => RST_B_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RST_B_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RST_B_A::_1
    }
}
#[doc = "Write proxy for field `RST_B`"]
pub struct RST_B_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All ADCs, PGAs and Decimation filters are disabled. Clock Configuration bits will be reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RST_B_A::_0)
    }
    #[doc = ".= All ADCs, PGAs and Decimation filters are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RST_B_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Low power Mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_EN_A {
    #[doc = "0: AFE will be in normal mode"]
    _0 = 0,
    #[doc = "1: AFE will be in low power mode. Setting this bit reduce the current consumption of ADC and Buffer Amplifier , the max modulator clock frequency is below 1Mhz."]
    _1 = 1,
}
impl From<LPM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPM_EN`"]
pub type LPM_EN_R = crate::R<bool, LPM_EN_A>;
impl LPM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_EN_A {
        match self.bits {
            false => LPM_EN_A::_0,
            true => LPM_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPM_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPM_EN_A::_1
    }
}
#[doc = "Write proxy for field `LPM_EN`"]
pub struct LPM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AFE will be in normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPM_EN_A::_0)
    }
    #[doc = "AFE will be in low power mode. Setting this bit reduce the current consumption of ADC and Buffer Amplifier , the max modulator clock frequency is below 1Mhz."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPM_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `SOFT_TRG3`"]
pub struct SOFT_TRG3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_TRG3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write proxy for field `SOFT_TRG2`"]
pub struct SOFT_TRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_TRG2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `SOFT_TRG1`"]
pub struct SOFT_TRG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_TRG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `SOFT_TRG0`"]
pub struct SOFT_TRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_TRG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "AFE Master Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_EN_A {
    #[doc = "0: All ADCs are disabled."]
    _0 = 0,
    #[doc = "1: All ADCs and filters will get simultaneously enabled ."]
    _1 = 1,
}
impl From<MSTR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTR_EN`"]
pub type MSTR_EN_R = crate::R<bool, MSTR_EN_A>;
impl MSTR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_EN_A {
        match self.bits {
            false => MSTR_EN_A::_0,
            true => MSTR_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_EN_A::_1
    }
}
#[doc = "Write proxy for field `MSTR_EN`"]
pub struct MSTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All ADCs are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_EN_A::_0)
    }
    #[doc = "All ADCs and filters will get simultaneously enabled ."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:15 - Start up count"]
    #[inline(always)]
    pub fn strtup_cnt(&self) -> STRTUP_CNT_R {
        STRTUP_CNT_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 18 - Result Format"]
    #[inline(always)]
    pub fn result_format(&self) -> RESULT_FORMAT_R {
        RESULT_FORMAT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Software Reset"]
    #[inline(always)]
    pub fn rst_b(&self) -> RST_B_R {
        RST_B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Low power Mode enable"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AFE Master Enable"]
    #[inline(always)]
    pub fn mstr_en(&self) -> MSTR_EN_R {
        MSTR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:15 - Start up count"]
    #[inline(always)]
    pub fn strtup_cnt(&mut self) -> STRTUP_CNT_W {
        STRTUP_CNT_W { w: self }
    }
    #[doc = "Bit 18 - Result Format"]
    #[inline(always)]
    pub fn result_format(&mut self) -> RESULT_FORMAT_W {
        RESULT_FORMAT_W { w: self }
    }
    #[doc = "Bit 21 - Delay OK"]
    #[inline(always)]
    pub fn dly_ok(&mut self) -> DLY_OK_W {
        DLY_OK_W { w: self }
    }
    #[doc = "Bit 22 - Software Reset"]
    #[inline(always)]
    pub fn rst_b(&mut self) -> RST_B_W {
        RST_B_W { w: self }
    }
    #[doc = "Bit 25 - Low power Mode enable"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LPM_EN_W {
        LPM_EN_W { w: self }
    }
    #[doc = "Bit 27 - Software Trigger3"]
    #[inline(always)]
    pub fn soft_trg3(&mut self) -> SOFT_TRG3_W {
        SOFT_TRG3_W { w: self }
    }
    #[doc = "Bit 28 - Software Trigger2"]
    #[inline(always)]
    pub fn soft_trg2(&mut self) -> SOFT_TRG2_W {
        SOFT_TRG2_W { w: self }
    }
    #[doc = "Bit 29 - Software Trigger1"]
    #[inline(always)]
    pub fn soft_trg1(&mut self) -> SOFT_TRG1_W {
        SOFT_TRG1_W { w: self }
    }
    #[doc = "Bit 30 - Software Trigger0"]
    #[inline(always)]
    pub fn soft_trg0(&mut self) -> SOFT_TRG0_W {
        SOFT_TRG0_W { w: self }
    }
    #[doc = "Bit 31 - AFE Master Enable"]
    #[inline(always)]
    pub fn mstr_en(&mut self) -> MSTR_EN_W {
        MSTR_EN_W { w: self }
    }
}
