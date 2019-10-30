#[doc = "Reader of register REVISION"]
pub type R = crate::R<u32, super::REVISION>;
#[doc = "Writer for register REVISION"]
pub type W = crate::W<u32, super::REVISION>;
#[doc = "Register REVISION `reset()`'s with value 0"]
impl crate::ResetValue for super::REVISION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `revision`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `revision`"]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
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
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
}
