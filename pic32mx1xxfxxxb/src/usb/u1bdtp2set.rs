#[doc = "Reader of register U1BDTP2SET"]
pub type R = crate::R<u32, super::U1BDTP2SET>;
#[doc = "Writer for register U1BDTP2SET"]
pub type W = crate::W<u32, super::U1BDTP2SET>;
#[doc = "Register U1BDTP2SET `reset()`'s with value 0"]
impl crate::ResetValue for super::U1BDTP2SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDTPTRH`"]
pub type BDTPTRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDTPTRH`"]
pub struct BDTPTRH_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTPTRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdtptrh(&self) -> BDTPTRH_R {
        BDTPTRH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdtptrh(&mut self) -> BDTPTRH_W {
        BDTPTRH_W { w: self }
    }
}
