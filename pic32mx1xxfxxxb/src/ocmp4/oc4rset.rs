#[doc = "Reader of register OC4RSET"]
pub type R = crate::R<u32, super::OC4RSET>;
#[doc = "Writer for register OC4RSET"]
pub type W = crate::W<u32, super::OC4RSET>;
#[doc = "Register OC4RSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OC4RSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC4R`"]
pub type OC4R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC4R`"]
pub struct OC4R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4R_W<'a> {
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
    pub fn oc4r(&self) -> OC4R_R {
        OC4R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc4r(&mut self) -> OC4R_W {
        OC4R_W { w: self }
    }
}
