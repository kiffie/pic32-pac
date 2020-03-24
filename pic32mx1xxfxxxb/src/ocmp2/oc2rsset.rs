#[doc = "Reader of register OC2RSSET"]
pub type R = crate::R<u32, super::OC2RSSET>;
#[doc = "Writer for register OC2RSSET"]
pub type W = crate::W<u32, super::OC2RSSET>;
#[doc = "Register OC2RSSET `reset()`'s with value 0"]
impl crate::ResetValue for super::OC2RSSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC2RS`"]
pub type OC2RS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC2RS`"]
pub struct OC2RS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2RS_W<'a> {
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
    pub fn oc2rs(&self) -> OC2RS_R {
        OC2RS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc2rs(&mut self) -> OC2RS_W {
        OC2RS_W { w: self }
    }
}
