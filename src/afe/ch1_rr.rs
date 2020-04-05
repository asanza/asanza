#[doc = "Reader of register CH1_RR"]
pub type R = crate::R<u32, super::CH1_RR>;
#[doc = "Reader of field `SDR`"]
pub type SDR_R = crate::R<u32, u32>;
#[doc = "Reader of field `SIGN_BITS`"]
pub type SIGN_BITS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:22 - Sample Data Result"]
    #[inline(always)]
    pub fn sdr(&self) -> SDR_R {
        SDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
    #[doc = "Bits 23:31 - Sign Bits"]
    #[inline(always)]
    pub fn sign_bits(&self) -> SIGN_BITS_R {
        SIGN_BITS_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
