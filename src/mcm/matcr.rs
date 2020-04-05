#[doc = "Reader of register MATCR"]
pub type R = crate::R<u32, super::MATCR>;
#[doc = "Writer for register MATCR"]
pub type W = crate::W<u32, super::MATCR>;
#[doc = "Register MATCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MATCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Attribute Configuration Master n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATC0_A {
    #[doc = "2: Master attributes are statically forced to {user, secure}."]
    _010 = 2,
    #[doc = "3: Master attributes are statically forced to {user, nonsecure}."]
    _011 = 3,
    #[doc = "4: Enable master attribute {privileged or user} and statically force {secure}."]
    _100 = 4,
    #[doc = "5: Enable master attribute {privileged or user} and statically force {nonsecure}."]
    _101 = 5,
}
impl From<ATC0_A> for u8 {
    #[inline(always)]
    fn from(variant: ATC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ATC0`"]
pub type ATC0_R = crate::R<u8, ATC0_A>;
impl ATC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ATC0_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(ATC0_A::_010),
            3 => Val(ATC0_A::_011),
            4 => Val(ATC0_A::_100),
            5 => Val(ATC0_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ATC0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ATC0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ATC0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ATC0_A::_101
    }
}
#[doc = "Write proxy for field `ATC0`"]
pub struct ATC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ATC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master attributes are statically forced to {user, secure}."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ATC0_A::_010)
    }
    #[doc = "Master attributes are statically forced to {user, nonsecure}."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ATC0_A::_011)
    }
    #[doc = "Enable master attribute {privileged or user} and statically force {secure}."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ATC0_A::_100)
    }
    #[doc = "Enable master attribute {privileged or user} and statically force {nonsecure}."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ATC0_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Read-Only Master n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO0_A {
    #[doc = "0: Writes to the ATCn are allowed."]
    _0 = 0,
    #[doc = "1: Writes to the ATCn are ignored."]
    _1 = 1,
}
impl From<RO0_A> for bool {
    #[inline(always)]
    fn from(variant: RO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO0`"]
pub type RO0_R = crate::R<bool, RO0_A>;
impl RO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO0_A {
        match self.bits {
            false => RO0_A::_0,
            true => RO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO0_A::_1
    }
}
#[doc = "Write proxy for field `RO0`"]
pub struct RO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to the ATCn are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO0_A::_0)
    }
    #[doc = "Writes to the ATCn are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO0_A::_1)
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
#[doc = "Attribute Configuration Master n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATC2_A {
    #[doc = "2: Master attributes are statically forced to {user, secure}."]
    _010 = 2,
    #[doc = "3: Master attributes are statically forced to {user, nonsecure}."]
    _011 = 3,
    #[doc = "4: Enable master attribute {privileged or user} and statically force {secure}."]
    _100 = 4,
    #[doc = "5: Enable master attribute {privileged or user} and statically force {nonsecure}."]
    _101 = 5,
}
impl From<ATC2_A> for u8 {
    #[inline(always)]
    fn from(variant: ATC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ATC2`"]
pub type ATC2_R = crate::R<u8, ATC2_A>;
impl ATC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ATC2_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(ATC2_A::_010),
            3 => Val(ATC2_A::_011),
            4 => Val(ATC2_A::_100),
            5 => Val(ATC2_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ATC2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ATC2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ATC2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ATC2_A::_101
    }
}
#[doc = "Write proxy for field `ATC2`"]
pub struct ATC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master attributes are statically forced to {user, secure}."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ATC2_A::_010)
    }
    #[doc = "Master attributes are statically forced to {user, nonsecure}."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ATC2_A::_011)
    }
    #[doc = "Enable master attribute {privileged or user} and statically force {secure}."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ATC2_A::_100)
    }
    #[doc = "Enable master attribute {privileged or user} and statically force {nonsecure}."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ATC2_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Read-Only Master n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO2_A {
    #[doc = "0: Writes to the ATCn are allowed."]
    _0 = 0,
    #[doc = "1: Writes to the ATCn are ignored."]
    _1 = 1,
}
impl From<RO2_A> for bool {
    #[inline(always)]
    fn from(variant: RO2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO2`"]
pub type RO2_R = crate::R<bool, RO2_A>;
impl RO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO2_A {
        match self.bits {
            false => RO2_A::_0,
            true => RO2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO2_A::_1
    }
}
#[doc = "Write proxy for field `RO2`"]
pub struct RO2_W<'a> {
    w: &'a mut W,
}
impl<'a> RO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to the ATCn are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO2_A::_0)
    }
    #[doc = "Writes to the ATCn are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO2_A::_1)
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
impl R {
    #[doc = "Bits 0:2 - Attribute Configuration Master n"]
    #[inline(always)]
    pub fn atc0(&self) -> ATC0_R {
        ATC0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Read-Only Master n"]
    #[inline(always)]
    pub fn ro0(&self) -> RO0_R {
        RO0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Attribute Configuration Master n"]
    #[inline(always)]
    pub fn atc2(&self) -> ATC2_R {
        ATC2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Read-Only Master n"]
    #[inline(always)]
    pub fn ro2(&self) -> RO2_R {
        RO2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Attribute Configuration Master n"]
    #[inline(always)]
    pub fn atc0(&mut self) -> ATC0_W {
        ATC0_W { w: self }
    }
    #[doc = "Bit 7 - Read-Only Master n"]
    #[inline(always)]
    pub fn ro0(&mut self) -> RO0_W {
        RO0_W { w: self }
    }
    #[doc = "Bits 16:18 - Attribute Configuration Master n"]
    #[inline(always)]
    pub fn atc2(&mut self) -> ATC2_W {
        ATC2_W { w: self }
    }
    #[doc = "Bit 23 - Read-Only Master n"]
    #[inline(always)]
    pub fn ro2(&mut self) -> RO2_W {
        RO2_W { w: self }
    }
}
