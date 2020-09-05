#[doc = "Reader of register SSASET"]
pub type R = crate::R<u32, super::SSASET>;
#[doc = "Writer for register SSASET"]
pub type W = crate::W<u32, super::SSASET>;
#[doc = "Register SSASET `reset()`'s with value 0"]
impl crate::ResetValue for super::SSASET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSA`"]
pub type SSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SSA`"]
pub struct SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> SSA_W<'a> {
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
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ssa(&mut self) -> SSA_W {
        SSA_W { w: self }
    }
}