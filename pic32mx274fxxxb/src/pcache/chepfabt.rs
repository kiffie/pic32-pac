#[doc = "Register `CHEPFABT` reader"]
pub struct R(crate::R<CHEPFABT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEPFABT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHEPFABT_SPEC>> for R {
    fn from(reader: crate::R<CHEPFABT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEPFABT` writer"]
pub struct W(crate::W<CHEPFABT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEPFABT_SPEC>;
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
impl core::convert::From<crate::W<CHEPFABT_SPEC>> for W {
    fn from(writer: crate::W<CHEPFABT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEPFABT` reader - "]
pub struct CHEPFABT_R(crate::FieldReader<u32, u32>);
impl CHEPFABT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHEPFABT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEPFABT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEPFABT` writer - "]
pub struct CHEPFABT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEPFABT_W<'a> {
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
    pub fn chepfabt(&self) -> CHEPFABT_R {
        CHEPFABT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chepfabt(&mut self) -> CHEPFABT_W {
        CHEPFABT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHEPFABT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chepfabt](index.html) module"]
pub struct CHEPFABT_SPEC;
impl crate::RegisterSpec for CHEPFABT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chepfabt::R](R) reader structure"]
impl crate::Readable for CHEPFABT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chepfabt::W](W) writer structure"]
impl crate::Writable for CHEPFABT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEPFABT to value 0"]
impl crate::Resettable for CHEPFABT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
