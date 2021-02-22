#[doc = "Register `AD1CSSL` reader"]
pub struct R(crate::R<AD1CSSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AD1CSSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AD1CSSL_SPEC>> for R {
    fn from(reader: crate::R<AD1CSSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AD1CSSL` writer"]
pub struct W(crate::W<AD1CSSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AD1CSSL_SPEC>;
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
impl core::convert::From<crate::W<AD1CSSL_SPEC>> for W {
    fn from(writer: crate::W<AD1CSSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSSL` reader - "]
pub struct CSSL_R(crate::FieldReader<u16, u16>);
impl CSSL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CSSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSL` writer - "]
pub struct CSSL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSL_W<'a> {
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
    pub fn cssl(&self) -> CSSL_R {
        CSSL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cssl(&mut self) -> CSSL_W {
        CSSL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AD1CSSL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1cssl](index.html) module"]
pub struct AD1CSSL_SPEC;
impl crate::RegisterSpec for AD1CSSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ad1cssl::R](R) reader structure"]
impl crate::Readable for AD1CSSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ad1cssl::W](W) writer structure"]
impl crate::Writable for AD1CSSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AD1CSSL to value 0"]
impl crate::Resettable for AD1CSSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
