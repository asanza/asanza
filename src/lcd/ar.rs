#[doc = "Reader of register AR"]
pub type R = crate::R<u32, super::AR>;
#[doc = "Writer for register AR"]
pub type W = crate::W<u32, super::AR>;
#[doc = "Register AR `reset()`'s with value 0"]
impl crate::ResetValue for super::AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRATE`"]
pub type BRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRATE`"]
pub struct BRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Blink mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BMODE_A {
    #[doc = "0: Display blank during the blink period."]
    _0 = 0,
    #[doc = "1: Display alternate display during blink period (Ignored if duty is 5 or greater)."]
    _1 = 1,
}
impl From<BMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BMODE`"]
pub type BMODE_R = crate::R<bool, BMODE_A>;
impl BMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BMODE_A {
        match self.bits {
            false => BMODE_A::_0,
            true => BMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BMODE_A::_1
    }
}
#[doc = "Write proxy for field `BMODE`"]
pub struct BMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Display blank during the blink period."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BMODE_A::_0)
    }
    #[doc = "Display alternate display during blink period (Ignored if duty is 5 or greater)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BMODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Blank display mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLANK_A {
    #[doc = "0: Normal or alternate display mode."]
    _0 = 0,
    #[doc = "1: Blank display mode."]
    _1 = 1,
}
impl From<BLANK_A> for bool {
    #[inline(always)]
    fn from(variant: BLANK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLANK`"]
pub type BLANK_R = crate::R<bool, BLANK_A>;
impl BLANK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLANK_A {
        match self.bits {
            false => BLANK_A::_0,
            true => BLANK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLANK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLANK_A::_1
    }
}
#[doc = "Write proxy for field `BLANK`"]
pub struct BLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLANK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal or alternate display mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLANK_A::_0)
    }
    #[doc = "Blank display mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLANK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Alternate display mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_A {
    #[doc = "0: Normal display mode."]
    _0 = 0,
    #[doc = "1: Alternate display mode."]
    _1 = 1,
}
impl From<ALT_A> for bool {
    #[inline(always)]
    fn from(variant: ALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALT`"]
pub type ALT_R = crate::R<bool, ALT_A>;
impl ALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALT_A {
        match self.bits {
            false => ALT_A::_0,
            true => ALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALT_A::_1
    }
}
#[doc = "Write proxy for field `ALT`"]
pub struct ALT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal display mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALT_A::_0)
    }
    #[doc = "Alternate display mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Blink command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLINK_A {
    #[doc = "0: Disables blinking."]
    _0 = 0,
    #[doc = "1: Starts blinking at blinking frequency specified by LCD blink rate calculation."]
    _1 = 1,
}
impl From<BLINK_A> for bool {
    #[inline(always)]
    fn from(variant: BLINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLINK`"]
pub type BLINK_R = crate::R<bool, BLINK_A>;
impl BLINK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLINK_A {
        match self.bits {
            false => BLINK_A::_0,
            true => BLINK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLINK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLINK_A::_1
    }
}
#[doc = "Write proxy for field `BLINK`"]
pub struct BLINK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLINK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables blinking."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLINK_A::_0)
    }
    #[doc = "Starts blinking at blinking frequency specified by LCD blink rate calculation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLINK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "LCD Frame Frequency Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDIF_A {
    #[doc = "0: Frame frequency interrupt condition has not occurred."]
    _0 = 0,
    #[doc = "1: Start of SLCD frame has occurred."]
    _1 = 1,
}
impl From<LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDIF`"]
pub type LCDIF_R = crate::R<bool, LCDIF_A>;
impl LCDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIF_A {
        match self.bits {
            false => LCDIF_A::_0,
            true => LCDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDIF_A::_1
    }
}
#[doc = "Write proxy for field `LCDIF`"]
pub struct LCDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame frequency interrupt condition has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCDIF_A::_0)
    }
    #[doc = "Start of SLCD frame has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCDIF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Blink-rate configuration"]
    #[inline(always)]
    pub fn brate(&self) -> BRATE_R {
        BRATE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Blink mode"]
    #[inline(always)]
    pub fn bmode(&self) -> BMODE_R {
        BMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Blank display mode"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Alternate display mode"]
    #[inline(always)]
    pub fn alt(&self) -> ALT_R {
        ALT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Blink command"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCD Frame Frequency Interrupt flag"]
    #[inline(always)]
    pub fn lcdif(&self) -> LCDIF_R {
        LCDIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blink-rate configuration"]
    #[inline(always)]
    pub fn brate(&mut self) -> BRATE_W {
        BRATE_W { w: self }
    }
    #[doc = "Bit 3 - Blink mode"]
    #[inline(always)]
    pub fn bmode(&mut self) -> BMODE_W {
        BMODE_W { w: self }
    }
    #[doc = "Bit 5 - Blank display mode"]
    #[inline(always)]
    pub fn blank(&mut self) -> BLANK_W {
        BLANK_W { w: self }
    }
    #[doc = "Bit 6 - Alternate display mode"]
    #[inline(always)]
    pub fn alt(&mut self) -> ALT_W {
        ALT_W { w: self }
    }
    #[doc = "Bit 7 - Blink command"]
    #[inline(always)]
    pub fn blink(&mut self) -> BLINK_W {
        BLINK_W { w: self }
    }
    #[doc = "Bit 15 - LCD Frame Frequency Interrupt flag"]
    #[inline(always)]
    pub fn lcdif(&mut self) -> LCDIF_W {
        LCDIF_W { w: self }
    }
}
