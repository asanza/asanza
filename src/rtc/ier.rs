#[doc = "Reader of register IER"]
pub type R = crate::R<u16, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u16, super::IER>;
#[doc = "Register IER `reset()`'s with value 0x01"]
impl crate::ResetValue for super::IER {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Tamper Interrupt Enable bit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPER_IE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled (Default on reset)."]
    _1 = 1,
}
impl From<TAMPER_IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPER_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAMPER_IE`"]
pub type TAMPER_IE_R = crate::R<bool, TAMPER_IE_A>;
impl TAMPER_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPER_IE_A {
        match self.bits {
            false => TAMPER_IE_A::_0,
            true => TAMPER_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAMPER_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAMPER_IE_A::_1
    }
}
#[doc = "Write proxy for field `TAMPER_IE`"]
pub struct TAMPER_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPER_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPER_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAMPER_IE_A::_0)
    }
    #[doc = "Interrupt is enabled (Default on reset)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAMPER_IE_A::_1)
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
#[doc = "Alarm Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALM_IE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<ALM_IE_A> for bool {
    #[inline(always)]
    fn from(variant: ALM_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALM_IE`"]
pub type ALM_IE_R = crate::R<bool, ALM_IE_A>;
impl ALM_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALM_IE_A {
        match self.bits {
            false => ALM_IE_A::_0,
            true => ALM_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALM_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALM_IE_A::_1
    }
}
#[doc = "Write proxy for field `ALM_IE`"]
pub struct ALM_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALM_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALM_IE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALM_IE_A::_1)
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
#[doc = "Days Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAY_IE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<DAY_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DAY_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAY_IE`"]
pub type DAY_IE_R = crate::R<bool, DAY_IE_A>;
impl DAY_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAY_IE_A {
        match self.bits {
            false => DAY_IE_A::_0,
            true => DAY_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAY_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAY_IE_A::_1
    }
}
#[doc = "Write proxy for field `DAY_IE`"]
pub struct DAY_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAY_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAY_IE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAY_IE_A::_1)
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
#[doc = "Hours Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOUR_IE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<HOUR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: HOUR_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HOUR_IE`"]
pub type HOUR_IE_R = crate::R<bool, HOUR_IE_A>;
impl HOUR_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOUR_IE_A {
        match self.bits {
            false => HOUR_IE_A::_0,
            true => HOUR_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOUR_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOUR_IE_A::_1
    }
}
#[doc = "Write proxy for field `HOUR_IE`"]
pub struct HOUR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOUR_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOUR_IE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOUR_IE_A::_1)
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
#[doc = "Minutes Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIN_IE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<MIN_IE_A> for bool {
    #[inline(always)]
    fn from(variant: MIN_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MIN_IE`"]
pub type MIN_IE_R = crate::R<bool, MIN_IE_A>;
impl MIN_IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIN_IE_A {
        match self.bits {
            false => MIN_IE_A::_0,
            true => MIN_IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MIN_IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MIN_IE_A::_1
    }
}
#[doc = "Write proxy for field `MIN_IE`"]
pub struct MIN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIN_IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MIN_IE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MIN_IE_A::_1)
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
#[doc = "1 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_1HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_1HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_1HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_1HZ`"]
pub type IE_1HZ_R = crate::R<bool, IE_1HZ_A>;
impl IE_1HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_1HZ_A {
        match self.bits {
            false => IE_1HZ_A::_0,
            true => IE_1HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_1HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_1HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_1HZ`"]
pub struct IE_1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_1HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_1HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_1HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_1HZ_A::_1)
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
#[doc = "2 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_2HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_2HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_2HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_2HZ`"]
pub type IE_2HZ_R = crate::R<bool, IE_2HZ_A>;
impl IE_2HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_2HZ_A {
        match self.bits {
            false => IE_2HZ_A::_0,
            true => IE_2HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_2HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_2HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_2HZ`"]
pub struct IE_2HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_2HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_2HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_2HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_2HZ_A::_1)
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
#[doc = "4 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_4HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_4HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_4HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_4HZ`"]
pub type IE_4HZ_R = crate::R<bool, IE_4HZ_A>;
impl IE_4HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_4HZ_A {
        match self.bits {
            false => IE_4HZ_A::_0,
            true => IE_4HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_4HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_4HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_4HZ`"]
pub struct IE_4HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_4HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_4HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_4HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_4HZ_A::_1)
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
#[doc = "8 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_8HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_8HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_8HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_8HZ`"]
pub type IE_8HZ_R = crate::R<bool, IE_8HZ_A>;
impl IE_8HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_8HZ_A {
        match self.bits {
            false => IE_8HZ_A::_0,
            true => IE_8HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_8HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_8HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_8HZ`"]
pub struct IE_8HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_8HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_8HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_8HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_8HZ_A::_1)
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
#[doc = "16 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_16HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_16HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_16HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_16HZ`"]
pub type IE_16HZ_R = crate::R<bool, IE_16HZ_A>;
impl IE_16HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_16HZ_A {
        match self.bits {
            false => IE_16HZ_A::_0,
            true => IE_16HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_16HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_16HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_16HZ`"]
pub struct IE_16HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_16HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_16HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_16HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_16HZ_A::_1)
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
#[doc = "32 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_32HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_32HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_32HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_32HZ`"]
pub type IE_32HZ_R = crate::R<bool, IE_32HZ_A>;
impl IE_32HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_32HZ_A {
        match self.bits {
            false => IE_32HZ_A::_0,
            true => IE_32HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_32HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_32HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_32HZ`"]
pub struct IE_32HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_32HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_32HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_32HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_32HZ_A::_1)
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
#[doc = "64 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_64HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_64HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_64HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_64HZ`"]
pub type IE_64HZ_R = crate::R<bool, IE_64HZ_A>;
impl IE_64HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_64HZ_A {
        match self.bits {
            false => IE_64HZ_A::_0,
            true => IE_64HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_64HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_64HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_64HZ`"]
pub struct IE_64HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_64HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_64HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_64HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_64HZ_A::_1)
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
#[doc = "128 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_128HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_128HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_128HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_128HZ`"]
pub type IE_128HZ_R = crate::R<bool, IE_128HZ_A>;
impl IE_128HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_128HZ_A {
        match self.bits {
            false => IE_128HZ_A::_0,
            true => IE_128HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_128HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_128HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_128HZ`"]
pub struct IE_128HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_128HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_128HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_128HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_128HZ_A::_1)
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
#[doc = "256 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_256HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_256HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_256HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_256HZ`"]
pub type IE_256HZ_R = crate::R<bool, IE_256HZ_A>;
impl IE_256HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_256HZ_A {
        match self.bits {
            false => IE_256HZ_A::_0,
            true => IE_256HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_256HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_256HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_256HZ`"]
pub struct IE_256HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_256HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_256HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_256HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_256HZ_A::_1)
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
#[doc = "512 Hz Interval Interrupt Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_512HZ_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IE_512HZ_A> for bool {
    #[inline(always)]
    fn from(variant: IE_512HZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE_512HZ`"]
pub type IE_512HZ_R = crate::R<bool, IE_512HZ_A>;
impl IE_512HZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_512HZ_A {
        match self.bits {
            false => IE_512HZ_A::_0,
            true => IE_512HZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_512HZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_512HZ_A::_1
    }
}
#[doc = "Write proxy for field `IE_512HZ`"]
pub struct IE_512HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_512HZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_512HZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_512HZ_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_512HZ_A::_1)
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
    #[doc = "Bit 0 - Tamper Interrupt Enable bit."]
    #[inline(always)]
    pub fn tamper_ie(&self) -> TAMPER_IE_R {
        TAMPER_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Alarm Interrupt Enable bit."]
    #[inline(always)]
    pub fn alm_ie(&self) -> ALM_IE_R {
        ALM_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Days Interrupt Enable bit."]
    #[inline(always)]
    pub fn day_ie(&self) -> DAY_IE_R {
        DAY_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hours Interrupt Enable bit."]
    #[inline(always)]
    pub fn hour_ie(&self) -> HOUR_IE_R {
        HOUR_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Minutes Interrupt Enable bit."]
    #[inline(always)]
    pub fn min_ie(&self) -> MIN_IE_R {
        MIN_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_1hz(&self) -> IE_1HZ_R {
        IE_1HZ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 2 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_2hz(&self) -> IE_2HZ_R {
        IE_2HZ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 4 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_4hz(&self) -> IE_4HZ_R {
        IE_4HZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 8 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_8hz(&self) -> IE_8HZ_R {
        IE_8HZ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 16 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_16hz(&self) -> IE_16HZ_R {
        IE_16HZ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 32 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_32hz(&self) -> IE_32HZ_R {
        IE_32HZ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 64 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_64hz(&self) -> IE_64HZ_R {
        IE_64HZ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 128 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_128hz(&self) -> IE_128HZ_R {
        IE_128HZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 256 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_256hz(&self) -> IE_256HZ_R {
        IE_256HZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 512 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_512hz(&self) -> IE_512HZ_R {
        IE_512HZ_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Interrupt Enable bit."]
    #[inline(always)]
    pub fn tamper_ie(&mut self) -> TAMPER_IE_W {
        TAMPER_IE_W { w: self }
    }
    #[doc = "Bit 2 - Alarm Interrupt Enable bit."]
    #[inline(always)]
    pub fn alm_ie(&mut self) -> ALM_IE_W {
        ALM_IE_W { w: self }
    }
    #[doc = "Bit 3 - Days Interrupt Enable bit."]
    #[inline(always)]
    pub fn day_ie(&mut self) -> DAY_IE_W {
        DAY_IE_W { w: self }
    }
    #[doc = "Bit 4 - Hours Interrupt Enable bit."]
    #[inline(always)]
    pub fn hour_ie(&mut self) -> HOUR_IE_W {
        HOUR_IE_W { w: self }
    }
    #[doc = "Bit 5 - Minutes Interrupt Enable bit."]
    #[inline(always)]
    pub fn min_ie(&mut self) -> MIN_IE_W {
        MIN_IE_W { w: self }
    }
    #[doc = "Bit 6 - 1 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_1hz(&mut self) -> IE_1HZ_W {
        IE_1HZ_W { w: self }
    }
    #[doc = "Bit 7 - 2 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_2hz(&mut self) -> IE_2HZ_W {
        IE_2HZ_W { w: self }
    }
    #[doc = "Bit 8 - 4 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_4hz(&mut self) -> IE_4HZ_W {
        IE_4HZ_W { w: self }
    }
    #[doc = "Bit 9 - 8 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_8hz(&mut self) -> IE_8HZ_W {
        IE_8HZ_W { w: self }
    }
    #[doc = "Bit 10 - 16 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_16hz(&mut self) -> IE_16HZ_W {
        IE_16HZ_W { w: self }
    }
    #[doc = "Bit 11 - 32 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_32hz(&mut self) -> IE_32HZ_W {
        IE_32HZ_W { w: self }
    }
    #[doc = "Bit 12 - 64 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_64hz(&mut self) -> IE_64HZ_W {
        IE_64HZ_W { w: self }
    }
    #[doc = "Bit 13 - 128 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_128hz(&mut self) -> IE_128HZ_W {
        IE_128HZ_W { w: self }
    }
    #[doc = "Bit 14 - 256 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_256hz(&mut self) -> IE_256HZ_W {
        IE_256HZ_W { w: self }
    }
    #[doc = "Bit 15 - 512 Hz Interval Interrupt Enable bit."]
    #[inline(always)]
    pub fn ie_512hz(&mut self) -> IE_512HZ_W {
        IE_512HZ_W { w: self }
    }
}
