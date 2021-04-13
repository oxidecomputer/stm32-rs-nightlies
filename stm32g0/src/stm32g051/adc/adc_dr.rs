#[doc = "Reader of register ADC_DR"]
pub type R = crate::R<u32, super::ADC_DR>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on pageÂ 389. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
