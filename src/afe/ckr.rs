#[doc = "Reader of register CKR"]
pub type R = crate::R<u32, super::CKR>;
#[doc = "Writer for register CKR"]
pub type W = crate::W<u32, super::CKR>;
#[doc = "Register CKR `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::CKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLS_A {
    #[doc = "0: mod_clk0"]
    _00 = 0,
    #[doc = "1: mod_clk1"]
    _01 = 1,
    #[doc = "2: mod_clk2"]
    _10 = 2,
    #[doc = "3: mod_clk3"]
    _11 = 3,
}
impl From<CLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLS`"]
pub type CLS_R = crate::R<u8, CLS_A>;
impl CLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLS_A {
        match self.bits {
            0 => CLS_A::_00,
            1 => CLS_A::_01,
            2 => CLS_A::_10,
            3 => CLS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLS_A::_11
    }
}
#[doc = "Write proxy for field `CLS`"]
pub struct CLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "mod_clk0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLS_A::_00)
    }
    #[doc = "mod_clk1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLS_A::_01)
    }
    #[doc = "mod_clk2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLS_A::_10)
    }
    #[doc = "mod_clk3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Clock Divider Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: divide by 1"]
    _0000 = 0,
    #[doc = "1: divide by 2 (default)"]
    _0001 = 1,
    #[doc = "2: divide by 4"]
    _0010 = 2,
    #[doc = "3: divide by 8"]
    _0011 = 3,
    #[doc = "4: divide by 16"]
    _0100 = 4,
    #[doc = "5: divide by 32"]
    _0101 = 5,
    #[doc = "6: divide by 64"]
    _0110 = 6,
    #[doc = "7: divide by 128"]
    _0111 = 7,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, DIV_A>;
impl DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIV_A::_0000),
            1 => Val(DIV_A::_0001),
            2 => Val(DIV_A::_0010),
            3 => Val(DIV_A::_0011),
            4 => Val(DIV_A::_0100),
            5 => Val(DIV_A::_0101),
            6 => Val(DIV_A::_0110),
            7 => Val(DIV_A::_0111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DIV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DIV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DIV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DIV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DIV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DIV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DIV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DIV_A::_0111
    }
}
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIV_A::_0000)
    }
    #[doc = "divide by 2 (default)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIV_A::_0001)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIV_A::_0010)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIV_A::_0011)
    }
    #[doc = "divide by 16"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIV_A::_0100)
    }
    #[doc = "divide by 32"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIV_A::_0101)
    }
    #[doc = "divide by 64"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIV_A::_0110)
    }
    #[doc = "divide by 128"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIV_A::_0111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:22 - Clock Source Select"]
    #[inline(always)]
    pub fn cls(&self) -> CLS_R {
        CLS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - Clock Divider Select"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22 - Clock Source Select"]
    #[inline(always)]
    pub fn cls(&mut self) -> CLS_W {
        CLS_W { w: self }
    }
    #[doc = "Bits 28:31 - Clock Divider Select"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
