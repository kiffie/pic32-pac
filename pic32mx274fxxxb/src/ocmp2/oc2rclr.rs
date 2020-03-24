#[doc = "Reader of register OC2RCLR"]
pub type R = crate::R<u32, super::OC2RCLR>;
#[doc = "Writer for register OC2RCLR"]
pub type W = crate::W<u32, super::OC2RCLR>;
#[doc = "Register OC2RCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OC2RCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC2R`"]
pub type OC2R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC2R`"]
pub struct OC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2R_W<'a> {
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
    pub fn oc2r(&self) -> OC2R_R {
        OC2R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc2r(&mut self) -> OC2R_W {
        OC2R_W { w: self }
    }
}
