#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub cfg1: CFG1,
    #[doc = "0x04 - "]
    pub cfg2: CFG2,
    #[doc = "0x08 - "]
    pub cfg3: CFG3,
    #[doc = "0x0c - "]
    pub cfg4: CFG4,
    #[doc = "0x10 - "]
    pub stat1: STAT1,
    #[doc = "0x14 - "]
    pub stat2: STAT2,
    #[doc = "0x18 - "]
    pub stat3: STAT3,
    #[doc = "0x1c - "]
    pub stat4: STAT4,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = ""]
pub mod cfg1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = ""]
pub mod cfg2;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg3](cfg3) module"]
pub type CFG3 = crate::Reg<u32, _CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG3;
#[doc = "`read()` method returns [cfg3::R](cfg3::R) reader structure"]
impl crate::Readable for CFG3 {}
#[doc = "`write(|w| ..)` method takes [cfg3::W](cfg3::W) writer structure"]
impl crate::Writable for CFG3 {}
#[doc = ""]
pub mod cfg3;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg4](cfg4) module"]
pub type CFG4 = crate::Reg<u32, _CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG4;
#[doc = "`read()` method returns [cfg4::R](cfg4::R) reader structure"]
impl crate::Readable for CFG4 {}
#[doc = "`write(|w| ..)` method takes [cfg4::W](cfg4::W) writer structure"]
impl crate::Writable for CFG4 {}
#[doc = ""]
pub mod cfg4;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat1](stat1) module"]
pub type STAT1 = crate::Reg<u32, _STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT1;
#[doc = "`read()` method returns [stat1::R](stat1::R) reader structure"]
impl crate::Readable for STAT1 {}
#[doc = "`write(|w| ..)` method takes [stat1::W](stat1::W) writer structure"]
impl crate::Writable for STAT1 {}
#[doc = ""]
pub mod stat1;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat2](stat2) module"]
pub type STAT2 = crate::Reg<u32, _STAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT2;
#[doc = "`read()` method returns [stat2::R](stat2::R) reader structure"]
impl crate::Readable for STAT2 {}
#[doc = "`write(|w| ..)` method takes [stat2::W](stat2::W) writer structure"]
impl crate::Writable for STAT2 {}
#[doc = ""]
pub mod stat2;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat3](stat3) module"]
pub type STAT3 = crate::Reg<u32, _STAT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT3;
#[doc = "`read()` method returns [stat3::R](stat3::R) reader structure"]
impl crate::Readable for STAT3 {}
#[doc = "`write(|w| ..)` method takes [stat3::W](stat3::W) writer structure"]
impl crate::Writable for STAT3 {}
#[doc = ""]
pub mod stat3;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat4](stat4) module"]
pub type STAT4 = crate::Reg<u32, _STAT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT4;
#[doc = "`read()` method returns [stat4::R](stat4::R) reader structure"]
impl crate::Readable for STAT4 {}
#[doc = "`write(|w| ..)` method takes [stat4::W](stat4::W) writer structure"]
impl crate::Writable for STAT4 {}
#[doc = ""]
pub mod stat4;
