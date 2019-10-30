#[doc = "Reader of register CFG3"]
pub type R = crate::R<u32, super::CFG3>;
#[doc = "Writer for register CFG3"]
pub type W = crate::W<u32, super::CFG3>;
#[doc = "Register CFG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cfg3`"]
pub type CFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cfg3`"]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
}
