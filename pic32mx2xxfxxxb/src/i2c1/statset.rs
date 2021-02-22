#[doc = "Register `STATSET` reader"]
pub struct R(crate::R<STATSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATSET_SPEC>> for R {
    fn from(reader: crate::R<STATSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATSET` writer"]
pub struct W(crate::W<STATSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATSET_SPEC>;
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
impl core::convert::From<crate::W<STATSET_SPEC>> for W {
    fn from(writer: crate::W<STATSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBF` reader - "]
pub struct TBF_R(crate::FieldReader<bool, bool>);
impl TBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBF` writer - "]
pub struct TBF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBF_W<'a> {
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
#[doc = "Field `RBF` reader - "]
pub struct RBF_R(crate::FieldReader<bool, bool>);
impl RBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBF` writer - "]
pub struct RBF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBF_W<'a> {
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
#[doc = "Field `R_W` reader - "]
pub struct R_W_R(crate::FieldReader<bool, bool>);
impl R_W_R {
    pub(crate) fn new(bits: bool) -> Self {
        R_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_W` writer - "]
pub struct R_W_W<'a> {
    w: &'a mut W,
}
impl<'a> R_W_W<'a> {
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
#[doc = "Field `S` reader - "]
pub struct S_R(crate::FieldReader<bool, bool>);
impl S_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` writer - "]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
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
#[doc = "Field `P` reader - "]
pub struct P_R(crate::FieldReader<bool, bool>);
impl P_R {
    pub(crate) fn new(bits: bool) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P` writer - "]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `D_A` reader - "]
pub struct D_A_R(crate::FieldReader<bool, bool>);
impl D_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        D_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_A` writer - "]
pub struct D_A_W<'a> {
    w: &'a mut W,
}
impl<'a> D_A_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `I2COV` reader - "]
pub struct I2COV_R(crate::FieldReader<bool, bool>);
impl I2COV_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2COV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2COV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2COV` writer - "]
pub struct I2COV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2COV_W<'a> {
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
#[doc = "Field `IWCOL` reader - "]
pub struct IWCOL_R(crate::FieldReader<bool, bool>);
impl IWCOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWCOL` writer - "]
pub struct IWCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IWCOL_W<'a> {
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
#[doc = "Field `ADD10` reader - "]
pub struct ADD10_R(crate::FieldReader<bool, bool>);
impl ADD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD10` writer - "]
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
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
#[doc = "Field `GCSTAT` reader - "]
pub struct GCSTAT_R(crate::FieldReader<bool, bool>);
impl GCSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCSTAT` writer - "]
pub struct GCSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> GCSTAT_W<'a> {
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
#[doc = "Field `BCL` reader - "]
pub struct BCL_R(crate::FieldReader<bool, bool>);
impl BCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCL` writer - "]
pub struct BCL_W<'a> {
    w: &'a mut W,
}
impl<'a> BCL_W<'a> {
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
#[doc = "Field `TRSTAT` reader - "]
pub struct TRSTAT_R(crate::FieldReader<bool, bool>);
impl TRSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRSTAT` writer - "]
pub struct TRSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSTAT_W<'a> {
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
#[doc = "Field `ACKSTAT` reader - "]
pub struct ACKSTAT_R(crate::FieldReader<bool, bool>);
impl ACKSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKSTAT` writer - "]
pub struct ACKSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKSTAT_W<'a> {
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
    pub fn tbf(&self) -> TBF_R {
        TBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rbf(&self) -> RBF_R {
        RBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_w(&self) -> R_W_R {
        R_W_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_a(&self) -> D_A_R {
        D_A_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2cov(&self) -> I2COV_R {
        I2COV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iwcol(&self) -> IWCOL_R {
        IWCOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gcstat(&self) -> GCSTAT_R {
        GCSTAT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bcl(&self) -> BCL_R {
        BCL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trstat(&self) -> TRSTAT_R {
        TRSTAT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ackstat(&self) -> ACKSTAT_R {
        ACKSTAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tbf(&mut self) -> TBF_W {
        TBF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rbf(&mut self) -> RBF_W {
        RBF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn r_w(&mut self) -> R_W_W {
        R_W_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_a(&mut self) -> D_A_W {
        D_A_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2cov(&mut self) -> I2COV_W {
        I2COV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn iwcol(&mut self) -> IWCOL_W {
        IWCOL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gcstat(&mut self) -> GCSTAT_W {
        GCSTAT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bcl(&mut self) -> BCL_W {
        BCL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trstat(&mut self) -> TRSTAT_W {
        TRSTAT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ackstat(&mut self) -> ACKSTAT_W {
        ACKSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C1STATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statset](index.html) module"]
pub struct STATSET_SPEC;
impl crate::RegisterSpec for STATSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statset::R](R) reader structure"]
impl crate::Readable for STATSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statset::W](W) writer structure"]
impl crate::Writable for STATSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATSET to value 0"]
impl crate::Resettable for STATSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
