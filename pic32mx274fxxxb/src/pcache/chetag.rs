#[doc = "Register `CHETAG` reader"]
pub struct R(crate::R<CHETAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHETAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHETAG_SPEC>> for R {
    fn from(reader: crate::R<CHETAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHETAG` writer"]
pub struct W(crate::W<CHETAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHETAG_SPEC>;
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
impl core::convert::From<crate::W<CHETAG_SPEC>> for W {
    fn from(writer: crate::W<CHETAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTYPE` reader - "]
pub struct LTYPE_R(crate::FieldReader<bool, bool>);
impl LTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTYPE` writer - "]
pub struct LTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LTYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LLOCK` reader - "]
pub struct LLOCK_R(crate::FieldReader<bool, bool>);
impl LLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLOCK` writer - "]
pub struct LLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LLOCK_W<'a> {
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
#[doc = "Field `LVALID` reader - "]
pub struct LVALID_R(crate::FieldReader<bool, bool>);
impl LVALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVALID` writer - "]
pub struct LVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> LVALID_W<'a> {
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
#[doc = "Field `LTAG` reader - "]
pub struct LTAG_R(crate::FieldReader<u32, u32>);
impl LTAG_R {
    pub(crate) fn new(bits: u32) -> Self {
        LTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTAG` writer - "]
pub struct LTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
#[doc = "Field `LTAGBOOT` reader - "]
pub struct LTAGBOOT_R(crate::FieldReader<bool, bool>);
impl LTAGBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTAGBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTAGBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTAGBOOT` writer - "]
pub struct LTAGBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> LTAGBOOT_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ltype(&self) -> LTYPE_R {
        LTYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn llock(&self) -> LLOCK_R {
        LLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lvalid(&self) -> LVALID_R {
        LVALID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn ltag(&self) -> LTAG_R {
        LTAG_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ltagboot(&self) -> LTAGBOOT_R {
        LTAGBOOT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ltype(&mut self) -> LTYPE_W {
        LTYPE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn llock(&mut self) -> LLOCK_W {
        LLOCK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lvalid(&mut self) -> LVALID_W {
        LVALID_W { w: self }
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn ltag(&mut self) -> LTAG_W {
        LTAG_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ltagboot(&mut self) -> LTAGBOOT_W {
        LTAGBOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHETAG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chetag](index.html) module"]
pub struct CHETAG_SPEC;
impl crate::RegisterSpec for CHETAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chetag::R](R) reader structure"]
impl crate::Readable for CHETAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chetag::W](W) writer structure"]
impl crate::Writable for CHETAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHETAG to value 0x02"]
impl crate::Resettable for CHETAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
