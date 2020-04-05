#[doc = "Reader of register WF29"]
pub type R = crate::R<u8, super::WF29>;
#[doc = "Writer for register WF29"]
pub type W = crate::W<u8, super::WF29>;
#[doc = "Register WF29 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF29 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPALCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    _1 = 1,
}
impl From<BPALCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPALCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPALCD29`"]
pub type BPALCD29_R = crate::R<bool, BPALCD29_A>;
impl BPALCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPALCD29_A {
        match self.bits {
            false => BPALCD29_A::_0,
            true => BPALCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPALCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPALCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPALCD29`"]
pub struct BPALCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPALCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPALCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPALCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPALCD29_A::_1)
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
pub enum BPBLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    _1 = 1,
}
impl From<BPBLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPBLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPBLCD29`"]
pub type BPBLCD29_R = crate::R<bool, BPBLCD29_A>;
impl BPBLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPBLCD29_A {
        match self.bits {
            false => BPBLCD29_A::_0,
            true => BPBLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPBLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPBLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPBLCD29`"]
pub struct BPBLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPBLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPBLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPBLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPBLCD29_A::_1)
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
pub enum BPCLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    _1 = 1,
}
impl From<BPCLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPCLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPCLCD29`"]
pub type BPCLCD29_R = crate::R<bool, BPCLCD29_A>;
impl BPCLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCLCD29_A {
        match self.bits {
            false => BPCLCD29_A::_0,
            true => BPCLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPCLCD29`"]
pub struct BPCLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPCLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPCLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCLCD29_A::_1)
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
pub enum BPDLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    _1 = 1,
}
impl From<BPDLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPDLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPDLCD29`"]
pub type BPDLCD29_R = crate::R<bool, BPDLCD29_A>;
impl BPDLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPDLCD29_A {
        match self.bits {
            false => BPDLCD29_A::_0,
            true => BPDLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPDLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPDLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPDLCD29`"]
pub struct BPDLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPDLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPDLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPDLCD29_A::_1)
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
pub enum BPELCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    _1 = 1,
}
impl From<BPELCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPELCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPELCD29`"]
pub type BPELCD29_R = crate::R<bool, BPELCD29_A>;
impl BPELCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPELCD29_A {
        match self.bits {
            false => BPELCD29_A::_0,
            true => BPELCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPELCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPELCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPELCD29`"]
pub struct BPELCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPELCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPELCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPELCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPELCD29_A::_1)
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
pub enum BPFLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    _1 = 1,
}
impl From<BPFLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPFLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPFLCD29`"]
pub type BPFLCD29_R = crate::R<bool, BPFLCD29_A>;
impl BPFLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFLCD29_A {
        match self.bits {
            false => BPFLCD29_A::_0,
            true => BPFLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPFLCD29`"]
pub struct BPFLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPFLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPFLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFLCD29_A::_1)
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
pub enum BPGLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    _1 = 1,
}
impl From<BPGLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPGLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPGLCD29`"]
pub type BPGLCD29_R = crate::R<bool, BPGLCD29_A>;
impl BPGLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPGLCD29_A {
        match self.bits {
            false => BPGLCD29_A::_0,
            true => BPGLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPGLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPGLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPGLCD29`"]
pub struct BPGLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPGLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPGLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPGLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPGLCD29_A::_1)
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
pub enum BPHLCD29_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    _1 = 1,
}
impl From<BPHLCD29_A> for bool {
    #[inline(always)]
    fn from(variant: BPHLCD29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPHLCD29`"]
pub type BPHLCD29_R = crate::R<bool, BPHLCD29_A>;
impl BPHLCD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPHLCD29_A {
        match self.bits {
            false => BPHLCD29_A::_0,
            true => BPHLCD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPHLCD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPHLCD29_A::_1
    }
}
#[doc = "Write proxy for field `BPHLCD29`"]
pub struct BPHLCD29_W<'a> {
    w: &'a mut W,
}
impl<'a> BPHLCD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPHLCD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPHLCD29_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPHLCD29_A::_1)
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
    pub fn bpalcd29(&self) -> BPALCD29_R {
        BPALCD29_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd29(&self) -> BPBLCD29_R {
        BPBLCD29_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd29(&self) -> BPCLCD29_R {
        BPCLCD29_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd29(&self) -> BPDLCD29_R {
        BPDLCD29_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd29(&self) -> BPELCD29_R {
        BPELCD29_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd29(&self) -> BPFLCD29_R {
        BPFLCD29_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd29(&self) -> BPGLCD29_R {
        BPGLCD29_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd29(&self) -> BPHLCD29_R {
        BPHLCD29_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd29(&mut self) -> BPALCD29_W {
        BPALCD29_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd29(&mut self) -> BPBLCD29_W {
        BPBLCD29_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd29(&mut self) -> BPCLCD29_W {
        BPCLCD29_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd29(&mut self) -> BPDLCD29_W {
        BPDLCD29_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd29(&mut self) -> BPELCD29_W {
        BPELCD29_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd29(&mut self) -> BPFLCD29_W {
        BPFLCD29_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd29(&mut self) -> BPGLCD29_W {
        BPGLCD29_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd29(&mut self) -> BPHLCD29_W {
        BPHLCD29_W { w: self }
    }
}
