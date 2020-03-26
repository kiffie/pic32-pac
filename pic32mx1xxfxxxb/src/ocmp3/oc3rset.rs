#[doc = "Reader of register OC3RSET"]
pub type R = crate::R<u32, super::OC3RSET>;
#[doc = "Writer for register OC3RSET"]
pub type W = crate::W<u32, super::OC3RSET>;
#[doc = "Register OC3RSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OC3RSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC3R`"]
pub type OC3R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC3R`"]
pub struct OC3R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3R_W<'a> {
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
    pub fn oc3r(&self) -> OC3R_R {
        OC3R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc3r(&mut self) -> OC3R_W {
        OC3R_W { w: self }
    }
}