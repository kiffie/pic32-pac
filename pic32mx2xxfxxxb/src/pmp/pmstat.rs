#[doc = "Register `PMSTAT` reader"]
pub struct R(crate::R<PMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMSTAT_SPEC>> for R {
    fn from(reader: crate::R<PMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMSTAT` writer"]
pub struct W(crate::W<PMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMSTAT_SPEC>;
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
impl core::convert::From<crate::W<PMSTAT_SPEC>> for W {
    fn from(writer: crate::W<PMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OB0E` reader - "]
pub struct OB0E_R(crate::FieldReader<bool, bool>);
impl OB0E_R {
    pub(crate) fn new(bits: bool) -> Self {
        OB0E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OB0E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OB0E` writer - "]
pub struct OB0E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB0E_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `OB1E` reader - "]
pub struct OB1E_R(crate::FieldReader<bool, bool>);
impl OB1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        OB1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OB1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OB1E` writer - "]
pub struct OB1E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB1E_W<'a> {
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
#[doc = "Field `OB2E` reader - "]
pub struct OB2E_R(crate::FieldReader<bool, bool>);
impl OB2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        OB2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OB2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OB2E` writer - "]
pub struct OB2E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB2E_W<'a> {
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
#[doc = "Field `OB3E` reader - "]
pub struct OB3E_R(crate::FieldReader<bool, bool>);
impl OB3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        OB3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OB3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OB3E` writer - "]
pub struct OB3E_W<'a> {
    w: &'a mut W,
}
impl<'a> OB3E_W<'a> {
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
#[doc = "Field `OBUF` reader - "]
pub struct OBUF_R(crate::FieldReader<bool, bool>);
impl OBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBUF` writer - "]
pub struct OBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> OBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `OBE` reader - "]
pub struct OBE_R(crate::FieldReader<bool, bool>);
impl OBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBE` writer - "]
pub struct OBE_W<'a> {
    w: &'a mut W,
}
impl<'a> OBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IB0F` reader - "]
pub struct IB0F_R(crate::FieldReader<bool, bool>);
impl IB0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IB0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IB0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IB0F` writer - "]
pub struct IB0F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB0F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `IB1F` reader - "]
pub struct IB1F_R(crate::FieldReader<bool, bool>);
impl IB1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IB1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IB1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IB1F` writer - "]
pub struct IB1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB1F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `IB2F` reader - "]
pub struct IB2F_R(crate::FieldReader<bool, bool>);
impl IB2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IB2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IB2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IB2F` writer - "]
pub struct IB2F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB2F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `IB3F` reader - "]
pub struct IB3F_R(crate::FieldReader<bool, bool>);
impl IB3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IB3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IB3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IB3F` writer - "]
pub struct IB3F_W<'a> {
    w: &'a mut W,
}
impl<'a> IB3F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `IBOV` reader - "]
pub struct IBOV_R(crate::FieldReader<bool, bool>);
impl IBOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBOV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBOV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBOV` writer - "]
pub struct IBOV_W<'a> {
    w: &'a mut W,
}
impl<'a> IBOV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `IBF` reader - "]
pub struct IBF_R(crate::FieldReader<bool, bool>);
impl IBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBF` writer - "]
pub struct IBF_W<'a> {
    w: &'a mut W,
}
impl<'a> IBF_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ob0e(&self) -> OB0E_R {
        OB0E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ob1e(&self) -> OB1E_R {
        OB1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ob2e(&self) -> OB2E_R {
        OB2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ob3e(&self) -> OB3E_R {
        OB3E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn obuf(&self) -> OBUF_R {
        OBUF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn obe(&self) -> OBE_R {
        OBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ib0f(&self) -> IB0F_R {
        IB0F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ib1f(&self) -> IB1F_R {
        IB1F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ib2f(&self) -> IB2F_R {
        IB2F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ib3f(&self) -> IB3F_R {
        IB3F_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ibov(&self) -> IBOV_R {
        IBOV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ibf(&self) -> IBF_R {
        IBF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ob0e(&mut self) -> OB0E_W {
        OB0E_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ob1e(&mut self) -> OB1E_W {
        OB1E_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ob2e(&mut self) -> OB2E_W {
        OB2E_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ob3e(&mut self) -> OB3E_W {
        OB3E_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn obuf(&mut self) -> OBUF_W {
        OBUF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn obe(&mut self) -> OBE_W {
        OBE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ib0f(&mut self) -> IB0F_W {
        IB0F_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ib1f(&mut self) -> IB1F_W {
        IB1F_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ib2f(&mut self) -> IB2F_W {
        IB2F_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ib3f(&mut self) -> IB3F_W {
        IB3F_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ibov(&mut self) -> IBOV_W {
        IBOV_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ibf(&mut self) -> IBF_W {
        IBF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstat](index.html) module"]
pub struct PMSTAT_SPEC;
impl crate::RegisterSpec for PMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmstat::R](R) reader structure"]
impl crate::Readable for PMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmstat::W](W) writer structure"]
impl crate::Writable for PMSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMSTAT to value 0x8f"]
impl crate::Resettable for PMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8f
    }
}
