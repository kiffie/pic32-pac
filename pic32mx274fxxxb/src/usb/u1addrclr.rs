#[doc = "Reader of register U1ADDRCLR"]
pub type R = crate::R<u32, super::U1ADDRCLR>;
#[doc = "Writer for register U1ADDRCLR"]
pub type W = crate::W<u32, super::U1ADDRCLR>;
#[doc = "Register U1ADDRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1ADDRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVADDR`"]
pub type DEVADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVADDR`"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `LSPDEN`"]
pub type LSPDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSPDEN`"]
pub struct LSPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPDEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lspden(&self) -> LSPDEN_R {
        LSPDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lspden(&mut self) -> LSPDEN_W {
        LSPDEN_W { w: self }
    }
}
