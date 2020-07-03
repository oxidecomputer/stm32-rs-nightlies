#[doc = "receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rir](rir) module"]
pub type RIR = crate::Reg<u32, _RIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIR;
#[doc = "`read()` method returns [rir::R](rir::R) reader structure"]
impl crate::Readable for RIR {}
#[doc = "receive FIFO mailbox identifier register"]
pub mod rir;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtr](rdtr) module"]
pub type RDTR = crate::Reg<u32, _RDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTR;
#[doc = "`read()` method returns [rdtr::R](rdtr::R) reader structure"]
impl crate::Readable for RDTR {}
#[doc = "mailbox data high register"]
pub mod rdtr;
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdlr](rdlr) module"]
pub type RDLR = crate::Reg<u32, _RDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDLR;
#[doc = "`read()` method returns [rdlr::R](rdlr::R) reader structure"]
impl crate::Readable for RDLR {}
#[doc = "mailbox data high register"]
pub mod rdlr;
#[doc = "receive FIFO mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdhr](rdhr) module"]
pub type RDHR = crate::Reg<u32, _RDHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDHR;
#[doc = "`read()` method returns [rdhr::R](rdhr::R) reader structure"]
impl crate::Readable for RDHR {}
#[doc = "receive FIFO mailbox data high register"]
pub mod rdhr;
