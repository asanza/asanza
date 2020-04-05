#[doc = "Reader of register DAYS"]
pub type R = crate::R<u16, super::DAYS>;
#[doc = "Writer for register DAYS"]
pub type W = crate::W<u16, super::DAYS>;
#[doc = "Register DAYS `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DAYS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `DAY_CNT`"]
pub type DAY_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY_CNT`"]
pub struct DAY_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Day of Week Counter Value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOW_A {
    #[doc = "0: Sunday"]
    _0 = 0,
    #[doc = "1: Monday"]
    _1 = 1,
    #[doc = "2: Tuesday"]
    _10 = 2,
    #[doc = "3: Wednesday"]
    _11 = 3,
    #[doc = "4: Thrusday"]
    _100 = 4,
    #[doc = "5: Friday"]
    _101 = 5,
    #[doc = "6: Saturday"]
    _110 = 6,
}
impl From<DOW_A> for u8 {
    #[inline(always)]
    fn from(variant: DOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DOW`"]
pub type DOW_R = crate::R<u8, DOW_A>;
impl DOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DOW_A::_0),
            1 => Val(DOW_A::_1),
            2 => Val(DOW_A::_10),
            3 => Val(DOW_A::_11),
            4 => Val(DOW_A::_100),
            5 => Val(DOW_A::_101),
            6 => Val(DOW_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOW_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DOW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DOW_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DOW_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DOW_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DOW_A::_110
    }
}
#[doc = "Write proxy for field `DOW`"]
pub struct DOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOW_A::_0)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOW_A::_1)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DOW_A::_10)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DOW_A::_11)
    }
    #[doc = "Thrusday"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DOW_A::_100)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DOW_A::_101)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DOW_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Days Counter Value."]
    #[inline(always)]
    pub fn day_cnt(&self) -> DAY_CNT_R {
        DAY_CNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Day of Week Counter Value."]
    #[inline(always)]
    pub fn dow(&self) -> DOW_R {
        DOW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Days Counter Value."]
    #[inline(always)]
    pub fn day_cnt(&mut self) -> DAY_CNT_W {
        DAY_CNT_W { w: self }
    }
    #[doc = "Bits 8:10 - Day of Week Counter Value."]
    #[inline(always)]
    pub fn dow(&mut self) -> DOW_W {
        DOW_W { w: self }
    }
}
