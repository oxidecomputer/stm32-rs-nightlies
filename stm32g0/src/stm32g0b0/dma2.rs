#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - high interrupt status register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch1: CH,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch2: CH,
    _reserved4: [u8; 4usize],
    #[doc = "0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch3: CH,
    _reserved5: [u8; 4usize],
    #[doc = "0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch4: CH,
    _reserved6: [u8; 4usize],
    #[doc = "0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch5: CH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel 1 configuration register"]
    pub cr: self::ch::CR,
    #[doc = "0x04 - DMA channel 1 number of data tegister"]
    pub ndtr: self::ch::NDTR,
    #[doc = "0x08 - DMA channel 1 peripheral address"]
    pub par: self::ch::PAR,
    #[doc = "0x0c - DMA channel 1 memory address"]
    pub mar: self::ch::MAR,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "low interrupt status register"]
pub mod isr;
#[doc = "high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`read()` method returns [ifcr::R](ifcr::R) reader structure"]
impl crate::Readable for IFCR {}
#[doc = "high interrupt status register"]
pub mod ifcr;
