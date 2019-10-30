#[doc = "Reader of register GITEXTRA1"]
pub type R = crate::R<u32, super::GITEXTRA1>;
#[doc = "Writer for register GITEXTRA1"]
pub type W = crate::W<u32, super::GITEXTRA1>;
#[doc = "Register GITEXTRA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GITEXTRA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gitextra`"]
pub type GITEXTRA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gitextra`"]
pub struct GITEXTRA_W<'a> {
    w: &'a mut W,
}
impl<'a> GITEXTRA_W<'a> {
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
    pub fn gitextra(&self) -> GITEXTRA_R {
        GITEXTRA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gitextra(&mut self) -> GITEXTRA_W {
        GITEXTRA_W { w: self }
    }
}
