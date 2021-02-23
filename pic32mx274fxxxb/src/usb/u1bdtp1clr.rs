#[doc = "Register `U1BDTP1CLR` reader"]
pub struct R(crate::R<U1BDTP1CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1BDTP1CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1BDTP1CLR_SPEC>> for R {
    fn from(reader: crate::R<U1BDTP1CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1BDTP1CLR` writer"]
pub struct W(crate::W<U1BDTP1CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1BDTP1CLR_SPEC>;
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
impl core::convert::From<crate::W<U1BDTP1CLR_SPEC>> for W {
    fn from(writer: crate::W<U1BDTP1CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDTPTRL` reader - "]
pub struct BDTPTRL_R(crate::FieldReader<u8, u8>);
impl BDTPTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BDTPTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDTPTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDTPTRL` writer - "]
pub struct BDTPTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDTPTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn bdtptrl(&self) -> BDTPTRL_R {
        BDTPTRL_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7"]
    #[inline(always)]
    pub fn bdtptrl(&mut self) -> BDTPTRL_W {
        BDTPTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1BDTP1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp1clr](index.html) module"]
pub struct U1BDTP1CLR_SPEC;
impl crate::RegisterSpec for U1BDTP1CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1bdtp1clr::R](R) reader structure"]
impl crate::Readable for U1BDTP1CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1bdtp1clr::W](W) writer structure"]
impl crate::Writable for U1BDTP1CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1BDTP1CLR to value 0"]
impl crate::Resettable for U1BDTP1CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
