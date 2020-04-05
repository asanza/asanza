#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "AFE Ready3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY3_A {
    #[doc = "0: AFE Channel3 is disabled or has not completed its start up period"]
    _0 = 0,
    #[doc = "1: AFE Channel3 is ready to initiate conversions."]
    _1 = 1,
}
impl From<RDY3_A> for bool {
    #[inline(always)]
    fn from(variant: RDY3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY3`"]
pub type RDY3_R = crate::R<bool, RDY3_A>;
impl RDY3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY3_A {
        match self.bits {
            false => RDY3_A::_0,
            true => RDY3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDY3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDY3_A::_1
    }
}
#[doc = "AFE Ready2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY2_A {
    #[doc = "0: AFE Channel2 is disabled or has not completed its start up period"]
    _0 = 0,
    #[doc = "1: AFE Channel2 is ready to initiate conversions."]
    _1 = 1,
}
impl From<RDY2_A> for bool {
    #[inline(always)]
    fn from(variant: RDY2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY2`"]
pub type RDY2_R = crate::R<bool, RDY2_A>;
impl RDY2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY2_A {
        match self.bits {
            false => RDY2_A::_0,
            true => RDY2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDY2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDY2_A::_1
    }
}
#[doc = "AFE Ready1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY1_A {
    #[doc = "0: AFE Channel1 is disabled or has not completed its start up period"]
    _0 = 0,
    #[doc = "1: AFE Channel1 is ready to initiate conversions."]
    _1 = 1,
}
impl From<RDY1_A> for bool {
    #[inline(always)]
    fn from(variant: RDY1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY1`"]
pub type RDY1_R = crate::R<bool, RDY1_A>;
impl RDY1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY1_A {
        match self.bits {
            false => RDY1_A::_0,
            true => RDY1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDY1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDY1_A::_1
    }
}
#[doc = "AFE Ready0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY0_A {
    #[doc = "0: AFE Channel0 is disabled or has not completed its start up period"]
    _0 = 0,
    #[doc = "1: AFE Channel0 is ready to initiate conversions."]
    _1 = 1,
}
impl From<RDY0_A> for bool {
    #[inline(always)]
    fn from(variant: RDY0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDY0`"]
pub type RDY0_R = crate::R<bool, RDY0_A>;
impl RDY0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY0_A {
        match self.bits {
            false => RDY0_A::_0,
            true => RDY0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDY0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDY0_A::_1
    }
}
#[doc = "Reader of field `OVR3`"]
pub type OVR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR2`"]
pub type OVR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR1`"]
pub type OVR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR0`"]
pub type OVR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `COC3`"]
pub type COC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `COC2`"]
pub type COC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `COC1`"]
pub type COC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `COC0`"]
pub type COC0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - AFE Ready3"]
    #[inline(always)]
    pub fn rdy3(&self) -> RDY3_R {
        RDY3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AFE Ready2"]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY2_R {
        RDY2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AFE Ready1"]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY1_R {
        RDY1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - AFE Ready0"]
    #[inline(always)]
    pub fn rdy0(&self) -> RDY0_R {
        RDY0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Overflow Flag"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Overflow Flag"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Overflow Flag"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Overflow Flag"]
    #[inline(always)]
    pub fn ovr0(&self) -> OVR0_R {
        OVR0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Conversion Complete"]
    #[inline(always)]
    pub fn coc3(&self) -> COC3_R {
        COC3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Conversion Complete"]
    #[inline(always)]
    pub fn coc2(&self) -> COC2_R {
        COC2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Conversion Complete"]
    #[inline(always)]
    pub fn coc1(&self) -> COC1_R {
        COC1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Conversion Complete"]
    #[inline(always)]
    pub fn coc0(&self) -> COC0_R {
        COC0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
