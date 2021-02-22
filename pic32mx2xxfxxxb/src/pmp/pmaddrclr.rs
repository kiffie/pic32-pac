#[doc = "Register `PMADDRCLR` reader"]
pub struct R(crate::R<PMADDRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMADDRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMADDRCLR_SPEC>> for R {
    fn from(reader: crate::R<PMADDRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMADDRCLR` writer"]
pub struct W(crate::W<PMADDRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMADDRCLR_SPEC>;
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
impl core::convert::From<crate::W<PMADDRCLR_SPEC>> for W {
    fn from(writer: crate::W<PMADDRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - "]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - "]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Field `CS` reader - "]
pub struct CS_R(crate::FieldReader<u8, u8>);
impl CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - "]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddrclr](index.html) module"]
pub struct PMADDRCLR_SPEC;
impl crate::RegisterSpec for PMADDRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmaddrclr::R](R) reader structure"]
impl crate::Readable for PMADDRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmaddrclr::W](W) writer structure"]
impl crate::Writable for PMADDRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMADDRCLR to value 0"]
impl crate::Resettable for PMADDRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
