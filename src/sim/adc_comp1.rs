#[doc = "Reader of register ADC_COMP1"]
pub type R = crate::R<u32, super::ADC_COMP1>;
#[doc = "Reader of field `ADCCOMPVAL2`"]
pub type ADCCOMPVAL2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC Temperature Compensation Value 2"]
    #[inline(always)]
    pub fn adccompval2(&self) -> ADCCOMPVAL2_R {
        ADCCOMPVAL2_R::new((self.bits & 0xffff) as u16)
    }
}
