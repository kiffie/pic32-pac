#[doc = "Reader of register RPC4R"]
pub type R = crate::R<u32, super::RPC4R>;
#[doc = "Writer for register RPC4R"]
pub type W = crate::W<u32, super::RPC4R>;
#[doc = "Register RPC4R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC4R`"]
pub type RPC4R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC4R`"]
pub struct RPC4R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC4R_W<'a> {
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
    pub fn rpc4r(&self) -> RPC4R_R {
        RPC4R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc4r(&mut self) -> RPC4R_W {
        RPC4R_W { w: self }
    }
}
