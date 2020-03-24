#[doc = "Reader of register RPB11R"]
pub type R = crate::R<u32, super::RPB11R>;
#[doc = "Writer for register RPB11R"]
pub type W = crate::W<u32, super::RPB11R>;
#[doc = "Register RPB11R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB11R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB11R`"]
pub type RPB11R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB11R`"]
pub struct RPB11R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB11R_W<'a> {
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
    pub fn rpb11r(&self) -> RPB11R_R {
        RPB11R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb11r(&mut self) -> RPB11R_W {
        RPB11R_W { w: self }
    }
}
