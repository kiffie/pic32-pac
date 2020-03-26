#[doc = "Reader of register OCFAR"]
pub type R = crate::R<u32, super::OCFAR>;
#[doc = "Writer for register OCFAR"]
pub type W = crate::W<u32, super::OCFAR>;
#[doc = "Register OCFAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OCFAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCFAR`"]
pub type OCFAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCFAR`"]
pub struct OCFAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCFAR_W<'a> {
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
    pub fn ocfar(&self) -> OCFAR_R {
        OCFAR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ocfar(&mut self) -> OCFAR_W {
        OCFAR_W { w: self }
    }
}
