#[doc = "Reader of register GACR"]
pub type R = crate::R<u8, super::GACR>;
#[doc = "Writer for register GACR"]
pub type W = crate::W<u8, super::GACR>;
#[doc = "Register GACR `reset()`'s with value 0"]
impl crate::ResetValue for super::GACR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Attribute Check Byte 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACB_A {
    #[doc = "0: User nonsecure: Read + Write; User Secure: Read + Write; Privileged Secure: Read + Write"]
    _000 = 0,
    #[doc = "1: User nonsecure: Read; User Secure: Read + Write; Privileged Secure: Read + Write"]
    _001 = 1,
    #[doc = "2: User nonsecure: None; User Secure: Read + Write; Privileged Secure: Read + Write"]
    _010 = 2,
    #[doc = "3: User nonsecure: Read; User Secure: Read; Privileged Secure: Read + Write"]
    _011 = 3,
    #[doc = "4: User nonsecure: None; User Secure: Read; Privileged Secure: Read + Write"]
    _100 = 4,
    #[doc = "5: User nonsecure: None; User Secure: None; Privileged Secure: Read + Write"]
    _101 = 5,
    #[doc = "6: User nonsecure: None; User Secure: None; Privileged Secure: Read"]
    _110 = 6,
    #[doc = "7: User nonsecure: None; User Secure: None; Privileged Secure: None"]
    _111 = 7,
}
impl From<ACB_A> for u8 {
    #[inline(always)]
    fn from(variant: ACB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACB`"]
pub type ACB_R = crate::R<u8, ACB_A>;
impl ACB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACB_A {
        match self.bits {
            0 => ACB_A::_000,
            1 => ACB_A::_001,
            2 => ACB_A::_010,
            3 => ACB_A::_011,
            4 => ACB_A::_100,
            5 => ACB_A::_101,
            6 => ACB_A::_110,
            7 => ACB_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ACB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ACB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ACB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ACB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ACB_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ACB_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == ACB_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == ACB_A::_111
    }
}
#[doc = "Write proxy for field `ACB`"]
pub struct ACB_W<'a> {
    w: &'a mut W,
}
impl<'a> ACB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "User nonsecure: Read + Write; User Secure: Read + Write; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ACB_A::_000)
    }
    #[doc = "User nonsecure: Read; User Secure: Read + Write; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ACB_A::_001)
    }
    #[doc = "User nonsecure: None; User Secure: Read + Write; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ACB_A::_010)
    }
    #[doc = "User nonsecure: Read; User Secure: Read; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ACB_A::_011)
    }
    #[doc = "User nonsecure: None; User Secure: Read; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ACB_A::_100)
    }
    #[doc = "User nonsecure: None; User Secure: None; Privileged Secure: Read + Write"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ACB_A::_101)
    }
    #[doc = "User nonsecure: None; User Secure: None; Privileged Secure: Read"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(ACB_A::_110)
    }
    #[doc = "User nonsecure: None; User Secure: None; Privileged Secure: None"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(ACB_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Read-Only Byte 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROB_A {
    #[doc = "0: Writes to the GPIOn_GACR are allowed."]
    _0 = 0,
    #[doc = "1: Writes to the GPIOn_GACR are ignored."]
    _1 = 1,
}
impl From<ROB_A> for bool {
    #[inline(always)]
    fn from(variant: ROB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROB`"]
pub type ROB_R = crate::R<bool, ROB_A>;
impl ROB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROB_A {
        match self.bits {
            false => ROB_A::_0,
            true => ROB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROB_A::_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Attribute Check Byte 3"]
    #[inline(always)]
    pub fn acb(&self) -> ACB_R {
        ACB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Read-Only Byte 3"]
    #[inline(always)]
    pub fn rob(&self) -> ROB_R {
        ROB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Attribute Check Byte 3"]
    #[inline(always)]
    pub fn acb(&mut self) -> ACB_W {
        ACB_W { w: self }
    }
}
