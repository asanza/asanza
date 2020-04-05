#[doc = "Reader of register CTRL0"]
pub type R = crate::R<u16, super::CTRL0>;
#[doc = "Writer for register CTRL0"]
pub type W = crate::W<u16, super::CTRL0>;
#[doc = "Register CTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Enable for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN0_A {
    #[doc = "0: DMA disabled"]
    _0 = 0,
    #[doc = "1: DMA enabled"]
    _1 = 1,
}
impl From<DEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEN0`"]
pub type DEN0_R = crate::R<bool, DEN0_A>;
impl DEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN0_A {
        match self.bits {
            false => DEN0_A::_0,
            true => DEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEN0_A::_1
    }
}
#[doc = "Write proxy for field `DEN0`"]
pub struct DEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEN0_A::_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEN0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Interrupt Enable for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN0_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<IEN0_A> for bool {
    #[inline(always)]
    fn from(variant: IEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEN0`"]
pub type IEN0_R = crate::R<bool, IEN0_A>;
impl IEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN0_A {
        match self.bits {
            false => IEN0_A::_0,
            true => IEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEN0_A::_1
    }
}
#[doc = "Write proxy for field `IEN0`"]
pub struct IEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEN0_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Active edge for edge detection on XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE0_A {
    #[doc = "0: STS0 never asserts"]
    _00 = 0,
    #[doc = "1: STS0 asserts on rising edges of XBAR_OUT0"]
    _01 = 1,
    #[doc = "2: STS0 asserts on falling edges of XBAR_OUT0"]
    _10 = 2,
    #[doc = "3: STS0 asserts on rising and falling edges of XBAR_OUT0"]
    _11 = 3,
}
impl From<EDGE0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE0`"]
pub type EDGE0_R = crate::R<u8, EDGE0_A>;
impl EDGE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE0_A {
        match self.bits {
            0 => EDGE0_A::_00,
            1 => EDGE0_A::_01,
            2 => EDGE0_A::_10,
            3 => EDGE0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EDGE0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EDGE0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EDGE0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EDGE0_A::_11
    }
}
#[doc = "Write proxy for field `EDGE0`"]
pub struct EDGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "STS0 never asserts"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EDGE0_A::_00)
    }
    #[doc = "STS0 asserts on rising edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EDGE0_A::_01)
    }
    #[doc = "STS0 asserts on falling edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EDGE0_A::_10)
    }
    #[doc = "STS0 asserts on rising and falling edges of XBAR_OUT0"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EDGE0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge detection status for XBAR_OUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS0_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT0"]
    _0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT0"]
    _1 = 1,
}
impl From<STS0_A> for bool {
    #[inline(always)]
    fn from(variant: STS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STS0`"]
pub type STS0_R = crate::R<bool, STS0_A>;
impl STS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS0_A {
        match self.bits {
            false => STS0_A::_0,
            true => STS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STS0_A::_1
    }
}
#[doc = "Write proxy for field `STS0`"]
pub struct STS0_W<'a> {
    w: &'a mut W,
}
impl<'a> STS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STS0_A::_0)
    }
    #[doc = "Active edge detected on XBAR_OUT0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STS0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA Enable for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEN1_A {
    #[doc = "0: DMA disabled"]
    _0 = 0,
    #[doc = "1: DMA enabled"]
    _1 = 1,
}
impl From<DEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEN1`"]
pub type DEN1_R = crate::R<bool, DEN1_A>;
impl DEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN1_A {
        match self.bits {
            false => DEN1_A::_0,
            true => DEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEN1_A::_1
    }
}
#[doc = "Write proxy for field `DEN1`"]
pub struct DEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEN1_A::_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Interrupt Enable for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEN1_A {
    #[doc = "0: Interrupt disabled"]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<IEN1_A> for bool {
    #[inline(always)]
    fn from(variant: IEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IEN1`"]
pub type IEN1_R = crate::R<bool, IEN1_A>;
impl IEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN1_A {
        match self.bits {
            false => IEN1_A::_0,
            true => IEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEN1_A::_1
    }
}
#[doc = "Write proxy for field `IEN1`"]
pub struct IEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEN1_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Active edge for edge detection on XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE1_A {
    #[doc = "0: STS1 never asserts"]
    _00 = 0,
    #[doc = "1: STS1 asserts on rising edges of XBAR_OUT1"]
    _01 = 1,
    #[doc = "2: STS1 asserts on falling edges of XBAR_OUT1"]
    _10 = 2,
    #[doc = "3: STS1 asserts on rising and falling edges of XBAR_OUT1"]
    _11 = 3,
}
impl From<EDGE1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE1`"]
pub type EDGE1_R = crate::R<u8, EDGE1_A>;
impl EDGE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE1_A {
        match self.bits {
            0 => EDGE1_A::_00,
            1 => EDGE1_A::_01,
            2 => EDGE1_A::_10,
            3 => EDGE1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EDGE1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EDGE1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EDGE1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EDGE1_A::_11
    }
}
#[doc = "Write proxy for field `EDGE1`"]
pub struct EDGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "STS1 never asserts"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EDGE1_A::_00)
    }
    #[doc = "STS1 asserts on rising edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EDGE1_A::_01)
    }
    #[doc = "STS1 asserts on falling edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EDGE1_A::_10)
    }
    #[doc = "STS1 asserts on rising and falling edges of XBAR_OUT1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EDGE1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Edge detection status for XBAR_OUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STS1_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT1"]
    _0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT1"]
    _1 = 1,
}
impl From<STS1_A> for bool {
    #[inline(always)]
    fn from(variant: STS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STS1`"]
pub type STS1_R = crate::R<bool, STS1_A>;
impl STS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS1_A {
        match self.bits {
            false => STS1_A::_0,
            true => STS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STS1_A::_1
    }
}
#[doc = "Write proxy for field `STS1`"]
pub struct STS1_W<'a> {
    w: &'a mut W,
}
impl<'a> STS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active edge not yet detected on XBAR_OUT1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STS1_A::_0)
    }
    #[doc = "Active edge detected on XBAR_OUT1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STS1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn den0(&self) -> DEN0_R {
        DEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn ien0(&self) -> IEN0_R {
        IEN0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    pub fn sts0(&self) -> STS0_R {
        STS0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn den1(&self) -> DEN1_R {
        DEN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn ien1(&self) -> IEN1_R {
        IEN1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    pub fn sts1(&self) -> STS1_R {
        STS1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn den0(&mut self) -> DEN0_W {
        DEN0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT0"]
    #[inline(always)]
    pub fn ien0(&mut self) -> IEN0_W {
        IEN0_W { w: self }
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT0"]
    #[inline(always)]
    pub fn edge0(&mut self) -> EDGE0_W {
        EDGE0_W { w: self }
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT0"]
    #[inline(always)]
    pub fn sts0(&mut self) -> STS0_W {
        STS0_W { w: self }
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn den1(&mut self) -> DEN1_W {
        DEN1_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT1"]
    #[inline(always)]
    pub fn ien1(&mut self) -> IEN1_W {
        IEN1_W { w: self }
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT1"]
    #[inline(always)]
    pub fn edge1(&mut self) -> EDGE1_W {
        EDGE1_W { w: self }
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT1"]
    #[inline(always)]
    pub fn sts1(&mut self) -> STS1_W {
        STS1_W { w: self }
    }
}
