#[doc = "Reader of register ADC_COMP0"]
pub type R = crate::R<u32, super::ADC_COMP0>;
#[doc = "Reader of field `ADCCOMPVAL0`"]
pub type ADCCOMPVAL0_R = crate::R<u16, u16>;
#[doc = "Reader of field `ADCCOMPVAL1`"]
pub type ADCCOMPVAL1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC Temperature Compensation Value 0"]
    #[inline(always)]
    pub fn adccompval0(&self) -> ADCCOMPVAL0_R {
        ADCCOMPVAL0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC Temperature Compensation Value 1"]
    #[inline(always)]
    pub fn adccompval1(&self) -> ADCCOMPVAL1_R {
        ADCCOMPVAL1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
