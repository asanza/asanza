#[doc = "Reader of register FILTER2_CFG"]
pub type R = crate::R<u16, super::FILTER2_CFG>;
#[doc = "Writer for register FILTER2_CFG"]
pub type W = crate::W<u16, super::FILTER2_CFG>;
#[doc = "Register FILTER2_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTER2_CFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tamper Detect Bit 2 Filter Duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIL_DUR2_A {
    #[doc = "0: Filtering operation disabled."]
    _0 = 0,
}
impl From<FIL_DUR2_A> for u8 {
    #[inline(always)]
    fn from(variant: FIL_DUR2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIL_DUR2`"]
pub type FIL_DUR2_R = crate::R<u8, FIL_DUR2_A>;
impl FIL_DUR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIL_DUR2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIL_DUR2_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIL_DUR2_A::_0
    }
}
#[doc = "Write proxy for field `FIL_DUR2`"]
pub struct FIL_DUR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FIL_DUR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIL_DUR2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Filtering operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIL_DUR2_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Tamper Filter 2 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL2_A {
    #[doc = "0: 32 kHz clock"]
    _000 = 0,
    #[doc = "1: 512 Hz clock"]
    _001 = 1,
    #[doc = "2: 128 Hz clock"]
    _010 = 2,
    #[doc = "3: 64 Hz clock"]
    _011 = 3,
    #[doc = "4: 16 Hz clock"]
    _100 = 4,
    #[doc = "5: 8 Hz clock"]
    _101 = 5,
    #[doc = "6: 4 Hz clock"]
    _110 = 6,
    #[doc = "7: 2 Hz clock"]
    _111 = 7,
}
impl From<CLK_SEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL2`"]
pub type CLK_SEL2_R = crate::R<u8, CLK_SEL2_A>;
impl CLK_SEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL2_A {
        match self.bits {
            0 => CLK_SEL2_A::_000,
            1 => CLK_SEL2_A::_001,
            2 => CLK_SEL2_A::_010,
            3 => CLK_SEL2_A::_011,
            4 => CLK_SEL2_A::_100,
            5 => CLK_SEL2_A::_101,
            6 => CLK_SEL2_A::_110,
            7 => CLK_SEL2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLK_SEL2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLK_SEL2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLK_SEL2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLK_SEL2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLK_SEL2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLK_SEL2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLK_SEL2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLK_SEL2_A::_111
    }
}
#[doc = "Write proxy for field `CLK_SEL2`"]
pub struct CLK_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_000)
    }
    #[doc = "512 Hz clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_001)
    }
    #[doc = "128 Hz clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_010)
    }
    #[doc = "64 Hz clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_011)
    }
    #[doc = "16 Hz clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_100)
    }
    #[doc = "8 Hz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_101)
    }
    #[doc = "4 Hz clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_110)
    }
    #[doc = "2 Hz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLK_SEL2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
#[doc = "Tamper Detect Input Bit 2 Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
    #[doc = "0: Tamper detect input bit 2 is active high."]
    _0 = 0,
    #[doc = "1: Tamper detect input bit 2 is active low."]
    _1 = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, POL2_A>;
impl POL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::_0,
            true => POL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL2_A::_1
    }
}
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper detect input bit 2 is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL2_A::_0)
    }
    #[doc = "Tamper detect input bit 2 is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Tamper Detect Bit 2 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur2(&self) -> FIL_DUR2_R {
        FIL_DUR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Tamper Filter 2 Clock Select"]
    #[inline(always)]
    pub fn clk_sel2(&self) -> CLK_SEL2_R {
        CLK_SEL2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Tamper Detect Input Bit 2 Polarity Control"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11 - Tamper Detect Bit 2 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur2(&mut self) -> FIL_DUR2_W {
        FIL_DUR2_W { w: self }
    }
    #[doc = "Bits 12:14 - Tamper Filter 2 Clock Select"]
    #[inline(always)]
    pub fn clk_sel2(&mut self) -> CLK_SEL2_W {
        CLK_SEL2_W { w: self }
    }
    #[doc = "Bit 15 - Tamper Detect Input Bit 2 Polarity Control"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
}
