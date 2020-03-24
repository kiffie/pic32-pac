#[doc = "Reader of register RPA8R"]
pub type R = crate::R<u32, super::RPA8R>;
#[doc = "Writer for register RPA8R"]
pub type W = crate::W<u32, super::RPA8R>;
#[doc = "Register RPA8R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPA8R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPA8R`"]
pub type RPA8R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPA8R`"]
pub struct RPA8R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPA8R_W<'a> {
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
    pub fn rpa8r(&self) -> RPA8R_R {
        RPA8R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpa8r(&mut self) -> RPA8R_W {
        RPA8R_W { w: self }
    }
}
