#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cfg1`"]
pub type CFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cfg1`"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
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
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
}
