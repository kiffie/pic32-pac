#[doc = "Register `TXREG` reader"]
pub struct R(crate::R<TXREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TXREG_SPEC>> for R {
    fn from(reader: crate::R<TXREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXREG` writer"]
pub struct W(crate::W<TXREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXREG_SPEC>;
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
impl core::convert::From<crate::W<TXREG_SPEC>> for W {
    fn from(writer: crate::W<TXREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXREG` reader - "]
pub struct TXREG_R(crate::FieldReader<u16, u16>);
impl TXREG_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXREG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXREG` writer - "]
pub struct TXREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREG_W<'a> {
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
    pub fn txreg(&self) -> TXREG_R {
        TXREG_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn txreg(&mut self) -> TXREG_W {
        TXREG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1TXREG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txreg](index.html) module"]
pub struct TXREG_SPEC;
impl crate::RegisterSpec for TXREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txreg::R](R) reader structure"]
impl crate::Readable for TXREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txreg::W](W) writer structure"]
impl crate::Writable for TXREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXREG to value 0"]
impl crate::Resettable for TXREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
