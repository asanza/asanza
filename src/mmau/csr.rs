#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Q flag: Accumulation Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Q_A {
    #[doc = "0: No accumulation operation or accumulation operation does not overflow"]
    _0 = 0,
    #[doc = "1: Accumulation overflows during a MAC instruction"]
    _1 = 1,
}
impl From<Q_A> for bool {
    #[inline(always)]
    fn from(variant: Q_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `Q`"]
pub type Q_R = crate::R<bool, Q_A>;
impl Q_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Q_A {
        match self.bits {
            false => Q_A::_0,
            true => Q_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Q_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Q_A::_1
    }
}
#[doc = "Write proxy for field `Q`"]
pub struct Q_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Q_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No accumulation operation or accumulation operation does not overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(Q_A::_0)
    }
    #[doc = "Accumulation overflows during a MAC instruction"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(Q_A::_1)
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
#[doc = "V flag: Multiply or Divide overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: Calculation is not include divider or multiply, or the product/quotient does not overflow"]
    _0 = 0,
    #[doc = "1: Product in multiply or multiply with accumulation, or quotient of a divide overflows"]
    _1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::_0,
            true => V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == V_A::_1
    }
}
#[doc = "Write proxy for field `V`"]
pub struct V_W<'a> {
    w: &'a mut W,
}
impl<'a> V_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: V_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calculation is not include divider or multiply, or the product/quotient does not overflow"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(V_A::_0)
    }
    #[doc = "Product in multiply or multiply with accumulation, or quotient of a divide overflows"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(V_A::_1)
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
#[doc = "DZ flag: Divide by Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DZ_A {
    #[doc = "0: For divide, the divisor is not zero, or the calculation is not divide"]
    _0 = 0,
    #[doc = "1: For divide, the divisor is zero, which is a divide-by-zero error"]
    _1 = 1,
}
impl From<DZ_A> for bool {
    #[inline(always)]
    fn from(variant: DZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DZ`"]
pub type DZ_R = crate::R<bool, DZ_A>;
impl DZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DZ_A {
        match self.bits {
            false => DZ_A::_0,
            true => DZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DZ_A::_1
    }
}
#[doc = "Write proxy for field `DZ`"]
pub struct DZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "For divide, the divisor is not zero, or the calculation is not divide"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DZ_A::_0)
    }
    #[doc = "For divide, the divisor is zero, which is a divide-by-zero error"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "N flag: Signed calculation result is negative\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum N_A {
    #[doc = "0: Calculation raw result is zero or positive, or unsigned number"]
    _0 = 0,
    #[doc = "1: Calculation raw result is negative"]
    _1 = 1,
}
impl From<N_A> for bool {
    #[inline(always)]
    fn from(variant: N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<bool, N_A>;
impl N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_A {
        match self.bits {
            false => N_A::_0,
            true => N_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == N_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == N_A::_1
    }
}
#[doc = "Write proxy for field `N`"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calculation raw result is zero or positive, or unsigned number"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(N_A::_0)
    }
    #[doc = "Calculation raw result is negative"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(N_A::_1)
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
#[doc = "Q Interrupt Flag: Accumulation Overflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QIF_A {
    #[doc = "0: No accumulation operation or accumulation operation does not overflow"]
    _0 = 0,
    #[doc = "1: Accumulation overflows during a MAC instruction"]
    _1 = 1,
}
impl From<QIF_A> for bool {
    #[inline(always)]
    fn from(variant: QIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QIF`"]
pub type QIF_R = crate::R<bool, QIF_A>;
impl QIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QIF_A {
        match self.bits {
            false => QIF_A::_0,
            true => QIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QIF_A::_1
    }
}
#[doc = "V Interrupt Flag: Multiply or Divide overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIF_A {
    #[doc = "0: Calculation is not include divider or multiply, or the product/quotient does not overflow"]
    _0 = 0,
    #[doc = "1: Product in multiply or multiply with accumulation, or quotient of a divide overflows"]
    _1 = 1,
}
impl From<VIF_A> for bool {
    #[inline(always)]
    fn from(variant: VIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIF`"]
pub type VIF_R = crate::R<bool, VIF_A>;
impl VIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIF_A {
        match self.bits {
            false => VIF_A::_0,
            true => VIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VIF_A::_1
    }
}
#[doc = "DZ Interrupt Flag: Divide by Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DZIF_A {
    #[doc = "0: For divide, the divisor is not zero, or the calculation is not divide"]
    _0 = 0,
    #[doc = "1: For divide, the divisor is zero, which is a divide-by-zero error"]
    _1 = 1,
}
impl From<DZIF_A> for bool {
    #[inline(always)]
    fn from(variant: DZIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DZIF`"]
pub type DZIF_R = crate::R<bool, DZIF_A>;
impl DZIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DZIF_A {
        match self.bits {
            false => DZIF_A::_0,
            true => DZIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DZIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DZIF_A::_1
    }
}
#[doc = "Accumulation Overflow (Q flag) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QIE_A {
    #[doc = "0: Q flag (CSR\\[QIF\\]) set will not generate interrupt."]
    _0 = 0,
    #[doc = "1: Q flag (CSR\\[QIF\\]) set will generate interrupt to inform an accumulation overflow"]
    _1 = 1,
}
impl From<QIE_A> for bool {
    #[inline(always)]
    fn from(variant: QIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QIE`"]
pub type QIE_R = crate::R<bool, QIE_A>;
impl QIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QIE_A {
        match self.bits {
            false => QIE_A::_0,
            true => QIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QIE_A::_1
    }
}
#[doc = "Write proxy for field `QIE`"]
pub struct QIE_W<'a> {
    w: &'a mut W,
}
impl<'a> QIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Q flag (CSR\\[QIF\\]) set will not generate interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QIE_A::_0)
    }
    #[doc = "Q flag (CSR\\[QIF\\]) set will generate interrupt to inform an accumulation overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Divide/Multiply Overflow (V flag) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIE_A {
    #[doc = "0: V flag (CSR\\[VIF\\]) set will not generate interrupt."]
    _0 = 0,
    #[doc = "1: V flag (CSR\\[VIF\\]) set will generate interrupt to inform a divide or multiply overflow"]
    _1 = 1,
}
impl From<VIE_A> for bool {
    #[inline(always)]
    fn from(variant: VIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VIE`"]
pub type VIE_R = crate::R<bool, VIE_A>;
impl VIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIE_A {
        match self.bits {
            false => VIE_A::_0,
            true => VIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VIE_A::_1
    }
}
#[doc = "Write proxy for field `VIE`"]
pub struct VIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "V flag (CSR\\[VIF\\]) set will not generate interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VIE_A::_0)
    }
    #[doc = "V flag (CSR\\[VIF\\]) set will generate interrupt to inform a divide or multiply overflow"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VIE_A::_1)
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
#[doc = "Divide-by-Zero Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DZIE_A {
    #[doc = "0: MMAU will not generate interrupt even CSR\\[DZIF\\]=1"]
    _0 = 0,
    #[doc = "1: If CSR\\[DZIF\\]
= 1, MMAU will generate an interrupt to signal a divide-by-zero."]
    _1 = 1,
}
impl From<DZIE_A> for bool {
    #[inline(always)]
    fn from(variant: DZIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DZIE`"]
pub type DZIE_R = crate::R<bool, DZIE_A>;
impl DZIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DZIE_A {
        match self.bits {
            false => DZIE_A::_0,
            true => DZIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DZIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DZIE_A::_1
    }
}
#[doc = "Write proxy for field `DZIE`"]
pub struct DZIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DZIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DZIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MMAU will not generate interrupt even CSR\\[DZIF\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DZIE_A::_0)
    }
    #[doc = "If CSR\\[DZIF\\]
= 1, MMAU will generate an interrupt to signal a divide-by-zero."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DZIE_A::_1)
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
#[doc = "DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRE_A {
    #[doc = "0: MMAU will not generate DMA request when in IDLE (not busy) state"]
    _0 = 0,
    #[doc = "1: MMAU will generate DMA request when in IDLE (not busy) state"]
    _1 = 1,
}
impl From<DRE_A> for bool {
    #[inline(always)]
    fn from(variant: DRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRE`"]
pub type DRE_R = crate::R<bool, DRE_A>;
impl DRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRE_A {
        match self.bits {
            false => DRE_A::_0,
            true => DRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRE_A::_1
    }
}
#[doc = "Write proxy for field `DRE`"]
pub struct DRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MMAU will not generate DMA request when in IDLE (not busy) state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRE_A::_0)
    }
    #[doc = "MMAU will generate DMA request when in IDLE (not busy) state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRE_A::_1)
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
#[doc = "Supervisor-Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SO_A {
    #[doc = "0: MMAU registers can be access in both User Mode or Supervisor Mode"]
    _0 = 0,
    #[doc = "1: MMAU registers can only be access in Supervisor Mode"]
    _1 = 1,
}
impl From<SO_A> for bool {
    #[inline(always)]
    fn from(variant: SO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SO`"]
pub type SO_R = crate::R<bool, SO_A>;
impl SO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SO_A {
        match self.bits {
            false => SO_A::_0,
            true => SO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SO_A::_1
    }
}
#[doc = "Write proxy for field `SO`"]
pub struct SO_W<'a> {
    w: &'a mut W,
}
impl<'a> SO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MMAU registers can be access in both User Mode or Supervisor Mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SO_A::_0)
    }
    #[doc = "MMAU registers can only be access in Supervisor Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Hardware Revision Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HDR_A {
    #[doc = "0: Current Hardware Revision Level is 0000"]
    _0000 = 0,
}
impl From<HDR_A> for u8 {
    #[inline(always)]
    fn from(variant: HDR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HDR`"]
pub type HDR_R = crate::R<u8, HDR_A>;
impl HDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HDR_A::_0000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == HDR_A::_0000
    }
}
#[doc = "BUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: MMAU is idle"]
    _0 = 0,
    #[doc = "1: MMAU is busy performing a divide or square root calculation"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSY_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Q flag: Accumulation Overflow"]
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - V flag: Multiply or Divide overflow"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DZ flag: Divide by Zero"]
    #[inline(always)]
    pub fn dz(&self) -> DZ_R {
        DZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N flag: Signed calculation result is negative"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Q Interrupt Flag: Accumulation Overflow Interrupt Status"]
    #[inline(always)]
    pub fn qif(&self) -> QIF_R {
        QIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - V Interrupt Flag: Multiply or Divide overflow"]
    #[inline(always)]
    pub fn vif(&self) -> VIF_R {
        VIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DZ Interrupt Flag: Divide by Zero"]
    #[inline(always)]
    pub fn dzif(&self) -> DZIF_R {
        DZIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Accumulation Overflow (Q flag) Interrupt Enable"]
    #[inline(always)]
    pub fn qie(&self) -> QIE_R {
        QIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Divide/Multiply Overflow (V flag) Interrupt Enable"]
    #[inline(always)]
    pub fn vie(&self) -> VIE_R {
        VIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub fn dzie(&self) -> DZIE_R {
        DZIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA Request Enable"]
    #[inline(always)]
    pub fn dre(&self) -> DRE_R {
        DRE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Supervisor-Only"]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Hardware Revision Level"]
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Q flag: Accumulation Overflow"]
    #[inline(always)]
    pub fn q(&mut self) -> Q_W {
        Q_W { w: self }
    }
    #[doc = "Bit 1 - V flag: Multiply or Divide overflow"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W {
        V_W { w: self }
    }
    #[doc = "Bit 2 - DZ flag: Divide by Zero"]
    #[inline(always)]
    pub fn dz(&mut self) -> DZ_W {
        DZ_W { w: self }
    }
    #[doc = "Bit 3 - N flag: Signed calculation result is negative"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W { w: self }
    }
    #[doc = "Bit 12 - Accumulation Overflow (Q flag) Interrupt Enable"]
    #[inline(always)]
    pub fn qie(&mut self) -> QIE_W {
        QIE_W { w: self }
    }
    #[doc = "Bit 13 - Divide/Multiply Overflow (V flag) Interrupt Enable"]
    #[inline(always)]
    pub fn vie(&mut self) -> VIE_W {
        VIE_W { w: self }
    }
    #[doc = "Bit 14 - Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub fn dzie(&mut self) -> DZIE_W {
        DZIE_W { w: self }
    }
    #[doc = "Bit 16 - DMA Request Enable"]
    #[inline(always)]
    pub fn dre(&mut self) -> DRE_W {
        DRE_W { w: self }
    }
    #[doc = "Bit 17 - Supervisor-Only"]
    #[inline(always)]
    pub fn so(&mut self) -> SO_W {
        SO_W { w: self }
    }
}
