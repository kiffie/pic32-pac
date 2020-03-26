#[doc = "Reader of register RCV"]
pub type R = crate::R<u32, super::RCV>;
#[doc = "Writer for register RCV"]
pub type W = crate::W<u32, super::RCV>;
#[doc = "Register RCV `reset()`'s with value 0"]
impl crate::ResetValue for super::RCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCV`"]
pub type RCV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCV`"]
pub struct RCV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_W<'a> {
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
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rcv(&mut self) -> RCV_W {
        RCV_W { w: self }
    }
}
