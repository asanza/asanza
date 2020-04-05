#[doc = "Reader of register CI"]
pub type R = crate::R<u8, super::CI>;
#[doc = "Writer for register CI"]
pub type W = crate::W<u8, super::CI>;
#[doc = "Register CI `reset()`'s with value 0"]
impl crate::ResetValue for super::CI {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPRFCI`"]
pub struct SPRFCI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRFCI_W<'a> {
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
#[doc = "Write proxy for field `SPTEFCI`"]
pub struct SPTEFCI_W<'a> {
    w: &'a mut W,
}
impl<'a> SPTEFCI_W<'a> {
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
#[doc = "Write proxy for field `RNFULLFCI`"]
pub struct RNFULLFCI_W<'a> {
    w: &'a mut W,
}
impl<'a> RNFULLFCI_W<'a> {
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
#[doc = "Write proxy for field `TNEAREFCI`"]
pub struct TNEAREFCI_W<'a> {
    w: &'a mut W,
}
impl<'a> TNEAREFCI_W<'a> {
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
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFOF_A {
    #[doc = "0: Receive FIFO overflow condition has not occurred"]
    _0 = 0,
    #[doc = "1: Receive FIFO overflow condition occurred"]
    _1 = 1,
}
impl From<RXFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFOF`"]
pub type RXFOF_R = crate::R<bool, RXFOF_A>;
impl RXFOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFOF_A {
        match self.bits {
            false => RXFOF_A::_0,
            true => RXFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFOF_A::_1
    }
}
#[doc = "Transmit FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFOF_A {
    #[doc = "0: Transmit FIFO overflow condition has not occurred"]
    _0 = 0,
    #[doc = "1: Transmit FIFO overflow condition occurred"]
    _1 = 1,
}
impl From<TXFOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFOF`"]
pub type TXFOF_R = crate::R<bool, TXFOF_A>;
impl TXFOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFOF_A {
        match self.bits {
            false => TXFOF_A::_0,
            true => TXFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFOF_A::_1
    }
}
#[doc = "Receive FIFO error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFERR_A {
    #[doc = "0: No receive FIFO error occurred"]
    _0 = 0,
    #[doc = "1: A receive FIFO error occurred"]
    _1 = 1,
}
impl From<RXFERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFERR`"]
pub type RXFERR_R = crate::R<bool, RXFERR_A>;
impl RXFERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFERR_A {
        match self.bits {
            false => RXFERR_A::_0,
            true => RXFERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFERR_A::_1
    }
}
#[doc = "Transmit FIFO error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFERR_A {
    #[doc = "0: No transmit FIFO error occurred"]
    _0 = 0,
    #[doc = "1: A transmit FIFO error occurred"]
    _1 = 1,
}
impl From<TXFERR_A> for bool {
    #[inline(always)]
    fn from(variant: TXFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFERR`"]
pub type TXFERR_R = crate::R<bool, TXFERR_A>;
impl TXFERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFERR_A {
        match self.bits {
            false => TXFERR_A::_0,
            true => TXFERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFERR_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    pub fn rxfof(&self) -> RXFOF_R {
        RXFOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO overflow flag"]
    #[inline(always)]
    pub fn txfof(&self) -> TXFOF_R {
        TXFOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO error flag"]
    #[inline(always)]
    pub fn rxferr(&self) -> RXFERR_R {
        RXFERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO error flag"]
    #[inline(always)]
    pub fn txferr(&self) -> TXFERR_R {
        TXFERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO full flag clear interrupt"]
    #[inline(always)]
    pub fn sprfci(&mut self) -> SPRFCI_W {
        SPRFCI_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO empty flag clear interrupt"]
    #[inline(always)]
    pub fn sptefci(&mut self) -> SPTEFCI_W {
        SPTEFCI_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO nearly full flag clear interrupt"]
    #[inline(always)]
    pub fn rnfullfci(&mut self) -> RNFULLFCI_W {
        RNFULLFCI_W { w: self }
    }
    #[doc = "Bit 3 - Transmit FIFO nearly empty flag clear interrupt"]
    #[inline(always)]
    pub fn tnearefci(&mut self) -> TNEAREFCI_W {
        TNEAREFCI_W { w: self }
    }
}
