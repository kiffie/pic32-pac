#[doc = "Reader of register RPC9R"]
pub type R = crate::R<u32, super::RPC9R>;
#[doc = "Writer for register RPC9R"]
pub type W = crate::W<u32, super::RPC9R>;
#[doc = "Register RPC9R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC9R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC9R`"]
pub type RPC9R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC9R`"]
pub struct RPC9R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC9R_W<'a> {
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
    pub fn rpc9r(&self) -> RPC9R_R {
        RPC9R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc9r(&mut self) -> RPC9R_W {
        RPC9R_W { w: self }
    }
}
