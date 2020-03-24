#[doc = "Reader of register U1BDTP3SET"]
pub type R = crate::R<u32, super::U1BDTP3SET>;
#[doc = "Writer for register U1BDTP3SET"]
pub type W = crate::W<u32, super::U1BDTP3SET>;
#[doc = "Register U1BDTP3SET `reset()`'s with value 0"]
impl crate::ResetValue for super::U1BDTP3SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDTPTRU`"]
pub type BDTPTRU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDTPTRU`"]
pub struct BDTPTRU_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTPTRU_W<'a> {
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
    pub fn bdtptru(&self) -> BDTPTRU_R {
        BDTPTRU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdtptru(&mut self) -> BDTPTRU_W {
        BDTPTRU_W { w: self }
    }
}
