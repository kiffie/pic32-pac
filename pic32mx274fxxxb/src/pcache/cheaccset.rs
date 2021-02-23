#[doc = "Register `CHEACCSET` reader"]
pub struct R(crate::R<CHEACCSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEACCSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHEACCSET_SPEC>> for R {
    fn from(reader: crate::R<CHEACCSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEACCSET` writer"]
pub struct W(crate::W<CHEACCSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEACCSET_SPEC>;
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
impl core::convert::From<crate::W<CHEACCSET_SPEC>> for W {
    fn from(writer: crate::W<CHEACCSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEIDX` reader - "]
pub struct CHEIDX_R(crate::FieldReader<u8, u8>);
impl CHEIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHEIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEIDX` writer - "]
pub struct CHEIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `CHEWEN` reader - "]
pub struct CHEWEN_R(crate::FieldReader<bool, bool>);
impl CHEWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEWEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEWEN` writer - "]
pub struct CHEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEWEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cheidx(&self) -> CHEIDX_R {
        CHEIDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chewen(&self) -> CHEWEN_R {
        CHEWEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cheidx(&mut self) -> CHEIDX_W {
        CHEIDX_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chewen(&mut self) -> CHEWEN_W {
        CHEWEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHEACCSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cheaccset](index.html) module"]
pub struct CHEACCSET_SPEC;
impl crate::RegisterSpec for CHEACCSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cheaccset::R](R) reader structure"]
impl crate::Readable for CHEACCSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cheaccset::W](W) writer structure"]
impl crate::Writable for CHEACCSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEACCSET to value 0"]
impl crate::Resettable for CHEACCSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
