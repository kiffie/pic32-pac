#[doc = "Reader of register RPB5R"]
pub type R = crate::R<u32, super::RPB5R>;
#[doc = "Writer for register RPB5R"]
pub type W = crate::W<u32, super::RPB5R>;
#[doc = "Register RPB5R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB5R`"]
pub type RPB5R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB5R`"]
pub struct RPB5R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB5R_W<'a> {
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
    pub fn rpb5r(&self) -> RPB5R_R {
        RPB5R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb5r(&mut self) -> RPB5R_W {
        RPB5R_W { w: self }
    }
}
