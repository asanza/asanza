#[doc = "Reader of register WF24"]
pub type R = crate::R<u8, super::WF24>;
#[doc = "Writer for register WF24"]
pub type W = crate::W<u8, super::WF24>;
#[doc = "Register WF24 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF24 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPALCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    _1 = 1,
}
impl From<BPALCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPALCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPALCD24`"]
pub type BPALCD24_R = crate::R<bool, BPALCD24_A>;
impl BPALCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPALCD24_A {
        match self.bits {
            false => BPALCD24_A::_0,
            true => BPALCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPALCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPALCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPALCD24`"]
pub struct BPALCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPALCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPALCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPALCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPALCD24_A::_1)
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
pub enum BPBLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    _1 = 1,
}
impl From<BPBLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPBLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPBLCD24`"]
pub type BPBLCD24_R = crate::R<bool, BPBLCD24_A>;
impl BPBLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPBLCD24_A {
        match self.bits {
            false => BPBLCD24_A::_0,
            true => BPBLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPBLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPBLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPBLCD24`"]
pub struct BPBLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPBLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPBLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPBLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPBLCD24_A::_1)
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
pub enum BPCLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    _1 = 1,
}
impl From<BPCLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPCLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPCLCD24`"]
pub type BPCLCD24_R = crate::R<bool, BPCLCD24_A>;
impl BPCLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPCLCD24_A {
        match self.bits {
            false => BPCLCD24_A::_0,
            true => BPCLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPCLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPCLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPCLCD24`"]
pub struct BPCLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPCLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPCLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPCLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPCLCD24_A::_1)
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
pub enum BPDLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    _1 = 1,
}
impl From<BPDLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPDLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPDLCD24`"]
pub type BPDLCD24_R = crate::R<bool, BPDLCD24_A>;
impl BPDLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPDLCD24_A {
        match self.bits {
            false => BPDLCD24_A::_0,
            true => BPDLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPDLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPDLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPDLCD24`"]
pub struct BPDLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPDLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPDLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPDLCD24_A::_1)
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
pub enum BPELCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    _1 = 1,
}
impl From<BPELCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPELCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPELCD24`"]
pub type BPELCD24_R = crate::R<bool, BPELCD24_A>;
impl BPELCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPELCD24_A {
        match self.bits {
            false => BPELCD24_A::_0,
            true => BPELCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPELCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPELCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPELCD24`"]
pub struct BPELCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPELCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPELCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPELCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPELCD24_A::_1)
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
pub enum BPFLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    _1 = 1,
}
impl From<BPFLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPFLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPFLCD24`"]
pub type BPFLCD24_R = crate::R<bool, BPFLCD24_A>;
impl BPFLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPFLCD24_A {
        match self.bits {
            false => BPFLCD24_A::_0,
            true => BPFLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPFLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPFLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPFLCD24`"]
pub struct BPFLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPFLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPFLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPFLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPFLCD24_A::_1)
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
pub enum BPGLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    _1 = 1,
}
impl From<BPGLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPGLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPGLCD24`"]
pub type BPGLCD24_R = crate::R<bool, BPGLCD24_A>;
impl BPGLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPGLCD24_A {
        match self.bits {
            false => BPGLCD24_A::_0,
            true => BPGLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPGLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPGLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPGLCD24`"]
pub struct BPGLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPGLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPGLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPGLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPGLCD24_A::_1)
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
pub enum BPHLCD24_A {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    _0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    _1 = 1,
}
impl From<BPHLCD24_A> for bool {
    #[inline(always)]
    fn from(variant: BPHLCD24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BPHLCD24`"]
pub type BPHLCD24_R = crate::R<bool, BPHLCD24_A>;
impl BPHLCD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BPHLCD24_A {
        match self.bits {
            false => BPHLCD24_A::_0,
            true => BPHLCD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPHLCD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPHLCD24_A::_1
    }
}
#[doc = "Write proxy for field `BPHLCD24`"]
pub struct BPHLCD24_W<'a> {
    w: &'a mut W,
}
impl<'a> BPHLCD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPHLCD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPHLCD24_A::_0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPHLCD24_A::_1)
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
    pub fn bpalcd24(&self) -> BPALCD24_R {
        BPALCD24_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd24(&self) -> BPBLCD24_R {
        BPBLCD24_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd24(&self) -> BPCLCD24_R {
        BPCLCD24_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd24(&self) -> BPDLCD24_R {
        BPDLCD24_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd24(&self) -> BPELCD24_R {
        BPELCD24_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd24(&self) -> BPFLCD24_R {
        BPFLCD24_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd24(&self) -> BPGLCD24_R {
        BPGLCD24_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd24(&self) -> BPHLCD24_R {
        BPHLCD24_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd24(&mut self) -> BPALCD24_W {
        BPALCD24_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd24(&mut self) -> BPBLCD24_W {
        BPBLCD24_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd24(&mut self) -> BPCLCD24_W {
        BPCLCD24_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd24(&mut self) -> BPDLCD24_W {
        BPDLCD24_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd24(&mut self) -> BPELCD24_W {
        BPELCD24_W { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd24(&mut self) -> BPFLCD24_W {
        BPFLCD24_W { w: self }
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd24(&mut self) -> BPGLCD24_W {
        BPGLCD24_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd24(&mut self) -> BPHLCD24_W {
        BPHLCD24_W { w: self }
    }
}
