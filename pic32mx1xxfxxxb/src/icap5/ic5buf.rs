#[doc = "Reader of register IC5BUF"]
pub type R = crate::R<u32, super::IC5BUF>;
#[doc = "Writer for register IC5BUF"]
pub type W = crate::W<u32, super::IC5BUF>;
#[doc = "Register IC5BUF `reset()`'s with value 0"]
impl crate::ResetValue for super::IC5BUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC5BUF`"]
pub type IC5BUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IC5BUF`"]
pub struct IC5BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5BUF_W<'a> {
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
    pub fn ic5buf(&self) -> IC5BUF_R {
        IC5BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ic5buf(&mut self) -> IC5BUF_W {
        IC5BUF_W { w: self }
    }
}
