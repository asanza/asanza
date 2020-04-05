#[doc = "Reader of register FILTER01_CFG"]
pub type R = crate::R<u16, super::FILTER01_CFG>;
#[doc = "Writer for register FILTER01_CFG"]
pub type W = crate::W<u16, super::FILTER01_CFG>;
#[doc = "Register FILTER01_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTER01_CFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tamper Detect Bit 1 Filter Duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIL_DUR1_A {
    #[doc = "0: Filtering operation disabled."]
    _0 = 0,
}
impl From<FIL_DUR1_A> for u8 {
    #[inline(always)]
    fn from(variant: FIL_DUR1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIL_DUR1`"]
pub type FIL_DUR1_R = crate::R<u8, FIL_DUR1_A>;
impl FIL_DUR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIL_DUR1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIL_DUR1_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIL_DUR1_A::_0
    }
}
#[doc = "Write proxy for field `FIL_DUR1`"]
pub struct FIL_DUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIL_DUR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIL_DUR1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Filtering operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIL_DUR1_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Tamper Filter 1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL1_A {
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
impl From<CLK_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL1`"]
pub type CLK_SEL1_R = crate::R<u8, CLK_SEL1_A>;
impl CLK_SEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL1_A {
        match self.bits {
            0 => CLK_SEL1_A::_000,
            1 => CLK_SEL1_A::_001,
            2 => CLK_SEL1_A::_010,
            3 => CLK_SEL1_A::_011,
            4 => CLK_SEL1_A::_100,
            5 => CLK_SEL1_A::_101,
            6 => CLK_SEL1_A::_110,
            7 => CLK_SEL1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLK_SEL1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLK_SEL1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLK_SEL1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLK_SEL1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLK_SEL1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLK_SEL1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLK_SEL1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLK_SEL1_A::_111
    }
}
#[doc = "Write proxy for field `CLK_SEL1`"]
pub struct CLK_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_000)
    }
    #[doc = "512 Hz clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_001)
    }
    #[doc = "128 Hz clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_010)
    }
    #[doc = "64 Hz clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_011)
    }
    #[doc = "16 Hz clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_100)
    }
    #[doc = "8 Hz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_101)
    }
    #[doc = "4 Hz clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_110)
    }
    #[doc = "2 Hz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLK_SEL1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Tamper Detect Input Bit 1 Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1_A {
    #[doc = "0: Tamper detect input bit 1 is active high."]
    _0 = 0,
    #[doc = "1: Tamper detect input bit 1 is active low."]
    _1 = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, POL1_A>;
impl POL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::_0,
            true => POL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL1_A::_1
    }
}
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper detect input bit 1 is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL1_A::_0)
    }
    #[doc = "Tamper detect input bit 1 is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Tamper Detect Bit 0 Filter Duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIL_DUR0_A {
    #[doc = "0: Filtering operation disabled."]
    _0 = 0,
}
impl From<FIL_DUR0_A> for u8 {
    #[inline(always)]
    fn from(variant: FIL_DUR0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIL_DUR0`"]
pub type FIL_DUR0_R = crate::R<u8, FIL_DUR0_A>;
impl FIL_DUR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIL_DUR0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIL_DUR0_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIL_DUR0_A::_0
    }
}
#[doc = "Write proxy for field `FIL_DUR0`"]
pub struct FIL_DUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> FIL_DUR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIL_DUR0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Filtering operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIL_DUR0_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Tamper Filter 0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SEL0_A {
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
impl From<CLK_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLK_SEL0`"]
pub type CLK_SEL0_R = crate::R<u8, CLK_SEL0_A>;
impl CLK_SEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SEL0_A {
        match self.bits {
            0 => CLK_SEL0_A::_000,
            1 => CLK_SEL0_A::_001,
            2 => CLK_SEL0_A::_010,
            3 => CLK_SEL0_A::_011,
            4 => CLK_SEL0_A::_100,
            5 => CLK_SEL0_A::_101,
            6 => CLK_SEL0_A::_110,
            7 => CLK_SEL0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLK_SEL0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLK_SEL0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLK_SEL0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLK_SEL0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLK_SEL0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLK_SEL0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLK_SEL0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLK_SEL0_A::_111
    }
}
#[doc = "Write proxy for field `CLK_SEL0`"]
pub struct CLK_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 kHz clock"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_000)
    }
    #[doc = "512 Hz clock"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_001)
    }
    #[doc = "128 Hz clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_010)
    }
    #[doc = "64 Hz clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_011)
    }
    #[doc = "16 Hz clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_100)
    }
    #[doc = "8 Hz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_101)
    }
    #[doc = "4 Hz clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_110)
    }
    #[doc = "2 Hz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLK_SEL0_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
#[doc = "Tamper Detect Input Bit 0 Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0_A {
    #[doc = "0: Tamper detect input bit 0 is active high."]
    _0 = 0,
    #[doc = "1: Tamper detect input bit 0 is active low."]
    _1 = 1,
}
impl From<POL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL0`"]
pub type POL0_R = crate::R<bool, POL0_A>;
impl POL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL0_A {
        match self.bits {
            false => POL0_A::_0,
            true => POL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL0_A::_1
    }
}
#[doc = "Write proxy for field `POL0`"]
pub struct POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper detect input bit 0 is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL0_A::_0)
    }
    #[doc = "Tamper detect input bit 0 is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL0_A::_1)
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
    #[doc = "Bits 0:3 - Tamper Detect Bit 1 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur1(&self) -> FIL_DUR1_R {
        FIL_DUR1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Tamper Filter 1 Clock Select"]
    #[inline(always)]
    pub fn clk_sel1(&self) -> CLK_SEL1_R {
        CLK_SEL1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Tamper Detect Input Bit 1 Polarity Control"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Tamper Detect Bit 0 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur0(&self) -> FIL_DUR0_R {
        FIL_DUR0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Tamper Filter 0 Clock Select"]
    #[inline(always)]
    pub fn clk_sel0(&self) -> CLK_SEL0_R {
        CLK_SEL0_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Tamper Detect Input Bit 0 Polarity Control"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Tamper Detect Bit 1 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur1(&mut self) -> FIL_DUR1_W {
        FIL_DUR1_W { w: self }
    }
    #[doc = "Bits 4:6 - Tamper Filter 1 Clock Select"]
    #[inline(always)]
    pub fn clk_sel1(&mut self) -> CLK_SEL1_W {
        CLK_SEL1_W { w: self }
    }
    #[doc = "Bit 7 - Tamper Detect Input Bit 1 Polarity Control"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Bits 8:11 - Tamper Detect Bit 0 Filter Duration"]
    #[inline(always)]
    pub fn fil_dur0(&mut self) -> FIL_DUR0_W {
        FIL_DUR0_W { w: self }
    }
    #[doc = "Bits 12:14 - Tamper Filter 0 Clock Select"]
    #[inline(always)]
    pub fn clk_sel0(&mut self) -> CLK_SEL0_W {
        CLK_SEL0_W { w: self }
    }
    #[doc = "Bit 15 - Tamper Detect Input Bit 0 Polarity Control"]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W {
        POL0_W { w: self }
    }
}
