#[doc = "Reader of register DCH2SSASET"]
pub type R = crate::R<u32, super::DCH2SSASET>;
#[doc = "Writer for register DCH2SSASET"]
pub type W = crate::W<u32, super::DCH2SSASET>;
#[doc = "Register DCH2SSASET `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH2SSASET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH2SSA`"]
pub type DCH2SSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH2SSA`"]
pub struct DCH2SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH2SSA_W<'a> {
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
    pub fn dch2ssa(&self) -> DCH2SSA_R {
        DCH2SSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch2ssa(&mut self) -> DCH2SSA_W {
        DCH2SSA_W { w: self }
    }
}
