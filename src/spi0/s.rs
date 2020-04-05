#[doc = "Reader of register S"]
pub type R = crate::R<u8, super::S>;
#[doc = "Writer for register S"]
pub type W = crate::W<u8, super::S>;
#[doc = "Register S `reset()`'s with value 0x20"]
impl crate::ResetValue for super::S {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "SPI read FIFO empty flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFIFOEF_A {
    #[doc = "0: Read FIFO has data. Reads of the DH:DL registers in 16-bit mode or the DL register in 8-bit mode will empty the read FIFO."]
    _0 = 0,
    #[doc = "1: Read FIFO is empty."]
    _1 = 1,
}
impl From<RFIFOEF_A> for bool {
    #[inline(always)]
    fn from(variant: RFIFOEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RFIFOEF`"]
pub type RFIFOEF_R = crate::R<bool, RFIFOEF_A>;
impl RFIFOEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFIFOEF_A {
        match self.bits {
            false => RFIFOEF_A::_0,
            true => RFIFOEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFIFOEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFIFOEF_A::_1
    }
}
#[doc = "Transmit FIFO full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFULLF_A {
    #[doc = "0: Transmit FIFO has less than 8 bytes"]
    _0 = 0,
    #[doc = "1: Transmit FIFO has 8 bytes of data"]
    _1 = 1,
}
impl From<TXFULLF_A> for bool {
    #[inline(always)]
    fn from(variant: TXFULLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFULLF`"]
pub type TXFULLF_R = crate::R<bool, TXFULLF_A>;
impl TXFULLF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFULLF_A {
        match self.bits {
            false => TXFULLF_A::_0,
            true => TXFULLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFULLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFULLF_A::_1
    }
}
#[doc = "Transmit FIFO nearly empty flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNEAREF_A {
    #[doc = "0: Transmit FIFO has more than 16 bits (when C3\\[TNEAREF_MARK\\]
is 0) or more than 32 bits (when C3\\[TNEAREF_MARK\\]
is 1) remaining to transmit"]
    _0 = 0,
    #[doc = "1: Transmit FIFO has an amount of data equal to or less than 16 bits (when C3\\[TNEAREF_MARK\\]
is 0) or 32 bits (when C3\\[TNEAREF_MARK\\]
is 1) remaining to transmit"]
    _1 = 1,
}
impl From<TNEAREF_A> for bool {
    #[inline(always)]
    fn from(variant: TNEAREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TNEAREF`"]
pub type TNEAREF_R = crate::R<bool, TNEAREF_A>;
impl TNEAREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNEAREF_A {
        match self.bits {
            false => TNEAREF_A::_0,
            true => TNEAREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TNEAREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TNEAREF_A::_1
    }
}
#[doc = "Receive FIFO nearly full flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNFULLF_A {
    #[doc = "0: Receive FIFO has received less than 48 bits (when C3\\[RNFULLF_MARK\\]
is 0) or less than 32 bits (when C3\\[RNFULLF_MARK\\]
is 1)"]
    _0 = 0,
    #[doc = "1: Receive FIFO has received data of an amount equal to or greater than 48 bits (when C3\\[RNFULLF_MARK\\]
is 0) or 32 bits (when C3\\[RNFULLF_MARK\\]
is 1)"]
    _1 = 1,
}
impl From<RNFULLF_A> for bool {
    #[inline(always)]
    fn from(variant: RNFULLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNFULLF`"]
pub type RNFULLF_R = crate::R<bool, RNFULLF_A>;
impl RNFULLF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNFULLF_A {
        match self.bits {
            false => RNFULLF_A::_0,
            true => RNFULLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNFULLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNFULLF_A::_1
    }
}
#[doc = "Master Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No mode fault error"]
    _0 = 0,
    #[doc = "1: Mode fault error detected"]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
#[doc = "SPI Transmit Buffer Empty Flag (when FIFO is not supported or not enabled) or SPI transmit FIFO empty flag (when FIFO is supported and enabled)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPTEF_A {
    #[doc = "0: SPI transmit buffer not empty (when FIFOMODE is not present or is 0) or SPI FIFO not empty (when FIFOMODE is 1)"]
    _0 = 0,
    #[doc = "1: SPI transmit buffer empty (when FIFOMODE is not present or is 0) or SPI FIFO empty (when FIFOMODE is 1)"]
    _1 = 1,
}
impl From<SPTEF_A> for bool {
    #[inline(always)]
    fn from(variant: SPTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPTEF`"]
pub type SPTEF_R = crate::R<bool, SPTEF_A>;
impl SPTEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPTEF_A {
        match self.bits {
            false => SPTEF_A::_0,
            true => SPTEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPTEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPTEF_A::_1
    }
}
#[doc = "SPI Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPMF_A {
    #[doc = "0: Value in the receive data buffer does not match the value in the MH:ML registers"]
    _0 = 0,
    #[doc = "1: Value in the receive data buffer matches the value in the MH:ML registers"]
    _1 = 1,
}
impl From<SPMF_A> for bool {
    #[inline(always)]
    fn from(variant: SPMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPMF`"]
pub type SPMF_R = crate::R<bool, SPMF_A>;
impl SPMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPMF_A {
        match self.bits {
            false => SPMF_A::_0,
            true => SPMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPMF_A::_1
    }
}
#[doc = "Write proxy for field `SPMF`"]
pub struct SPMF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Value in the receive data buffer does not match the value in the MH:ML registers"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPMF_A::_0)
    }
    #[doc = "Value in the receive data buffer matches the value in the MH:ML registers"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPMF_A::_1)
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
#[doc = "SPI Read Buffer Full Flag (when FIFO is not supported or not enabled) or SPI read FIFO FULL flag (when FIFO is supported and enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPRF_A {
    #[doc = "0: No data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is not full (when FIFOMODE is 1)"]
    _0 = 0,
    #[doc = "1: Data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is full (when FIFOMODE is 1)"]
    _1 = 1,
}
impl From<SPRF_A> for bool {
    #[inline(always)]
    fn from(variant: SPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPRF`"]
pub type SPRF_R = crate::R<bool, SPRF_A>;
impl SPRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPRF_A {
        match self.bits {
            false => SPRF_A::_0,
            true => SPRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPRF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - SPI read FIFO empty flag"]
    #[inline(always)]
    pub fn rfifoef(&self) -> RFIFOEF_R {
        RFIFOEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO full flag"]
    #[inline(always)]
    pub fn txfullf(&self) -> TXFULLF_R {
        TXFULLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO nearly empty flag"]
    #[inline(always)]
    pub fn tnearef(&self) -> TNEAREF_R {
        TNEAREF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO nearly full flag"]
    #[inline(always)]
    pub fn rnfullf(&self) -> RNFULLF_R {
        RNFULLF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Mode Fault Flag"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag (when FIFO is not supported or not enabled) or SPI transmit FIFO empty flag (when FIFO is supported and enabled)"]
    #[inline(always)]
    pub fn sptef(&self) -> SPTEF_R {
        SPTEF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    pub fn spmf(&self) -> SPMF_R {
        SPMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPI Read Buffer Full Flag (when FIFO is not supported or not enabled) or SPI read FIFO FULL flag (when FIFO is supported and enabled)"]
    #[inline(always)]
    pub fn sprf(&self) -> SPRF_R {
        SPRF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    pub fn spmf(&mut self) -> SPMF_W {
        SPMF_W { w: self }
    }
}
