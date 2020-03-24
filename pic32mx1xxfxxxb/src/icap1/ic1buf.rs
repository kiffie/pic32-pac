#[doc = "Reader of register IC1BUF"]
pub type R = crate::R<u32, super::IC1BUF>;
#[doc = "Writer for register IC1BUF"]
pub type W = crate::W<u32, super::IC1BUF>;
#[doc = "Register IC1BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::IC1BUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC1BUF`"]
pub type IC1BUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IC1BUF`"]
pub struct IC1BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1BUF_W<'a> {
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
    pub fn ic1buf(&self) -> IC1BUF_R {
        IC1BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic1buf(&mut self) -> IC1BUF_W {
        IC1BUF_W { w: self }
    }
}
