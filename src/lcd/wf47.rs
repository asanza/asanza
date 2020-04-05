#[doc = "Reader of register WF47"]
pub type R = crate::R<u8, super::WF47>;
#[doc = "Writer for register WF47"]
pub type W = crate::W<u8, super::WF47>;
#[doc = "Register WF47 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF47 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPALCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    _1 = 1,
}
impl From<BPALCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPALCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPALCD47`"]
pub type BPALCD47_R = crate::R<bool, BPALCD47_A>;
impl BPALCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPALCD47_A {
        match self.bits {
            false => BPALCD47_A::_0,
            true => BPALCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPALCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPALCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPALCD47`"]
pub struct BPALCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPALCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPALCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPALCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPALCD47_A::_1)
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
pub enum BPBLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    _1 = 1,
}
impl From<BPBLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPBLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPBLCD47`"]
pub type BPBLCD47_R = crate::R<bool, BPBLCD47_A>;
impl BPBLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPBLCD47_A {
        match self.bits {
            false => BPBLCD47_A::_0,
            true => BPBLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPBLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPBLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPBLCD47`"]
pub struct BPBLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPBLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPBLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPBLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPBLCD47_A::_1)
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
pub enum BPCLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    _1 = 1,
}
impl From<BPCLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPCLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPCLCD47`"]
pub type BPCLCD47_R = crate::R<bool, BPCLCD47_A>;
impl BPCLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCLCD47_A {
        match self.bits {
            false => BPCLCD47_A::_0,
            true => BPCLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPCLCD47`"]
pub struct BPCLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPCLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPCLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCLCD47_A::_1)
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
pub enum BPDLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    _1 = 1,
}
impl From<BPDLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPDLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPDLCD47`"]
pub type BPDLCD47_R = crate::R<bool, BPDLCD47_A>;
impl BPDLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPDLCD47_A {
        match self.bits {
            false => BPDLCD47_A::_0,
            true => BPDLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPDLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPDLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPDLCD47`"]
pub struct BPDLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPDLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPDLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPDLCD47_A::_1)
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
pub enum BPELCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    _1 = 1,
}
impl From<BPELCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPELCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPELCD47`"]
pub type BPELCD47_R = crate::R<bool, BPELCD47_A>;
impl BPELCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPELCD47_A {
        match self.bits {
            false => BPELCD47_A::_0,
            true => BPELCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPELCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPELCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPELCD47`"]
pub struct BPELCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPELCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPELCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPELCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPELCD47_A::_1)
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
pub enum BPFLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    _1 = 1,
}
impl From<BPFLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPFLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPFLCD47`"]
pub type BPFLCD47_R = crate::R<bool, BPFLCD47_A>;
impl BPFLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFLCD47_A {
        match self.bits {
            false => BPFLCD47_A::_0,
            true => BPFLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPFLCD47`"]
pub struct BPFLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPFLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPFLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFLCD47_A::_1)
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
pub enum BPGLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    _1 = 1,
}
impl From<BPGLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPGLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPGLCD47`"]
pub type BPGLCD47_R = crate::R<bool, BPGLCD47_A>;
impl BPGLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPGLCD47_A {
        match self.bits {
            false => BPGLCD47_A::_0,
            true => BPGLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPGLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPGLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPGLCD47`"]
pub struct BPGLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPGLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPGLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPGLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPGLCD47_A::_1)
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
pub enum BPHLCD47_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    _1 = 1,
}
impl From<BPHLCD47_A> for bool {
    #[inline(always)]
    fn from(variant: BPHLCD47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPHLCD47`"]
pub type BPHLCD47_R = crate::R<bool, BPHLCD47_A>;
impl BPHLCD47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPHLCD47_A {
        match self.bits {
            false => BPHLCD47_A::_0,
            true => BPHLCD47_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPHLCD47_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPHLCD47_A::_1
    }
}
#[doc = "Write proxy for field `BPHLCD47`"]
pub struct BPHLCD47_W<'a> {
    w: &'a mut W,
}
impl<'a> BPHLCD47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPHLCD47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPHLCD47_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPHLCD47_A::_1)
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
    pub fn bpalcd47(&self) -> BPALCD47_R {
        BPALCD47_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd47(&self) -> BPBLCD47_R {
        BPBLCD47_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd47(&self) -> BPCLCD47_R {
        BPCLCD47_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd47(&self) -> BPDLCD47_R {
        BPDLCD47_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd47(&self) -> BPELCD47_R {
        BPELCD47_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd47(&self) -> BPFLCD47_R {
        BPFLCD47_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd47(&self) -> BPGLCD47_R {
        BPGLCD47_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd47(&self) -> BPHLCD47_R {
        BPHLCD47_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd47(&mut self) -> BPALCD47_W {
        BPALCD47_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd47(&mut self) -> BPBLCD47_W {
        BPBLCD47_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd47(&mut self) -> BPCLCD47_W {
        BPCLCD47_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd47(&mut self) -> BPDLCD47_W {
        BPDLCD47_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd47(&mut self) -> BPELCD47_W {
        BPELCD47_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd47(&mut self) -> BPFLCD47_W {
        BPFLCD47_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd47(&mut self) -> BPGLCD47_W {
        BPGLCD47_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd47(&mut self) -> BPHLCD47_W {
        BPHLCD47_W { w: self }
    }
}
