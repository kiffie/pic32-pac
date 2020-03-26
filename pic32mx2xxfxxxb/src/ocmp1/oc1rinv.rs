#[doc = "Reader of register OC1RINV"]
pub type R = crate::R<u32, super::OC1RINV>;
#[doc = "Writer for register OC1RINV"]
pub type W = crate::W<u32, super::OC1RINV>;
#[doc = "Register OC1RINV `reset()`'s with value 0"]
impl crate::ResetValue for super::OC1RINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC1R`"]
pub type OC1R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC1R`"]
pub struct OC1R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc1r(&self) -> OC1R_R {
        OC1R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc1r(&mut self) -> OC1R_W {
        OC1R_W { w: self }
    }
}
