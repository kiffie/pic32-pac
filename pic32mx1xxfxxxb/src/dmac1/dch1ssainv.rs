#[doc = "Reader of register DCH1SSAINV"]
pub type R = crate::R<u32, super::DCH1SSAINV>;
#[doc = "Writer for register DCH1SSAINV"]
pub type W = crate::W<u32, super::DCH1SSAINV>;
#[doc = "Register DCH1SSAINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH1SSAINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH1SSA`"]
pub type DCH1SSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH1SSA`"]
pub struct DCH1SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH1SSA_W<'a> {
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
    pub fn dch1ssa(&self) -> DCH1SSA_R {
        DCH1SSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch1ssa(&mut self) -> DCH1SSA_W {
        DCH1SSA_W { w: self }
    }
}