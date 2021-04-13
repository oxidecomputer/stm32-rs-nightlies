#[doc = "Reader of register AES_DINR"]
pub type R = crate::R<u32, super::AES_DINR>;
#[doc = "Writer for register AES_DINR"]
pub type W = crate::W<u32, super::AES_DINR>;
#[doc = "Register AES_DINR `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_DINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIN`"]
pub type DIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DIN`"]
pub struct DIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the AES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for input) - Mode 3 (decryption) and Mode 4 (key derivation then single decryption): ciphertext The data swap operation is described in page 499."]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input data word A four-fold sequential write to this bitfield during the input phase results in writing a complete 128-bit block of input data to the AES peripheral. From the first to the fourth write, the corresponding data weights are \\[127:96\\], \\[95:64\\], \\[63:32\\], and \\[31:0\\]. Upon each write, the data from the 32-bit input buffer are handled by the data swap block according to the DATATYPE\\[1:0\\]
bitfield, then written into the AES core 128-bit input buffer. The data signification of the input data block depends on the AES operating mode: - Mode 1 (encryption): plaintext - Mode 2 (key derivation): the bitfield is not used (AES_KEYRx registers used for input) - Mode 3 (decryption) and Mode 4 (key derivation then single decryption): ciphertext The data swap operation is described in page 499."]
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W {
        DIN_W { w: self }
    }
}
