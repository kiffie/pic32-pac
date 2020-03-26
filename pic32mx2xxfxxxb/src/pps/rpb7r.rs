#[doc = "Reader of register RPB7R"]
pub type R = crate::R<u32, super::RPB7R>;
#[doc = "Writer for register RPB7R"]
pub type W = crate::W<u32, super::RPB7R>;
#[doc = "Register RPB7R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB7R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB7R`"]
pub type RPB7R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB7R`"]
pub struct RPB7R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB7R_W<'a> {
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
    pub fn rpb7r(&self) -> RPB7R_R {
        RPB7R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb7r(&mut self) -> RPB7R_W {
        RPB7R_W { w: self }
    }
}
