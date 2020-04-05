#[doc = "Reader of register YEARMON"]
pub type R = crate::R<u16, super::YEARMON>;
#[doc = "Writer for register YEARMON"]
pub type W = crate::W<u16, super::YEARMON>;
#[doc = "Register YEARMON `reset()`'s with value 0x01"]
impl crate::ResetValue for super::YEARMON {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "These bits give the value of the Months Counter . Valid Values are:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MON_CNT_A {
    #[doc = "0: Illegal Value"]
    _0 = 0,
    #[doc = "1: January"]
    _1 = 1,
    #[doc = "2: February"]
    _10 = 2,
    #[doc = "3: March"]
    _11 = 3,
    #[doc = "4: April"]
    _100 = 4,
    #[doc = "5: May"]
    _101 = 5,
    #[doc = "6: June"]
    _110 = 6,
    #[doc = "7: July"]
    _111 = 7,
    #[doc = "8: August"]
    _1000 = 8,
    #[doc = "9: September"]
    _1001 = 9,
    #[doc = "10: October"]
    _1010 = 10,
    #[doc = "11: November"]
    _1011 = 11,
    #[doc = "12: December"]
    _1100 = 12,
    #[doc = "13: Illegal Value"]
    _1101 = 13,
    #[doc = "14: Illegal Value"]
    _1110 = 14,
    #[doc = "15: Illegal Value"]
    _1111 = 15,
}
impl From<MON_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: MON_CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MON_CNT`"]
pub type MON_CNT_R = crate::R<u8, MON_CNT_A>;
impl MON_CNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_CNT_A {
        match self.bits {
            0 => MON_CNT_A::_0,
            1 => MON_CNT_A::_1,
            2 => MON_CNT_A::_10,
            3 => MON_CNT_A::_11,
            4 => MON_CNT_A::_100,
            5 => MON_CNT_A::_101,
            6 => MON_CNT_A::_110,
            7 => MON_CNT_A::_111,
            8 => MON_CNT_A::_1000,
            9 => MON_CNT_A::_1001,
            10 => MON_CNT_A::_1010,
            11 => MON_CNT_A::_1011,
            12 => MON_CNT_A::_1100,
            13 => MON_CNT_A::_1101,
            14 => MON_CNT_A::_1110,
            15 => MON_CNT_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MON_CNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MON_CNT_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MON_CNT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MON_CNT_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MON_CNT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MON_CNT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MON_CNT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MON_CNT_A::_111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == MON_CNT_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == MON_CNT_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == MON_CNT_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == MON_CNT_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == MON_CNT_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == MON_CNT_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == MON_CNT_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == MON_CNT_A::_1111
    }
}
#[doc = "Write proxy for field `MON_CNT`"]
pub struct MON_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MON_CNT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Illegal Value"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MON_CNT_A::_0)
    }
    #[doc = "January"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1)
    }
    #[doc = "February"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MON_CNT_A::_10)
    }
    #[doc = "March"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MON_CNT_A::_11)
    }
    #[doc = "April"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MON_CNT_A::_100)
    }
    #[doc = "May"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MON_CNT_A::_101)
    }
    #[doc = "June"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MON_CNT_A::_110)
    }
    #[doc = "July"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MON_CNT_A::_111)
    }
    #[doc = "August"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1000)
    }
    #[doc = "September"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1001)
    }
    #[doc = "October"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1010)
    }
    #[doc = "November"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1011)
    }
    #[doc = "December"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1100)
    }
    #[doc = "Illegal Value"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1101)
    }
    #[doc = "Illegal Value"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1110)
    }
    #[doc = "Illegal Value"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(MON_CNT_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `YROFST`"]
pub type YROFST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YROFST`"]
pub struct YROFST_W<'a> {
    w: &'a mut W,
}
impl<'a> YROFST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - These bits give the value of the Months Counter . Valid Values are:"]
    #[inline(always)]
    pub fn mon_cnt(&self) -> MON_CNT_R {
        MON_CNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Year Offset Count Value"]
    #[inline(always)]
    pub fn yrofst(&self) -> YROFST_R {
        YROFST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits give the value of the Months Counter . Valid Values are:"]
    #[inline(always)]
    pub fn mon_cnt(&mut self) -> MON_CNT_W {
        MON_CNT_W { w: self }
    }
    #[doc = "Bits 8:15 - Year Offset Count Value"]
    #[inline(always)]
    pub fn yrofst(&mut self) -> YROFST_W {
        YROFST_W { w: self }
    }
}
