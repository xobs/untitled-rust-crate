#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub dat: DAT,
    #[doc = "0x04 - "]
    pub addr: ADDR,
    #[doc = "0x08 - "]
    pub ctrl: CTRL,
    #[doc = "0x0c - "]
    pub raw: RAW,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dat](dat) module"]
pub type DAT = crate::Reg<u32, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "`write(|w| ..)` method takes [dat::W](dat::W) writer structure"]
impl crate::Writable for DAT {}
#[doc = ""]
pub mod dat;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = ""]
pub mod addr;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = ""]
pub mod ctrl;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [raw](raw) module"]
pub type RAW = crate::Reg<u32, _RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAW;
#[doc = "`read()` method returns [raw::R](raw::R) reader structure"]
impl crate::Readable for RAW {}
#[doc = "`write(|w| ..)` method takes [raw::W](raw::W) writer structure"]
impl crate::Writable for RAW {}
#[doc = ""]
pub mod raw;
