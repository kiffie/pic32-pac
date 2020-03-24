#[doc = "Reader of register DCH0SSAINV"]
pub type R = crate::R<u32, super::DCH0SSAINV>;
#[doc = "Writer for register DCH0SSAINV"]
pub type W = crate::W<u32, super::DCH0SSAINV>;
#[doc = "Register DCH0SSAINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH0SSAINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH0SSA`"]
pub type DCH0SSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH0SSA`"]
pub struct DCH0SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH0SSA_W<'a> {
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
    pub fn dch0ssa(&self) -> DCH0SSA_R {
        DCH0SSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch0ssa(&mut self) -> DCH0SSA_W {
        DCH0SSA_W { w: self }
    }
}
