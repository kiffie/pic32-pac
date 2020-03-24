#[doc = "Reader of register RPB13R"]
pub type R = crate::R<u32, super::RPB13R>;
#[doc = "Writer for register RPB13R"]
pub type W = crate::W<u32, super::RPB13R>;
#[doc = "Register RPB13R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB13R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB13R`"]
pub type RPB13R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB13R`"]
pub struct RPB13R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB13R_W<'a> {
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
    pub fn rpb13r(&self) -> RPB13R_R {
        RPB13R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb13r(&mut self) -> RPB13R_W {
        RPB13R_W { w: self }
    }
}
