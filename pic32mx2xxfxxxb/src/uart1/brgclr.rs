#[doc = "Reader of register BRGCLR"]
pub type R = crate::R<u32, super::BRGCLR>;
#[doc = "Writer for register BRGCLR"]
pub type W = crate::W<u32, super::BRGCLR>;
#[doc = "Register BRGCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::BRGCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BRG`"]
pub type BRG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRG`"]
pub struct BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRG_W<'a> {
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
    pub fn brg(&self) -> BRG_R {
        BRG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn brg(&mut self) -> BRG_W {
        BRG_W { w: self }
    }
}
