#[doc = "Reader of register WF3"]
pub type R = crate::R<u8, super::WF3>;
#[doc = "Writer for register WF3"]
pub type W = crate::W<u8, super::WF3>;
#[doc = "Register WF3 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPALCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    _1 = 1,
}
impl From<BPALCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPALCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPALCD3`"]
pub type BPALCD3_R = crate::R<bool, BPALCD3_A>;
impl BPALCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPALCD3_A {
        match self.bits {
            false => BPALCD3_A::_0,
            true => BPALCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPALCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPALCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPALCD3`"]
pub struct BPALCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPALCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPALCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPALCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPALCD3_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPBLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    _1 = 1,
}
impl From<BPBLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPBLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPBLCD3`"]
pub type BPBLCD3_R = crate::R<bool, BPBLCD3_A>;
impl BPBLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPBLCD3_A {
        match self.bits {
            false => BPBLCD3_A::_0,
            true => BPBLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPBLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPBLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPBLCD3`"]
pub struct BPBLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPBLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPBLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPBLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPBLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPCLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    _1 = 1,
}
impl From<BPCLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPCLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPCLCD3`"]
pub type BPCLCD3_R = crate::R<bool, BPCLCD3_A>;
impl BPCLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCLCD3_A {
        match self.bits {
            false => BPCLCD3_A::_0,
            true => BPCLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPCLCD3`"]
pub struct BPCLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPCLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPCLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPDLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    _1 = 1,
}
impl From<BPDLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPDLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPDLCD3`"]
pub type BPDLCD3_R = crate::R<bool, BPDLCD3_A>;
impl BPDLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPDLCD3_A {
        match self.bits {
            false => BPDLCD3_A::_0,
            true => BPDLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPDLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPDLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPDLCD3`"]
pub struct BPDLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPDLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPDLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPDLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPELCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    _1 = 1,
}
impl From<BPELCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPELCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPELCD3`"]
pub type BPELCD3_R = crate::R<bool, BPELCD3_A>;
impl BPELCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPELCD3_A {
        match self.bits {
            false => BPELCD3_A::_0,
            true => BPELCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPELCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPELCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPELCD3`"]
pub struct BPELCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPELCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPELCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPELCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPELCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPFLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    _1 = 1,
}
impl From<BPFLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPFLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPFLCD3`"]
pub type BPFLCD3_R = crate::R<bool, BPFLCD3_A>;
impl BPFLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFLCD3_A {
        match self.bits {
            false => BPFLCD3_A::_0,
            true => BPFLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPFLCD3`"]
pub struct BPFLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPFLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPFLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPGLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    _1 = 1,
}
impl From<BPGLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPGLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPGLCD3`"]
pub type BPGLCD3_R = crate::R<bool, BPGLCD3_A>;
impl BPGLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPGLCD3_A {
        match self.bits {
            false => BPGLCD3_A::_0,
            true => BPGLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPGLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPGLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPGLCD3`"]
pub struct BPGLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPGLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPGLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPGLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPGLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPHLCD3_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    _1 = 1,
}
impl From<BPHLCD3_A> for bool {
    #[inline(always)]
    fn from(variant: BPHLCD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPHLCD3`"]
pub type BPHLCD3_R = crate::R<bool, BPHLCD3_A>;
impl BPHLCD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPHLCD3_A {
        match self.bits {
            false => BPHLCD3_A::_0,
            true => BPHLCD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPHLCD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPHLCD3_A::_1
    }
}
#[doc = "Write proxy for field `BPHLCD3`"]
pub struct BPHLCD3_W<'a> {
    w: &'a mut W,
}
impl<'a> BPHLCD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPHLCD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPHLCD3_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPHLCD3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd3(&self) -> BPALCD3_R {
        BPALCD3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd3(&self) -> BPBLCD3_R {
        BPBLCD3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd3(&self) -> BPCLCD3_R {
        BPCLCD3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd3(&self) -> BPDLCD3_R {
        BPDLCD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd3(&self) -> BPELCD3_R {
        BPELCD3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd3(&self) -> BPFLCD3_R {
        BPFLCD3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd3(&self) -> BPGLCD3_R {
        BPGLCD3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd3(&self) -> BPHLCD3_R {
        BPHLCD3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd3(&mut self) -> BPALCD3_W {
        BPALCD3_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd3(&mut self) -> BPBLCD3_W {
        BPBLCD3_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd3(&mut self) -> BPCLCD3_W {
        BPCLCD3_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd3(&mut self) -> BPDLCD3_W {
        BPDLCD3_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd3(&mut self) -> BPELCD3_W {
        BPELCD3_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd3(&mut self) -> BPFLCD3_W {
        BPFLCD3_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd3(&mut self) -> BPGLCD3_W {
        BPGLCD3_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd3(&mut self) -> BPHLCD3_W {
        BPHLCD3_W { w: self }
    }
}
