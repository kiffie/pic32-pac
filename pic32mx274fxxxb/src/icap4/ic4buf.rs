#[doc = "Reader of register IC4BUF"]
pub type R = crate::R<u32, super::IC4BUF>;
#[doc = "Writer for register IC4BUF"]
pub type W = crate::W<u32, super::IC4BUF>;
#[doc = "Register IC4BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::IC4BUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC4BUF`"]
pub type IC4BUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IC4BUF`"]
pub struct IC4BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4BUF_W<'a> {
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
    pub fn ic4buf(&self) -> IC4BUF_R {
        IC4BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic4buf(&mut self) -> IC4BUF_W {
        IC4BUF_W { w: self }
    }
}
