#[doc = "Reader of register STATCLR"]
pub type R = crate::R<u32, super::STATCLR>;
#[doc = "Writer for register STATCLR"]
pub type W = crate::W<u32, super::STATCLR>;
#[doc = "Register STATCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::STATCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBF`"]
pub type TBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBF`"]
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
#[doc = "Reader of field `RBF`"]
pub type RBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBF`"]
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
#[doc = "Reader of field `R_W`"]
pub type R_W_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R_W`"]
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
#[doc = "Reader of field `S`"]
pub type S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S`"]
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
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P`"]
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
#[doc = "Reader of field `D_A`"]
pub type D_A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D_A`"]
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
#[doc = "Reader of field `I2COV`"]
pub type I2COV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2COV`"]
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
#[doc = "Reader of field `IWCOL`"]
pub type IWCOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWCOL`"]
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
#[doc = "Reader of field `ADD10`"]
pub type ADD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADD10`"]
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
#[doc = "Reader of field `GCSTAT`"]
pub type GCSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCSTAT`"]
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
#[doc = "Reader of field `BCL`"]
pub type BCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCL`"]
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
#[doc = "Reader of field `TRSTAT`"]
pub type TRSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRSTAT`"]
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
#[doc = "Reader of field `ACKSTAT`"]
pub type ACKSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKSTAT`"]
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
}
