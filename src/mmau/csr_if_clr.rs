#[doc = "Reader of register CSR_IF_CLR"]
pub type R = crate::R<u32, super::CSR_IF_CLR>;
#[doc = "Writer for register CSR_IF_CLR"]
pub type W = crate::W<u32, super::CSR_IF_CLR>;
#[doc = "Register CSR_IF_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR_IF_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Q flag: Accumulation Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Q_AW {
    #[doc = "0: Write \"0\" will clear CSR\\[Q\\]"]
    _0 = 0,
    #[doc = "1: Write \"1\" will set CSR\\[Q\\]"]
    _1 = 1,
}
impl From<Q_AW> for bool {
    #[inline(always)]
    fn from(variant: Q_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `Q`"]
pub struct Q_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: Q_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" will clear CSR\\[Q\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(Q_AW::_0)
    }
    #[doc = "Write \"1\" will set CSR\\[Q\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(Q_AW::_1)
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
pub enum V_AW {
    #[doc = "0: Write \"0\" will clear CSR\\[V\\]"]
    _0 = 0,
    #[doc = "1: Write \"1\" will set CSR\\[V\\]"]
    _1 = 1,
}
impl From<V_AW> for bool {
    #[inline(always)]
    fn from(variant: V_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `V`"]
pub struct V_W<'a> {
    w: &'a mut W,
}
impl<'a> V_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: V_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" will clear CSR\\[V\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(V_AW::_0)
    }
    #[doc = "Write \"1\" will set CSR\\[V\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(V_AW::_1)
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
pub enum DZ_AW {
    #[doc = "0: Write \"0\" will clear CSR\\[DZ\\]"]
    _0 = 0,
    #[doc = "1: Write \"1\" will set CSR\\[DZ\\]"]
    _1 = 1,
}
impl From<DZ_AW> for bool {
    #[inline(always)]
    fn from(variant: DZ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DZ`"]
pub struct DZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DZ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" will clear CSR\\[DZ\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DZ_AW::_0)
    }
    #[doc = "Write \"1\" will set CSR\\[DZ\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DZ_AW::_1)
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
pub enum N_AW {
    #[doc = "0: Write \"0\" to clear CSR\\[N\\]"]
    _0 = 0,
    #[doc = "1: Write \"1\" to set CSR\\[N\\]"]
    _1 = 1,
}
impl From<N_AW> for bool {
    #[inline(always)]
    fn from(variant: N_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `N`"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: N_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" to clear CSR\\[N\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(N_AW::_0)
    }
    #[doc = "Write \"1\" to set CSR\\[N\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(N_AW::_1)
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
pub enum QIF_AW {
    #[doc = "0: Write \"0\" is ignored"]
    _0 = 0,
    #[doc = "1: Write \"1\" to clear CSR\\[QIF\\]"]
    _1 = 1,
}
impl From<QIF_AW> for bool {
    #[inline(always)]
    fn from(variant: QIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `QIF`"]
pub struct QIF_W<'a> {
    w: &'a mut W,
}
impl<'a> QIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" is ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QIF_AW::_0)
    }
    #[doc = "Write \"1\" to clear CSR\\[QIF\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QIF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "V Interrupt Flag: Multiply or Divide overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIF_AW {
    #[doc = "0: Write \"0\" is ignored"]
    _0 = 0,
    #[doc = "1: Write \"1\" to clear CSR\\[VIF\\]"]
    _1 = 1,
}
impl From<VIF_AW> for bool {
    #[inline(always)]
    fn from(variant: VIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VIF`"]
pub struct VIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" is ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VIF_AW::_0)
    }
    #[doc = "Write \"1\" to clear CSR\\[VIF\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VIF_AW::_1)
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
#[doc = "DZ Interrupt Flag: Divide by Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DZIF_AW {
    #[doc = "0: Write \"0\" is ignored"]
    _0 = 0,
    #[doc = "1: Write \"1\" to clear CSR\\[DZIF\\]"]
    _1 = 1,
}
impl From<DZIF_AW> for bool {
    #[inline(always)]
    fn from(variant: DZIF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DZIF`"]
pub struct DZIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DZIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DZIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write \"0\" is ignored"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DZIF_AW::_0)
    }
    #[doc = "Write \"1\" to clear CSR\\[DZIF\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DZIF_AW::_1)
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
impl R {}
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
    #[doc = "Bit 4 - Q Interrupt Flag: Accumulation Overflow Interrupt Status"]
    #[inline(always)]
    pub fn qif(&mut self) -> QIF_W {
        QIF_W { w: self }
    }
    #[doc = "Bit 5 - V Interrupt Flag: Multiply or Divide overflow"]
    #[inline(always)]
    pub fn vif(&mut self) -> VIF_W {
        VIF_W { w: self }
    }
    #[doc = "Bit 6 - DZ Interrupt Flag: Divide by Zero"]
    #[inline(always)]
    pub fn dzif(&mut self) -> DZIF_W {
        DZIF_W { w: self }
    }
}
