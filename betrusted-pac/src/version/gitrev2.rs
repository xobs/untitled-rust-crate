#[doc = "Reader of register GITREV2"]
pub type R = crate::R<u32, super::GITREV2>;
#[doc = "Writer for register GITREV2"]
pub type W = crate::W<u32, super::GITREV2>;
#[doc = "Register GITREV2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GITREV2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `gitrev`"]
pub type GITREV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `gitrev`"]
pub struct GITREV_W<'a> {
    w: &'a mut W,
}
impl<'a> GITREV_W<'a> {
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
    pub fn gitrev(&self) -> GITREV_R {
        GITREV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn gitrev(&mut self) -> GITREV_W {
        GITREV_W { w: self }
    }
}
