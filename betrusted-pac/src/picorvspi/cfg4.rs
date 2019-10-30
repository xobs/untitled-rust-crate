#[doc = "Reader of register CFG4"]
pub type R = crate::R<u32, super::CFG4>;
#[doc = "Writer for register CFG4"]
pub type W = crate::W<u32, super::CFG4>;
#[doc = "Register CFG4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cfg4`"]
pub type CFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cfg4`"]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
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
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
}
