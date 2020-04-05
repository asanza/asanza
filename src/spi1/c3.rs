#[doc = "Reader of register C3"]
pub type R = crate::R<u8, super::C3>;
#[doc = "Writer for register C3"]
pub type W = crate::W<u8, super::C3>;
#[doc = "Register C3 `reset()`'s with value 0"]
impl crate::ResetValue for super::C3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOMODE_A {
    #[doc = "0: FIFO mode disabled"]
    _0 = 0,
    #[doc = "1: FIFO mode enabled"]
    _1 = 1,
}
impl From<FIFOMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFOMODE`"]
pub type FIFOMODE_R = crate::R<bool, FIFOMODE_A>;
impl FIFOMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOMODE_A {
        match self.bits {
            false => FIFOMODE_A::_0,
            true => FIFOMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOMODE_A::_1
    }
}
#[doc = "Write proxy for field `FIFOMODE`"]
pub struct FIFOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO mode disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOMODE_A::_0)
    }
    #[doc = "FIFO mode enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOMODE_A::_1)
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
#[doc = "Receive FIFO nearly full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNFULLIEN_A {
    #[doc = "0: No interrupt upon RNFULLF being set"]
    _0 = 0,
    #[doc = "1: Enable interrupts upon RNFULLF being set"]
    _1 = 1,
}
impl From<RNFULLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNFULLIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNFULLIEN`"]
pub type RNFULLIEN_R = crate::R<bool, RNFULLIEN_A>;
impl RNFULLIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNFULLIEN_A {
        match self.bits {
            false => RNFULLIEN_A::_0,
            true => RNFULLIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNFULLIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNFULLIEN_A::_1
    }
}
#[doc = "Write proxy for field `RNFULLIEN`"]
pub struct RNFULLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNFULLIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNFULLIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt upon RNFULLF being set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNFULLIEN_A::_0)
    }
    #[doc = "Enable interrupts upon RNFULLF being set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNFULLIEN_A::_1)
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
#[doc = "Transmit FIFO nearly empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNEARIEN_A {
    #[doc = "0: No interrupt upon TNEAREF being set"]
    _0 = 0,
    #[doc = "1: Enable interrupts upon TNEAREF being set"]
    _1 = 1,
}
impl From<TNEARIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TNEARIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TNEARIEN`"]
pub type TNEARIEN_R = crate::R<bool, TNEARIEN_A>;
impl TNEARIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNEARIEN_A {
        match self.bits {
            false => TNEARIEN_A::_0,
            true => TNEARIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TNEARIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TNEARIEN_A::_1
    }
}
#[doc = "Write proxy for field `TNEARIEN`"]
pub struct TNEARIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TNEARIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNEARIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt upon TNEAREF being set"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNEARIEN_A::_0)
    }
    #[doc = "Enable interrupts upon TNEAREF being set"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNEARIEN_A::_1)
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
#[doc = "Interrupt clearing mechanism select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTCLR_A {
    #[doc = "0: These interrupts are cleared when the corresponding flags are cleared depending on the state of the FIFOs"]
    _0 = 0,
    #[doc = "1: These interrupts are cleared by writing the corresponding bits in the CI register"]
    _1 = 1,
}
impl From<INTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: INTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTCLR`"]
pub type INTCLR_R = crate::R<bool, INTCLR_A>;
impl INTCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTCLR_A {
        match self.bits {
            false => INTCLR_A::_0,
            true => INTCLR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTCLR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTCLR_A::_1
    }
}
#[doc = "Write proxy for field `INTCLR`"]
pub struct INTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "These interrupts are cleared when the corresponding flags are cleared depending on the state of the FIFOs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTCLR_A::_0)
    }
    #[doc = "These interrupts are cleared by writing the corresponding bits in the CI register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTCLR_A::_1)
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
#[doc = "Receive FIFO nearly full watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNFULLF_MARK_A {
    #[doc = "0: RNFULLF is set when the receive FIFO has 48 bits or more"]
    _0 = 0,
    #[doc = "1: RNFULLF is set when the receive FIFO has 32 bits or more"]
    _1 = 1,
}
impl From<RNFULLF_MARK_A> for bool {
    #[inline(always)]
    fn from(variant: RNFULLF_MARK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNFULLF_MARK`"]
pub type RNFULLF_MARK_R = crate::R<bool, RNFULLF_MARK_A>;
impl RNFULLF_MARK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNFULLF_MARK_A {
        match self.bits {
            false => RNFULLF_MARK_A::_0,
            true => RNFULLF_MARK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNFULLF_MARK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNFULLF_MARK_A::_1
    }
}
#[doc = "Write proxy for field `RNFULLF_MARK`"]
pub struct RNFULLF_MARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RNFULLF_MARK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNFULLF_MARK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RNFULLF is set when the receive FIFO has 48 bits or more"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNFULLF_MARK_A::_0)
    }
    #[doc = "RNFULLF is set when the receive FIFO has 32 bits or more"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNFULLF_MARK_A::_1)
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
#[doc = "Transmit FIFO nearly empty watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNEAREF_MARK_A {
    #[doc = "0: TNEAREF is set when the transmit FIFO has 16 bits or less"]
    _0 = 0,
    #[doc = "1: TNEAREF is set when the transmit FIFO has 32 bits or less"]
    _1 = 1,
}
impl From<TNEAREF_MARK_A> for bool {
    #[inline(always)]
    fn from(variant: TNEAREF_MARK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TNEAREF_MARK`"]
pub type TNEAREF_MARK_R = crate::R<bool, TNEAREF_MARK_A>;
impl TNEAREF_MARK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNEAREF_MARK_A {
        match self.bits {
            false => TNEAREF_MARK_A::_0,
            true => TNEAREF_MARK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TNEAREF_MARK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TNEAREF_MARK_A::_1
    }
}
#[doc = "Write proxy for field `TNEAREF_MARK`"]
pub struct TNEAREF_MARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TNEAREF_MARK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNEAREF_MARK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TNEAREF is set when the transmit FIFO has 16 bits or less"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TNEAREF_MARK_A::_0)
    }
    #[doc = "TNEAREF is set when the transmit FIFO has 32 bits or less"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TNEAREF_MARK_A::_1)
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
impl R {
    #[doc = "Bit 0 - FIFO mode enable"]
    #[inline(always)]
    pub fn fifomode(&self) -> FIFOMODE_R {
        FIFOMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO nearly full interrupt enable"]
    #[inline(always)]
    pub fn rnfullien(&self) -> RNFULLIEN_R {
        RNFULLIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO nearly empty interrupt enable"]
    #[inline(always)]
    pub fn tnearien(&self) -> TNEARIEN_R {
        TNEARIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt clearing mechanism select"]
    #[inline(always)]
    pub fn intclr(&self) -> INTCLR_R {
        INTCLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO nearly full watermark"]
    #[inline(always)]
    pub fn rnfullf_mark(&self) -> RNFULLF_MARK_R {
        RNFULLF_MARK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO nearly empty watermark"]
    #[inline(always)]
    pub fn tnearef_mark(&self) -> TNEAREF_MARK_R {
        TNEAREF_MARK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO mode enable"]
    #[inline(always)]
    pub fn fifomode(&mut self) -> FIFOMODE_W {
        FIFOMODE_W { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO nearly full interrupt enable"]
    #[inline(always)]
    pub fn rnfullien(&mut self) -> RNFULLIEN_W {
        RNFULLIEN_W { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO nearly empty interrupt enable"]
    #[inline(always)]
    pub fn tnearien(&mut self) -> TNEARIEN_W {
        TNEARIEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt clearing mechanism select"]
    #[inline(always)]
    pub fn intclr(&mut self) -> INTCLR_W {
        INTCLR_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO nearly full watermark"]
    #[inline(always)]
    pub fn rnfullf_mark(&mut self) -> RNFULLF_MARK_W {
        RNFULLF_MARK_W { w: self }
    }
    #[doc = "Bit 5 - Transmit FIFO nearly empty watermark"]
    #[inline(always)]
    pub fn tnearef_mark(&mut self) -> TNEAREF_MARK_W {
        TNEAREF_MARK_W { w: self }
    }
}
