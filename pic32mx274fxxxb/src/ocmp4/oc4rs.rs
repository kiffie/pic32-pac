#[doc = "Reader of register OC4RS"]
pub type R = crate::R<u32, super::OC4RS>;
#[doc = "Writer for register OC4RS"]
pub type W = crate::W<u32, super::OC4RS>;
#[doc = "Register OC4RS `reset()`'s with value 0"]
impl crate::ResetValue for super::OC4RS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC4RS`"]
pub type OC4RS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC4RS`"]
pub struct OC4RS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4RS_W<'a> {
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
    pub fn oc4rs(&self) -> OC4RS_R {
        OC4RS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc4rs(&mut self) -> OC4RS_W {
        OC4RS_W { w: self }
    }
}
