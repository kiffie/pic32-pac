#[doc = "Register `DATSET` reader"]
pub struct R(crate::R<DATSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DATSET_SPEC>> for R {
    fn from(reader: crate::R<DATSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATSET` writer"]
pub struct W(crate::W<DATSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATSET_SPEC>;
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
impl core::convert::From<crate::W<DATSET_SPEC>> for W {
    fn from(writer: crate::W<DATSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCHPDAT` reader - "]
pub struct DCHPDAT_R(crate::FieldReader<u8, u8>);
impl DCHPDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCHPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCHPDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCHPDAT` writer - "]
pub struct DCHPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DCHPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dchpdat(&self) -> DCHPDAT_R {
        DCHPDAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dchpdat(&mut self) -> DCHPDAT_W {
        DCHPDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datset](index.html) module"]
pub struct DATSET_SPEC;
impl crate::RegisterSpec for DATSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datset::R](R) reader structure"]
impl crate::Readable for DATSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datset::W](W) writer structure"]
impl crate::Writable for DATSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATSET to value 0"]
impl crate::Resettable for DATSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
