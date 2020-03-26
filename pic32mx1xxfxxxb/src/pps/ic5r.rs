#[doc = "Reader of register IC5R"]
pub type R = crate::R<u32, super::IC5R>;
#[doc = "Writer for register IC5R"]
pub type W = crate::W<u32, super::IC5R>;
#[doc = "Register IC5R `reset()`'s with value 0"]
impl crate::ResetValue for super::IC5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC5R`"]
pub type IC5R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC5R`"]
pub struct IC5R_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ic5r(&self) -> IC5R_R {
        IC5R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ic5r(&mut self) -> IC5R_W {
        IC5R_W { w: self }
    }
}