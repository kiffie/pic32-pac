#[doc = "Register `RPC7R` reader"]
pub struct R(crate::R<RPC7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPC7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPC7R_SPEC>> for R {
    fn from(reader: crate::R<RPC7R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPC7R` writer"]
pub struct W(crate::W<RPC7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPC7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<RPC7R_SPEC>> for W {
    fn from(writer: crate::W<RPC7R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPC7R` reader - "]
pub struct RPC7R_R(crate::FieldReader<u8, u8>);
impl RPC7R_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPC7R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPC7R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPC7R` writer - "]
pub struct RPC7R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC7R_W<'a> {
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
    pub fn rpc7r(&self) -> RPC7R_R {
        RPC7R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc7r(&mut self) -> RPC7R_W {
        RPC7R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RPC7R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc7r](index.html) module"]
pub struct RPC7R_SPEC;
impl crate::RegisterSpec for RPC7R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpc7r::R](R) reader structure"]
impl crate::Readable for RPC7R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpc7r::W](W) writer structure"]
impl crate::Writable for RPC7R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPC7R to value 0"]
impl crate::Resettable for RPC7R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
