#[doc = "Reader of register AD1CSSLSET"]
pub type R = crate::R<u32, super::AD1CSSLSET>;
#[doc = "Writer for register AD1CSSLSET"]
pub type W = crate::W<u32, super::AD1CSSLSET>;
#[doc = "Register AD1CSSLSET `reset()`'s with value 0"]
impl crate::ResetValue for super::AD1CSSLSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSSL`"]
pub type CSSL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSSL`"]
pub struct CSSL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cssl(&self) -> CSSL_R {
        CSSL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cssl(&mut self) -> CSSL_W {
        CSSL_W { w: self }
    }
}
