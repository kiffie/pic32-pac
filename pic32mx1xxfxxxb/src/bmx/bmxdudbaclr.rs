#[doc = "Reader of register BMXDUDBACLR"]
pub type R = crate::R<u32, super::BMXDUDBACLR>;
#[doc = "Writer for register BMXDUDBACLR"]
pub type W = crate::W<u32, super::BMXDUDBACLR>;
#[doc = "Register BMXDUDBACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::BMXDUDBACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMXDUDBA`"]
pub type BMXDUDBA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXDUDBA`"]
pub struct BMXDUDBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDUDBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdudba(&self) -> BMXDUDBA_R {
        BMXDUDBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdudba(&mut self) -> BMXDUDBA_W {
        BMXDUDBA_W { w: self }
    }
}
