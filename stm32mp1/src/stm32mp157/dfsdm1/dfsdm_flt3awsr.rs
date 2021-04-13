#[doc = "Reader of register DFSDM_FLT3AWSR"]
pub type R = crate::R<u32, super::DFSDM_FLT3AWSR>;
#[doc = "Reader of field `AWLTF`"]
pub type AWLTF_R = crate::R<u8, u8>;
#[doc = "Reader of field `AWHTF`"]
pub type AWHTF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - AWLTF"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AWHTF"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
