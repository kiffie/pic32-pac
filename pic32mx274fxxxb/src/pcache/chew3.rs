#[doc = "Register `CHEW3` reader"]
pub struct R(crate::R<CHEW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHEW3_SPEC>> for R {
    fn from(reader: crate::R<CHEW3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEW3` writer"]
pub struct W(crate::W<CHEW3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEW3_SPEC>;
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
impl core::convert::From<crate::W<CHEW3_SPEC>> for W {
    fn from(writer: crate::W<CHEW3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEW3` reader - "]
pub struct CHEW3_R(crate::FieldReader<u32, u32>);
impl CHEW3_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHEW3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEW3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEW3` writer - "]
pub struct CHEW3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEW3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chew3(&self) -> CHEW3_R {
        CHEW3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chew3(&mut self) -> CHEW3_W {
        CHEW3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHEW3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chew3](index.html) module"]
pub struct CHEW3_SPEC;
impl crate::RegisterSpec for CHEW3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chew3::R](R) reader structure"]
impl crate::Readable for CHEW3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chew3::W](W) writer structure"]
impl crate::Writable for CHEW3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEW3 to value 0"]
impl crate::Resettable for CHEW3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
