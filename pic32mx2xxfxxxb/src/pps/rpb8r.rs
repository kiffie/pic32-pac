#[doc = "Reader of register RPB8R"]
pub type R = crate::R<u32, super::RPB8R>;
#[doc = "Writer for register RPB8R"]
pub type W = crate::W<u32, super::RPB8R>;
#[doc = "Register RPB8R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB8R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB8R`"]
pub type RPB8R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB8R`"]
pub struct RPB8R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB8R_W<'a> {
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
    pub fn rpb8r(&self) -> RPB8R_R {
        RPB8R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb8r(&mut self) -> RPB8R_W {
        RPB8R_W { w: self }
    }
}
