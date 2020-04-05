#[doc = "Reader of register ISR"]
pub type R = crate::R<u16, super::ISR>;
#[doc = "Writer for register ISR"]
pub type W = crate::W<u16, super::ISR>;
#[doc = "Register ISR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ISR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Tamper Interrupt Status bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPER_IS_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted (Default on reset) ."]
    _1 = 1,
}
impl From<TAMPER_IS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPER_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAMPER_IS`"]
pub type TAMPER_IS_R = crate::R<bool, TAMPER_IS_A>;
impl TAMPER_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPER_IS_A {
        match self.bits {
            false => TAMPER_IS_A::_0,
            true => TAMPER_IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAMPER_IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAMPER_IS_A::_1
    }
}
#[doc = "Alarm Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALM_IS_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<ALM_IS_A> for bool {
    #[inline(always)]
    fn from(variant: ALM_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALM_IS`"]
pub type ALM_IS_R = crate::R<bool, ALM_IS_A>;
impl ALM_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALM_IS_A {
        match self.bits {
            false => ALM_IS_A::_0,
            true => ALM_IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALM_IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALM_IS_A::_1
    }
}
#[doc = "Write proxy for field `ALM_IS`"]
pub struct ALM_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALM_IS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALM_IS_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALM_IS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Days Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAY_IS_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<DAY_IS_A> for bool {
    #[inline(always)]
    fn from(variant: DAY_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAY_IS`"]
pub type DAY_IS_R = crate::R<bool, DAY_IS_A>;
impl DAY_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAY_IS_A {
        match self.bits {
            false => DAY_IS_A::_0,
            true => DAY_IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAY_IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAY_IS_A::_1
    }
}
#[doc = "Write proxy for field `DAY_IS`"]
pub struct DAY_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAY_IS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAY_IS_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAY_IS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Hours Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOUR_IS_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<HOUR_IS_A> for bool {
    #[inline(always)]
    fn from(variant: HOUR_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOUR_IS`"]
pub type HOUR_IS_R = crate::R<bool, HOUR_IS_A>;
impl HOUR_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOUR_IS_A {
        match self.bits {
            false => HOUR_IS_A::_0,
            true => HOUR_IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOUR_IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOUR_IS_A::_1
    }
}
#[doc = "Write proxy for field `HOUR_IS`"]
pub struct HOUR_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOUR_IS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOUR_IS_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOUR_IS_A::_1)
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
#[doc = "Minutes Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIN_IS_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<MIN_IS_A> for bool {
    #[inline(always)]
    fn from(variant: MIN_IS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIN_IS`"]
pub type MIN_IS_R = crate::R<bool, MIN_IS_A>;
impl MIN_IS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIN_IS_A {
        match self.bits {
            false => MIN_IS_A::_0,
            true => MIN_IS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MIN_IS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MIN_IS_A::_1
    }
}
#[doc = "Write proxy for field `MIN_IS`"]
pub struct MIN_IS_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_IS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIN_IS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MIN_IS_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MIN_IS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "1 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_1HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_1HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_1HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_1HZ`"]
pub type IS_1HZ_R = crate::R<bool, IS_1HZ_A>;
impl IS_1HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_1HZ_A {
        match self.bits {
            false => IS_1HZ_A::_0,
            true => IS_1HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_1HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_1HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_1HZ`"]
pub struct IS_1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_1HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_1HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_1HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_1HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "2 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_2HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_2HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_2HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_2HZ`"]
pub type IS_2HZ_R = crate::R<bool, IS_2HZ_A>;
impl IS_2HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_2HZ_A {
        match self.bits {
            false => IS_2HZ_A::_0,
            true => IS_2HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_2HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_2HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_2HZ`"]
pub struct IS_2HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_2HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_2HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_2HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_2HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "4 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_4HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_4HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_4HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_4HZ`"]
pub type IS_4HZ_R = crate::R<bool, IS_4HZ_A>;
impl IS_4HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_4HZ_A {
        match self.bits {
            false => IS_4HZ_A::_0,
            true => IS_4HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_4HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_4HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_4HZ`"]
pub struct IS_4HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_4HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_4HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_4HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_4HZ_A::_1)
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
#[doc = "8 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_8HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_8HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_8HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_8HZ`"]
pub type IS_8HZ_R = crate::R<bool, IS_8HZ_A>;
impl IS_8HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_8HZ_A {
        match self.bits {
            false => IS_8HZ_A::_0,
            true => IS_8HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_8HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_8HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_8HZ`"]
pub struct IS_8HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_8HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_8HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_8HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_8HZ_A::_1)
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
#[doc = "16 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_16HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_16HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_16HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_16HZ`"]
pub type IS_16HZ_R = crate::R<bool, IS_16HZ_A>;
impl IS_16HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_16HZ_A {
        match self.bits {
            false => IS_16HZ_A::_0,
            true => IS_16HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_16HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_16HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_16HZ`"]
pub struct IS_16HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_16HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_16HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_16HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_16HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "32 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_32HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_32HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_32HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_32HZ`"]
pub type IS_32HZ_R = crate::R<bool, IS_32HZ_A>;
impl IS_32HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_32HZ_A {
        match self.bits {
            false => IS_32HZ_A::_0,
            true => IS_32HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_32HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_32HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_32HZ`"]
pub struct IS_32HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_32HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_32HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_32HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_32HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "64 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_64HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_64HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_64HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_64HZ`"]
pub type IS_64HZ_R = crate::R<bool, IS_64HZ_A>;
impl IS_64HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_64HZ_A {
        match self.bits {
            false => IS_64HZ_A::_0,
            true => IS_64HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_64HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_64HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_64HZ`"]
pub struct IS_64HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_64HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_64HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_64HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_64HZ_A::_1)
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
#[doc = "128 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_128HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_128HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_128HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_128HZ`"]
pub type IS_128HZ_R = crate::R<bool, IS_128HZ_A>;
impl IS_128HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_128HZ_A {
        match self.bits {
            false => IS_128HZ_A::_0,
            true => IS_128HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_128HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_128HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_128HZ`"]
pub struct IS_128HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_128HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_128HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_128HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_128HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "256 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_256HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_256HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_256HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_256HZ`"]
pub type IS_256HZ_R = crate::R<bool, IS_256HZ_A>;
impl IS_256HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_256HZ_A {
        match self.bits {
            false => IS_256HZ_A::_0,
            true => IS_256HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_256HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_256HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_256HZ`"]
pub struct IS_256HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_256HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_256HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_256HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_256HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "512 Hz Interval Interrupt Status bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS_512HZ_A {
    #[doc = "0: Interrupt is de-asserted."]
    _0 = 0,
    #[doc = "1: Interrupt is asserted."]
    _1 = 1,
}
impl From<IS_512HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IS_512HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IS_512HZ`"]
pub type IS_512HZ_R = crate::R<bool, IS_512HZ_A>;
impl IS_512HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS_512HZ_A {
        match self.bits {
            false => IS_512HZ_A::_0,
            true => IS_512HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IS_512HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IS_512HZ_A::_1
    }
}
#[doc = "Write proxy for field `IS_512HZ`"]
pub struct IS_512HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_512HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IS_512HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is de-asserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IS_512HZ_A::_0)
    }
    #[doc = "Interrupt is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IS_512HZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Interrupt Status bit."]
    #[inline(always)]
    pub fn tamper_is(&self) -> TAMPER_IS_R {
        TAMPER_IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alarm Interrupt Status bit."]
    #[inline(always)]
    pub fn alm_is(&self) -> ALM_IS_R {
        ALM_IS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Days Interrupt Status bit."]
    #[inline(always)]
    pub fn day_is(&self) -> DAY_IS_R {
        DAY_IS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hours Interrupt Status bit."]
    #[inline(always)]
    pub fn hour_is(&self) -> HOUR_IS_R {
        HOUR_IS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Minutes Interrupt Status bit."]
    #[inline(always)]
    pub fn min_is(&self) -> MIN_IS_R {
        MIN_IS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_1hz(&self) -> IS_1HZ_R {
        IS_1HZ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 2 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_2hz(&self) -> IS_2HZ_R {
        IS_2HZ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 4 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_4hz(&self) -> IS_4HZ_R {
        IS_4HZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 8 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_8hz(&self) -> IS_8HZ_R {
        IS_8HZ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 16 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_16hz(&self) -> IS_16HZ_R {
        IS_16HZ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 32 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_32hz(&self) -> IS_32HZ_R {
        IS_32HZ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 64 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_64hz(&self) -> IS_64HZ_R {
        IS_64HZ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 128 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_128hz(&self) -> IS_128HZ_R {
        IS_128HZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 256 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_256hz(&self) -> IS_256HZ_R {
        IS_256HZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 512 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_512hz(&self) -> IS_512HZ_R {
        IS_512HZ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Alarm Interrupt Status bit."]
    #[inline(always)]
    pub fn alm_is(&mut self) -> ALM_IS_W {
        ALM_IS_W { w: self }
    }
    #[doc = "Bit 3 - Days Interrupt Status bit."]
    #[inline(always)]
    pub fn day_is(&mut self) -> DAY_IS_W {
        DAY_IS_W { w: self }
    }
    #[doc = "Bit 4 - Hours Interrupt Status bit."]
    #[inline(always)]
    pub fn hour_is(&mut self) -> HOUR_IS_W {
        HOUR_IS_W { w: self }
    }
    #[doc = "Bit 5 - Minutes Interrupt Status bit."]
    #[inline(always)]
    pub fn min_is(&mut self) -> MIN_IS_W {
        MIN_IS_W { w: self }
    }
    #[doc = "Bit 6 - 1 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_1hz(&mut self) -> IS_1HZ_W {
        IS_1HZ_W { w: self }
    }
    #[doc = "Bit 7 - 2 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_2hz(&mut self) -> IS_2HZ_W {
        IS_2HZ_W { w: self }
    }
    #[doc = "Bit 8 - 4 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_4hz(&mut self) -> IS_4HZ_W {
        IS_4HZ_W { w: self }
    }
    #[doc = "Bit 9 - 8 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_8hz(&mut self) -> IS_8HZ_W {
        IS_8HZ_W { w: self }
    }
    #[doc = "Bit 10 - 16 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_16hz(&mut self) -> IS_16HZ_W {
        IS_16HZ_W { w: self }
    }
    #[doc = "Bit 11 - 32 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_32hz(&mut self) -> IS_32HZ_W {
        IS_32HZ_W { w: self }
    }
    #[doc = "Bit 12 - 64 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_64hz(&mut self) -> IS_64HZ_W {
        IS_64HZ_W { w: self }
    }
    #[doc = "Bit 13 - 128 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_128hz(&mut self) -> IS_128HZ_W {
        IS_128HZ_W { w: self }
    }
    #[doc = "Bit 14 - 256 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_256hz(&mut self) -> IS_256HZ_W {
        IS_256HZ_W { w: self }
    }
    #[doc = "Bit 15 - 512 Hz Interval Interrupt Status bit."]
    #[inline(always)]
    pub fn is_512hz(&mut self) -> IS_512HZ_W {
        IS_512HZ_W { w: self }
    }
}
