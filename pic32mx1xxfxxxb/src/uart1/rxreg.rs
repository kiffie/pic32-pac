#[doc = "Register `RXREG` reader"]
pub struct R(crate::R<RXREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RXREG_SPEC>> for R {
    fn from(reader: crate::R<RXREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXREG` writer"]
pub struct W(crate::W<RXREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXREG_SPEC>;
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
impl core::convert::From<crate::W<RXREG_SPEC>> for W {
    fn from(writer: crate::W<RXREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXREG` reader - "]
pub struct RXREG_R(crate::FieldReader<u16, u16>);
impl RXREG_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXREG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXREG` writer - "]
pub struct RXREG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxreg(&self) -> RXREG_R {
        RXREG_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxreg(&mut self) -> RXREG_W {
        RXREG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1RXREG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxreg](index.html) module"]
pub struct RXREG_SPEC;
impl crate::RegisterSpec for RXREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxreg::R](R) reader structure"]
impl crate::Readable for RXREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxreg::W](W) writer structure"]
impl crate::Writable for RXREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXREG to value 0"]
impl crate::Resettable for RXREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
