#[doc = "Reader of register SCGC6"]
pub type R = crate::R<u32, super::SCGC6>;
#[doc = "Writer for register SCGC6"]
pub type W = crate::W<u32, super::SCGC6>;
#[doc = "Register SCGC6 `reset()`'s with value 0xc000_0001"]
impl crate::ResetValue for super::SCGC6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0001
    }
}
#[doc = "FTFA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTFA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTFA_A> for bool {
    #[inline(always)]
    fn from(variant: FTFA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTFA`"]
pub type FTFA_R = crate::R<bool, FTFA_A>;
impl FTFA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTFA_A {
        match self.bits {
            false => FTFA_A::_0,
            true => FTFA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTFA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTFA_A::_1
    }
}
#[doc = "Write proxy for field `FTFA`"]
pub struct FTFA_W<'a> {
    w: &'a mut W,
}
impl<'a> FTFA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTFA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTFA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTFA_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DMA Channel MUX Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACHMUX_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMACHMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DMACHMUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMACHMUX`"]
pub type DMACHMUX_R = crate::R<bool, DMACHMUX_A>;
impl DMACHMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACHMUX_A {
        match self.bits {
            false => DMACHMUX_A::_0,
            true => DMACHMUX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMACHMUX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMACHMUX_A::_1
    }
}
#[doc = "Write proxy for field `DMACHMUX`"]
pub struct DMACHMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACHMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMACHMUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMACHMUX_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMACHMUX_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "RNGA Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<RNGA_A> for bool {
    #[inline(always)]
    fn from(variant: RNGA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNGA`"]
pub type RNGA_R = crate::R<bool, RNGA_A>;
impl RNGA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGA_A {
        match self.bits {
            false => RNGA_A::_0,
            true => RNGA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNGA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNGA_A::_1
    }
}
#[doc = "Write proxy for field `RNGA`"]
pub struct RNGA_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNGA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNGA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "LPUART Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPUART_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPUART`"]
pub type LPUART_R = crate::R<bool, LPUART_A>;
impl LPUART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART_A {
        match self.bits {
            false => LPUART_A::_0,
            true => LPUART_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPUART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPUART_A::_1
    }
}
#[doc = "Write proxy for field `LPUART`"]
pub struct LPUART_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPUART_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPUART_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "SAR ADC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, ADC_A>;
impl ADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::_0,
            true => ADC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC_A::_1
    }
}
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "PIT0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PIT0_A> for bool {
    #[inline(always)]
    fn from(variant: PIT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIT0`"]
pub type PIT0_R = crate::R<bool, PIT0_A>;
impl PIT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT0_A {
        match self.bits {
            false => PIT0_A::_0,
            true => PIT0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT0_A::_1
    }
}
#[doc = "Write proxy for field `PIT0`"]
pub struct PIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PIT1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIT1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PIT1_A> for bool {
    #[inline(always)]
    fn from(variant: PIT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIT1`"]
pub type PIT1_R = crate::R<bool, PIT1_A>;
impl PIT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT1_A {
        match self.bits {
            false => PIT1_A::_0,
            true => PIT1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT1_A::_1
    }
}
#[doc = "Write proxy for field `PIT1`"]
pub struct PIT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "AFE Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFE_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<AFE_A> for bool {
    #[inline(always)]
    fn from(variant: AFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AFE`"]
pub type AFE_R = crate::R<bool, AFE_A>;
impl AFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFE_A {
        match self.bits {
            false => AFE_A::_0,
            true => AFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AFE_A::_1
    }
}
#[doc = "Write proxy for field `AFE`"]
pub struct AFE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AFE_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Programmable CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, CRC_A>;
impl CRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC_A::_1
    }
}
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "PDB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PDB_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDB`"]
pub type PDB_R = crate::R<bool, PDB_A>;
impl PDB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_A {
        match self.bits {
            false => PDB_A::_0,
            true => PDB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDB_A::_1
    }
}
#[doc = "Write proxy for field `PDB`"]
pub struct PDB_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "PCTLJ Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTJ_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTJ_A> for bool {
    #[inline(always)]
    fn from(variant: PORTJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTJ`"]
pub type PORTJ_R = crate::R<bool, PORTJ_A>;
impl PORTJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTJ_A {
        match self.bits {
            false => PORTJ_A::_0,
            true => PORTJ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTJ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTJ_A::_1
    }
}
#[doc = "Write proxy for field `PORTJ`"]
pub struct PORTJ_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTJ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTJ_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTJ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "PCTLK Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTK_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTK_A> for bool {
    #[inline(always)]
    fn from(variant: PORTK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTK`"]
pub type PORTK_R = crate::R<bool, PORTK_A>;
impl PORTK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTK_A {
        match self.bits {
            false => PORTK_A::_0,
            true => PORTK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTK_A::_1
    }
}
#[doc = "Write proxy for field `PORTK`"]
pub struct PORTK_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTK_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "PCTLL Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTL_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTL_A> for bool {
    #[inline(always)]
    fn from(variant: PORTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTL`"]
pub type PORTL_R = crate::R<bool, PORTL_A>;
impl PORTL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTL_A {
        match self.bits {
            false => PORTL_A::_0,
            true => PORTL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTL_A::_1
    }
}
#[doc = "Write proxy for field `PORTL`"]
pub struct PORTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTL_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "PCTLM Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTM_A> for bool {
    #[inline(always)]
    fn from(variant: PORTM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTM`"]
pub type PORTM_R = crate::R<bool, PORTM_A>;
impl PORTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTM_A {
        match self.bits {
            false => PORTM_A::_0,
            true => PORTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTM_A::_1
    }
}
#[doc = "Write proxy for field `PORTM`"]
pub struct PORTM_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "LPTMR Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTMR_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<LPTMR_A> for bool {
    #[inline(always)]
    fn from(variant: LPTMR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTMR`"]
pub type LPTMR_R = crate::R<bool, LPTMR_A>;
impl LPTMR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTMR_A {
        match self.bits {
            false => LPTMR_A::_0,
            true => LPTMR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPTMR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPTMR_A::_1
    }
}
#[doc = "Write proxy for field `LPTMR`"]
pub struct LPTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPTMR_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPTMR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FTFA Clock Gate Control"]
    #[inline(always)]
    pub fn ftfa(&self) -> FTFA_R {
        FTFA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel MUX Clock Gate Control"]
    #[inline(always)]
    pub fn dmachmux(&self) -> DMACHMUX_R {
        DMACHMUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&self) -> RNGA_R {
        RNGA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPUART Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart(&self) -> LPUART_R {
        LPUART_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SAR ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PIT0 Clock Gate Control"]
    #[inline(always)]
    pub fn pit0(&self) -> PIT0_R {
        PIT0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PIT1 Clock Gate Control"]
    #[inline(always)]
    pub fn pit1(&self) -> PIT1_R {
        PIT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AFE Clock Gate Control"]
    #[inline(always)]
    pub fn afe(&self) -> AFE_R {
        AFE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Programmable CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PCTLJ Clock Gate Control"]
    #[inline(always)]
    pub fn portj(&self) -> PORTJ_R {
        PORTJ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - PCTLK Clock Gate Control"]
    #[inline(always)]
    pub fn portk(&self) -> PORTK_R {
        PORTK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PCTLL Clock Gate Control"]
    #[inline(always)]
    pub fn portl(&self) -> PORTL_R {
        PORTL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PCTLM Clock Gate Control"]
    #[inline(always)]
    pub fn portm(&self) -> PORTM_R {
        PORTM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LPTMR Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr(&self) -> LPTMR_R {
        LPTMR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTFA Clock Gate Control"]
    #[inline(always)]
    pub fn ftfa(&mut self) -> FTFA_W {
        FTFA_W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel MUX Clock Gate Control"]
    #[inline(always)]
    pub fn dmachmux(&mut self) -> DMACHMUX_W {
        DMACHMUX_W { w: self }
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&mut self) -> RNGA_W {
        RNGA_W { w: self }
    }
    #[doc = "Bit 10 - LPUART Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart(&mut self) -> LPUART_W {
        LPUART_W { w: self }
    }
    #[doc = "Bit 11 - SAR ADC Clock Gate Control"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 13 - PIT0 Clock Gate Control"]
    #[inline(always)]
    pub fn pit0(&mut self) -> PIT0_W {
        PIT0_W { w: self }
    }
    #[doc = "Bit 14 - PIT1 Clock Gate Control"]
    #[inline(always)]
    pub fn pit1(&mut self) -> PIT1_W {
        PIT1_W { w: self }
    }
    #[doc = "Bit 16 - AFE Clock Gate Control"]
    #[inline(always)]
    pub fn afe(&mut self) -> AFE_W {
        AFE_W { w: self }
    }
    #[doc = "Bit 20 - Programmable CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&mut self) -> PDB_W {
        PDB_W { w: self }
    }
    #[doc = "Bit 23 - PCTLJ Clock Gate Control"]
    #[inline(always)]
    pub fn portj(&mut self) -> PORTJ_W {
        PORTJ_W { w: self }
    }
    #[doc = "Bit 24 - PCTLK Clock Gate Control"]
    #[inline(always)]
    pub fn portk(&mut self) -> PORTK_W {
        PORTK_W { w: self }
    }
    #[doc = "Bit 25 - PCTLL Clock Gate Control"]
    #[inline(always)]
    pub fn portl(&mut self) -> PORTL_W {
        PORTL_W { w: self }
    }
    #[doc = "Bit 26 - PCTLM Clock Gate Control"]
    #[inline(always)]
    pub fn portm(&mut self) -> PORTM_W {
        PORTM_W { w: self }
    }
    #[doc = "Bit 28 - LPTMR Clock Gate Control"]
    #[inline(always)]
    pub fn lptmr(&mut self) -> LPTMR_W {
        LPTMR_W { w: self }
    }
}
