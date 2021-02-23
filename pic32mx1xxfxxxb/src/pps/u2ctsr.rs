#[doc = "Register `U2CTSR` reader"]
pub struct R(crate::R<U2CTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U2CTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U2CTSR_SPEC>> for R {
    fn from(reader: crate::R<U2CTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U2CTSR` writer"]
pub struct W(crate::W<U2CTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U2CTSR_SPEC>;
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
impl core::convert::From<crate::W<U2CTSR_SPEC>> for W {
    fn from(writer: crate::W<U2CTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U2CTSR` reader - "]
pub struct U2CTSR_R(crate::FieldReader<u8, u8>);
impl U2CTSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        U2CTSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2CTSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2CTSR` writer - "]
pub struct U2CTSR_W<'a> {
    w: &'a mut W,
}
impl<'a> U2CTSR_W<'a> {
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
    pub fn u2ctsr(&self) -> U2CTSR_R {
        U2CTSR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u2ctsr(&mut self) -> U2CTSR_W {
        U2CTSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U2CTSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2ctsr](index.html) module"]
pub struct U2CTSR_SPEC;
impl crate::RegisterSpec for U2CTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u2ctsr::R](R) reader structure"]
impl crate::Readable for U2CTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u2ctsr::W](W) writer structure"]
impl crate::Writable for U2CTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U2CTSR to value 0"]
impl crate::Resettable for U2CTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
