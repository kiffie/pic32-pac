#[doc = "Reader of register CHELRU"]
pub type R = crate::R<u32, super::CHELRU>;
#[doc = "Writer for register CHELRU"]
pub type W = crate::W<u32, super::CHELRU>;
#[doc = "Register CHELRU `reset()`'s with value 0"]
impl crate::ResetValue for super::CHELRU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHELRU`"]
pub type CHELRU_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHELRU`"]
pub struct CHELRU_W<'a> {
    w: &'a mut W,
}
impl<'a> CHELRU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn chelru(&self) -> CHELRU_R {
        CHELRU_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn chelru(&mut self) -> CHELRU_W {
        CHELRU_W { w: self }
    }
}
