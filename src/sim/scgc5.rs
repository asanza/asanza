#[doc = "Reader of register SCGC5"]
pub type R = crate::R<u32, super::SCGC5>;
#[doc = "Writer for register SCGC5"]
pub type W = crate::W<u32, super::SCGC5>;
#[doc = "Register SCGC5 `reset()`'s with value 0x000b_0000"]
impl crate::ResetValue for super::SCGC5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000b_0000
    }
}
#[doc = "Segmented LCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLCD_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SLCD_A> for bool {
    #[inline(always)]
    fn from(variant: SLCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLCD`"]
pub type SLCD_R = crate::R<bool, SLCD_A>;
impl SLCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLCD_A {
        match self.bits {
            false => SLCD_A::_0,
            true => SLCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLCD_A::_1
    }
}
#[doc = "Write proxy for field `SLCD`"]
pub struct SLCD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLCD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLCD_A::_1)
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
#[doc = "PCTLA Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTA_A> for bool {
    #[inline(always)]
    fn from(variant: PORTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTA`"]
pub type PORTA_R = crate::R<bool, PORTA_A>;
impl PORTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTA_A {
        match self.bits {
            false => PORTA_A::_0,
            true => PORTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTA_A::_1
    }
}
#[doc = "Write proxy for field `PORTA`"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTA_A::_1)
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
#[doc = "PCTLB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTB_A> for bool {
    #[inline(always)]
    fn from(variant: PORTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTB`"]
pub type PORTB_R = crate::R<bool, PORTB_A>;
impl PORTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTB_A {
        match self.bits {
            false => PORTB_A::_0,
            true => PORTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTB_A::_1
    }
}
#[doc = "Write proxy for field `PORTB`"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTB_A::_1)
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
#[doc = "PCTLC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTC_A> for bool {
    #[inline(always)]
    fn from(variant: PORTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTC`"]
pub type PORTC_R = crate::R<bool, PORTC_A>;
impl PORTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTC_A {
        match self.bits {
            false => PORTC_A::_0,
            true => PORTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTC_A::_1
    }
}
#[doc = "Write proxy for field `PORTC`"]
pub struct PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "PCTLD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTD_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTD_A> for bool {
    #[inline(always)]
    fn from(variant: PORTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTD`"]
pub type PORTD_R = crate::R<bool, PORTD_A>;
impl PORTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTD_A {
        match self.bits {
            false => PORTD_A::_0,
            true => PORTD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTD_A::_1
    }
}
#[doc = "Write proxy for field `PORTD`"]
pub struct PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTD_A::_1)
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
#[doc = "PCTLE Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTE_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTE_A> for bool {
    #[inline(always)]
    fn from(variant: PORTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTE`"]
pub type PORTE_R = crate::R<bool, PORTE_A>;
impl PORTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTE_A {
        match self.bits {
            false => PORTE_A::_0,
            true => PORTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTE_A::_1
    }
}
#[doc = "Write proxy for field `PORTE`"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTE_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTE_A::_1)
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
#[doc = "PCTLF Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTF_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTF_A> for bool {
    #[inline(always)]
    fn from(variant: PORTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTF`"]
pub type PORTF_R = crate::R<bool, PORTF_A>;
impl PORTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTF_A {
        match self.bits {
            false => PORTF_A::_0,
            true => PORTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTF_A::_1
    }
}
#[doc = "Write proxy for field `PORTF`"]
pub struct PORTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTF_A::_1)
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
#[doc = "PCTLG Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTG_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTG_A> for bool {
    #[inline(always)]
    fn from(variant: PORTG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTG`"]
pub type PORTG_R = crate::R<bool, PORTG_A>;
impl PORTG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTG_A {
        match self.bits {
            false => PORTG_A::_0,
            true => PORTG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTG_A::_1
    }
}
#[doc = "Write proxy for field `PORTG`"]
pub struct PORTG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTG_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTG_A::_1)
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
#[doc = "PCTLH Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTH_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTH_A> for bool {
    #[inline(always)]
    fn from(variant: PORTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTH`"]
pub type PORTH_R = crate::R<bool, PORTH_A>;
impl PORTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTH_A {
        match self.bits {
            false => PORTH_A::_0,
            true => PORTH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTH_A::_1
    }
}
#[doc = "Write proxy for field `PORTH`"]
pub struct PORTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTH_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTH_A::_1)
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
#[doc = "PCTLI Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORTI_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PORTI_A> for bool {
    #[inline(always)]
    fn from(variant: PORTI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORTI`"]
pub type PORTI_R = crate::R<bool, PORTI_A>;
impl PORTI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORTI_A {
        match self.bits {
            false => PORTI_A::_0,
            true => PORTI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORTI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORTI_A::_1
    }
}
#[doc = "Write proxy for field `PORTI`"]
pub struct PORTI_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORTI_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORTI_A::_1)
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
#[doc = "iRTC Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTC_A::_1
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
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
#[doc = "iRTC_REG_FILE Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCREG_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<RTCREG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCREG`"]
pub type RTCREG_R = crate::R<bool, RTCREG_A>;
impl RTCREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCREG_A {
        match self.bits {
            false => RTCREG_A::_0,
            true => RTCREG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCREG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCREG_A::_1
    }
}
#[doc = "Write proxy for field `RTCREG`"]
pub struct RTCREG_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCREG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCREG_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCREG_A::_1)
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
#[doc = "Peripheral Crossbar Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XBAR_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<XBAR_A> for bool {
    #[inline(always)]
    fn from(variant: XBAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XBAR`"]
pub type XBAR_R = crate::R<bool, XBAR_A>;
impl XBAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XBAR_A {
        match self.bits {
            false => XBAR_A::_0,
            true => XBAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == XBAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == XBAR_A::_1
    }
}
#[doc = "Write proxy for field `XBAR`"]
pub struct XBAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XBAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XBAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(XBAR_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(XBAR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "QaudTimer channel 0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TMR0_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR0`"]
pub type TMR0_R = crate::R<bool, TMR0_A>;
impl TMR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0_A {
        match self.bits {
            false => TMR0_A::_0,
            true => TMR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR0_A::_1
    }
}
#[doc = "Write proxy for field `TMR0`"]
pub struct TMR0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0_A::_1)
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
#[doc = "QaudTimer channel 1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TMR1_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR1`"]
pub type TMR1_R = crate::R<bool, TMR1_A>;
impl TMR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_A {
        match self.bits {
            false => TMR1_A::_0,
            true => TMR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR1_A::_1
    }
}
#[doc = "Write proxy for field `TMR1`"]
pub struct TMR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1_A::_1)
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
#[doc = "QaudTimer channel 2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TMR2_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR2`"]
pub type TMR2_R = crate::R<bool, TMR2_A>;
impl TMR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2_A {
        match self.bits {
            false => TMR2_A::_0,
            true => TMR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR2_A::_1
    }
}
#[doc = "Write proxy for field `TMR2`"]
pub struct TMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2_A::_1)
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
#[doc = "QaudTimer channel 3 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<TMR3_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR3`"]
pub type TMR3_R = crate::R<bool, TMR3_A>;
impl TMR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3_A {
        match self.bits {
            false => TMR3_A::_0,
            true => TMR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR3_A::_1
    }
}
#[doc = "Write proxy for field `TMR3`"]
pub struct TMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3_A::_1)
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
impl R {
    #[doc = "Bit 3 - Segmented LCD Clock Gate Control"]
    #[inline(always)]
    pub fn slcd(&self) -> SLCD_R {
        SLCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PCTLA Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PCTLB Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PCTLC Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PCTLD Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PCTLE Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCTLF Clock Gate Control"]
    #[inline(always)]
    pub fn portf(&self) -> PORTF_R {
        PORTF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PCTLG Clock Gate Control"]
    #[inline(always)]
    pub fn portg(&self) -> PORTG_R {
        PORTG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PCTLH Clock Gate Control"]
    #[inline(always)]
    pub fn porth(&self) -> PORTH_R {
        PORTH_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PCTLI Clock Gate Control"]
    #[inline(always)]
    pub fn porti(&self) -> PORTI_R {
        PORTI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - iRTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - iRTC_REG_FILE Clock Gate Control"]
    #[inline(always)]
    pub fn rtcreg(&self) -> RTCREG_R {
        RTCREG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral Crossbar Clock Gate Control"]
    #[inline(always)]
    pub fn xbar(&self) -> XBAR_R {
        XBAR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - QaudTimer channel 0 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr0(&self) -> TMR0_R {
        TMR0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - QaudTimer channel 1 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - QaudTimer channel 2 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - QaudTimer channel 3 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Segmented LCD Clock Gate Control"]
    #[inline(always)]
    pub fn slcd(&mut self) -> SLCD_W {
        SLCD_W { w: self }
    }
    #[doc = "Bit 6 - PCTLA Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
    #[doc = "Bit 7 - PCTLB Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
    #[doc = "Bit 8 - PCTLC Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
    #[doc = "Bit 9 - PCTLD Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
    #[doc = "Bit 10 - PCTLE Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
    #[doc = "Bit 11 - PCTLF Clock Gate Control"]
    #[inline(always)]
    pub fn portf(&mut self) -> PORTF_W {
        PORTF_W { w: self }
    }
    #[doc = "Bit 12 - PCTLG Clock Gate Control"]
    #[inline(always)]
    pub fn portg(&mut self) -> PORTG_W {
        PORTG_W { w: self }
    }
    #[doc = "Bit 13 - PCTLH Clock Gate Control"]
    #[inline(always)]
    pub fn porth(&mut self) -> PORTH_W {
        PORTH_W { w: self }
    }
    #[doc = "Bit 14 - PCTLI Clock Gate Control"]
    #[inline(always)]
    pub fn porti(&mut self) -> PORTI_W {
        PORTI_W { w: self }
    }
    #[doc = "Bit 16 - iRTC Clock Gate Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 17 - iRTC_REG_FILE Clock Gate Control"]
    #[inline(always)]
    pub fn rtcreg(&mut self) -> RTCREG_W {
        RTCREG_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral Crossbar Clock Gate Control"]
    #[inline(always)]
    pub fn xbar(&mut self) -> XBAR_W {
        XBAR_W { w: self }
    }
    #[doc = "Bit 23 - QaudTimer channel 0 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr0(&mut self) -> TMR0_W {
        TMR0_W { w: self }
    }
    #[doc = "Bit 24 - QaudTimer channel 1 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr1(&mut self) -> TMR1_W {
        TMR1_W { w: self }
    }
    #[doc = "Bit 25 - QaudTimer channel 2 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr2(&mut self) -> TMR2_W {
        TMR2_W { w: self }
    }
    #[doc = "Bit 26 - QaudTimer channel 3 Clock Gate Control"]
    #[inline(always)]
    pub fn tmr3(&mut self) -> TMR3_W {
        TMR3_W { w: self }
    }
}
