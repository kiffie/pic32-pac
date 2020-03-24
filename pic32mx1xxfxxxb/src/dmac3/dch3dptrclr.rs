#[doc = "Reader of register DCH3DPTRCLR"]
pub type R = crate::R<u32, super::DCH3DPTRCLR>;
#[doc = "Writer for register DCH3DPTRCLR"]
pub type W = crate::W<u32, super::DCH3DPTRCLR>;
#[doc = "Register DCH3DPTRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3DPTRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHDPTR`"]
pub type CHDPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHDPTR`"]
pub struct CHDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDPTR_W<'a> {
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
    pub fn chdptr(&self) -> CHDPTR_R {
        CHDPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdptr(&mut self) -> CHDPTR_W {
        CHDPTR_W { w: self }
    }
}
