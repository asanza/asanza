#[doc = "Reader of register WF9"]
pub type R = crate::R<u8, super::WF9>;
#[doc = "Writer for register WF9"]
pub type W = crate::W<u8, super::WF9>;
#[doc = "Register WF9 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF9 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPALCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    _1 = 1,
}
impl From<BPALCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPALCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPALCD9`"]
pub type BPALCD9_R = crate::R<bool, BPALCD9_A>;
impl BPALCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPALCD9_A {
        match self.bits {
            false => BPALCD9_A::_0,
            true => BPALCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPALCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPALCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPALCD9`"]
pub struct BPALCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPALCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPALCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPALCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPALCD9_A::_1)
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
pub enum BPBLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    _1 = 1,
}
impl From<BPBLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPBLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPBLCD9`"]
pub type BPBLCD9_R = crate::R<bool, BPBLCD9_A>;
impl BPBLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPBLCD9_A {
        match self.bits {
            false => BPBLCD9_A::_0,
            true => BPBLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPBLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPBLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPBLCD9`"]
pub struct BPBLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPBLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPBLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPBLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPBLCD9_A::_1)
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
pub enum BPCLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    _1 = 1,
}
impl From<BPCLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPCLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPCLCD9`"]
pub type BPCLCD9_R = crate::R<bool, BPCLCD9_A>;
impl BPCLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCLCD9_A {
        match self.bits {
            false => BPCLCD9_A::_0,
            true => BPCLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPCLCD9`"]
pub struct BPCLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPCLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPCLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCLCD9_A::_1)
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
pub enum BPDLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    _1 = 1,
}
impl From<BPDLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPDLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPDLCD9`"]
pub type BPDLCD9_R = crate::R<bool, BPDLCD9_A>;
impl BPDLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPDLCD9_A {
        match self.bits {
            false => BPDLCD9_A::_0,
            true => BPDLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPDLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPDLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPDLCD9`"]
pub struct BPDLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPDLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPDLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPDLCD9_A::_1)
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
pub enum BPELCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    _1 = 1,
}
impl From<BPELCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPELCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPELCD9`"]
pub type BPELCD9_R = crate::R<bool, BPELCD9_A>;
impl BPELCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPELCD9_A {
        match self.bits {
            false => BPELCD9_A::_0,
            true => BPELCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPELCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPELCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPELCD9`"]
pub struct BPELCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPELCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPELCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPELCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPELCD9_A::_1)
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
pub enum BPFLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    _1 = 1,
}
impl From<BPFLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPFLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPFLCD9`"]
pub type BPFLCD9_R = crate::R<bool, BPFLCD9_A>;
impl BPFLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFLCD9_A {
        match self.bits {
            false => BPFLCD9_A::_0,
            true => BPFLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPFLCD9`"]
pub struct BPFLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPFLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPFLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFLCD9_A::_1)
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
pub enum BPGLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    _1 = 1,
}
impl From<BPGLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPGLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPGLCD9`"]
pub type BPGLCD9_R = crate::R<bool, BPGLCD9_A>;
impl BPGLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPGLCD9_A {
        match self.bits {
            false => BPGLCD9_A::_0,
            true => BPGLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPGLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPGLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPGLCD9`"]
pub struct BPGLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPGLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPGLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPGLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPGLCD9_A::_1)
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
pub enum BPHLCD9_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    _1 = 1,
}
impl From<BPHLCD9_A> for bool {
    #[inline(always)]
    fn from(variant: BPHLCD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPHLCD9`"]
pub type BPHLCD9_R = crate::R<bool, BPHLCD9_A>;
impl BPHLCD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPHLCD9_A {
        match self.bits {
            false => BPHLCD9_A::_0,
            true => BPHLCD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPHLCD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPHLCD9_A::_1
    }
}
#[doc = "Write proxy for field `BPHLCD9`"]
pub struct BPHLCD9_W<'a> {
    w: &'a mut W,
}
impl<'a> BPHLCD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPHLCD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPHLCD9_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPHLCD9_A::_1)
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
    pub fn bpalcd9(&self) -> BPALCD9_R {
        BPALCD9_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd9(&self) -> BPBLCD9_R {
        BPBLCD9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd9(&self) -> BPCLCD9_R {
        BPCLCD9_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd9(&self) -> BPDLCD9_R {
        BPDLCD9_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd9(&self) -> BPELCD9_R {
        BPELCD9_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd9(&self) -> BPFLCD9_R {
        BPFLCD9_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd9(&self) -> BPGLCD9_R {
        BPGLCD9_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd9(&self) -> BPHLCD9_R {
        BPHLCD9_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd9(&mut self) -> BPALCD9_W {
        BPALCD9_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd9(&mut self) -> BPBLCD9_W {
        BPBLCD9_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd9(&mut self) -> BPCLCD9_W {
        BPCLCD9_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd9(&mut self) -> BPDLCD9_W {
        BPDLCD9_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd9(&mut self) -> BPELCD9_W {
        BPELCD9_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd9(&mut self) -> BPFLCD9_W {
        BPFLCD9_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd9(&mut self) -> BPGLCD9_W {
        BPGLCD9_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd9(&mut self) -> BPHLCD9_W {
        BPHLCD9_W { w: self }
    }
}
