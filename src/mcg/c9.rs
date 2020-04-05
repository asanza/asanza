#[doc = "Reader of register C9"]
pub type R = crate::R<u8, super::C9>;
#[doc = "Coarse Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COARSE_LOCK_A {
    #[doc = "0: PLL is currently unlocked."]
    _0 = 0,
    #[doc = "1: PLL is currently locked after first sample."]
    _1 = 1,
}
impl From<COARSE_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COARSE_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COARSE_LOCK`"]
pub type COARSE_LOCK_R = crate::R<bool, COARSE_LOCK_A>;
impl COARSE_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COARSE_LOCK_A {
        match self.bits {
            false => COARSE_LOCK_A::_0,
            true => COARSE_LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COARSE_LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COARSE_LOCK_A::_1
    }
}
#[doc = "Coarse Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COARSE_LOLS_A {
    #[doc = "0: PLL has not lost lock since COARSE_LOLS was last cleared."]
    _0 = 0,
    #[doc = "1: PLL has lost lock since COARSE_LOLS was last cleared."]
    _1 = 1,
}
impl From<COARSE_LOLS_A> for bool {
    #[inline(always)]
    fn from(variant: COARSE_LOLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COARSE_LOLS`"]
pub type COARSE_LOLS_R = crate::R<bool, COARSE_LOLS_A>;
impl COARSE_LOLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COARSE_LOLS_A {
        match self.bits {
            false => COARSE_LOLS_A::_0,
            true => COARSE_LOLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COARSE_LOLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COARSE_LOLS_A::_1
    }
}
impl R {
    #[doc = "Bit 6 - Coarse Lock Status"]
    #[inline(always)]
    pub fn coarse_lock(&self) -> COARSE_LOCK_R {
        COARSE_LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Coarse Loss of Lock Status"]
    #[inline(always)]
    pub fn coarse_lols(&self) -> COARSE_LOLS_R {
        COARSE_LOLS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
