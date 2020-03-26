#[doc = "Reader of register IC2BUF"]
pub type R = crate::R<u32, super::IC2BUF>;
#[doc = "Writer for register IC2BUF"]
pub type W = crate::W<u32, super::IC2BUF>;
#[doc = "Register IC2BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::IC2BUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC2BUF`"]
pub type IC2BUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IC2BUF`"]
pub struct IC2BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2BUF_W<'a> {
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
    pub fn ic2buf(&self) -> IC2BUF_R {
        IC2BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic2buf(&mut self) -> IC2BUF_W {
        IC2BUF_W { w: self }
    }
}
