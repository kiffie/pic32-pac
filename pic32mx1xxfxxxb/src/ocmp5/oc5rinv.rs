#[doc = "Reader of register OC5RINV"]
pub type R = crate::R<u32, super::OC5RINV>;
#[doc = "Writer for register OC5RINV"]
pub type W = crate::W<u32, super::OC5RINV>;
#[doc = "Register OC5RINV `reset()`'s with value 0"]
impl crate::ResetValue for super::OC5RINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC5R`"]
pub type OC5R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC5R`"]
pub struct OC5R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5R_W<'a> {
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
    pub fn oc5r(&self) -> OC5R_R {
        OC5R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc5r(&mut self) -> OC5R_W {
        OC5R_W { w: self }
    }
}
