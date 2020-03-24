#[doc = "Reader of register BRGINV"]
pub type R = crate::R<u32, super::BRGINV>;
#[doc = "Writer for register BRGINV"]
pub type W = crate::W<u32, super::BRGINV>;
#[doc = "Register BRGINV `reset()`'s with value 0"]
impl crate::ResetValue for super::BRGINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRG`"]
pub type BRG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BRG`"]
pub struct BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn brg(&self) -> BRG_R {
        BRG_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn brg(&mut self) -> BRG_W {
        BRG_W { w: self }
    }
}
