#[doc = "Register `U1STAT` reader"]
pub struct R(crate::R<U1STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1STAT_SPEC>> for R {
    fn from(reader: crate::R<U1STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1STAT` writer"]
pub struct W(crate::W<U1STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1STAT_SPEC>;
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
impl core::convert::From<crate::W<U1STAT_SPEC>> for W {
    fn from(writer: crate::W<U1STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPBI` reader - "]
pub struct PPBI_R(crate::FieldReader<bool, bool>);
impl PPBI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPBI` writer - "]
pub struct PPBI_W<'a> {
    w: &'a mut W,
}
impl<'a> PPBI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DIR` reader - "]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - "]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ENDPT` reader - "]
pub struct ENDPT_R(crate::FieldReader<u8, u8>);
impl ENDPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENDPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDPT` writer - "]
pub struct ENDPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ppbi(&self) -> PPBI_R {
        PPBI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn endpt(&self) -> ENDPT_R {
        ENDPT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ppbi(&mut self) -> PPBI_W {
        PPBI_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn endpt(&mut self) -> ENDPT_W {
        ENDPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1stat](index.html) module"]
pub struct U1STAT_SPEC;
impl crate::RegisterSpec for U1STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1stat::R](R) reader structure"]
impl crate::Readable for U1STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1stat::W](W) writer structure"]
impl crate::Writable for U1STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1STAT to value 0"]
impl crate::Resettable for U1STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
