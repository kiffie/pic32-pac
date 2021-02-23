#[doc = "Register `PMMODECLR` reader"]
pub struct R(crate::R<PMMODECLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMODECLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMMODECLR_SPEC>> for R {
    fn from(reader: crate::R<PMMODECLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMODECLR` writer"]
pub struct W(crate::W<PMMODECLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMODECLR_SPEC>;
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
impl core::convert::From<crate::W<PMMODECLR_SPEC>> for W {
    fn from(writer: crate::W<PMMODECLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITE` reader - "]
pub struct WAITE_R(crate::FieldReader<u8, u8>);
impl WAITE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITE` writer - "]
pub struct WAITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `WAITM` reader - "]
pub struct WAITM_R(crate::FieldReader<u8, u8>);
impl WAITM_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITM` writer - "]
pub struct WAITM_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `WAITB` reader - "]
pub struct WAITB_R(crate::FieldReader<u8, u8>);
impl WAITB_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAITB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITB` writer - "]
pub struct WAITB_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `MODE` reader - "]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - "]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `INCM` reader - "]
pub struct INCM_R(crate::FieldReader<u8, u8>);
impl INCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        INCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCM` writer - "]
pub struct INCM_W<'a> {
    w: &'a mut W,
}
impl<'a> INCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `IRQM` reader - "]
pub struct IRQM_R(crate::FieldReader<u8, u8>);
impl IRQM_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQM` writer - "]
pub struct IRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `BUSY` reader - "]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - "]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn waite(&self) -> WAITE_R {
        WAITE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn waitm(&self) -> WAITM_R {
        WAITM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn waitb(&self) -> WAITB_R {
        WAITB_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn incm(&self) -> INCM_R {
        INCM_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn irqm(&self) -> IRQM_R {
        IRQM_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn waite(&mut self) -> WAITE_W {
        WAITE_W { w: self }
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn waitm(&mut self) -> WAITM_W {
        WAITM_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn waitb(&mut self) -> WAITB_W {
        WAITB_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn incm(&mut self) -> INCM_W {
        INCM_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn irqm(&mut self) -> IRQM_W {
        IRQM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMMODECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmodeclr](index.html) module"]
pub struct PMMODECLR_SPEC;
impl crate::RegisterSpec for PMMODECLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmmodeclr::R](R) reader structure"]
impl crate::Readable for PMMODECLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmodeclr::W](W) writer structure"]
impl crate::Writable for PMMODECLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMODECLR to value 0"]
impl crate::Resettable for PMMODECLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
