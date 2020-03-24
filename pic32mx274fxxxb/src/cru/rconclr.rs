#[doc = "Reader of register RCONCLR"]
pub type R = crate::R<u32, super::RCONCLR>;
#[doc = "Writer for register RCONCLR"]
pub type W = crate::W<u32, super::RCONCLR>;
#[doc = "Register RCONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POR`"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Reader of field `BOR`"]
pub type BOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOR`"]
pub struct BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_W<'a> {
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
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
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
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
#[doc = "Reader of field `WDTO`"]
pub type WDTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTO`"]
pub struct WDTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTO_W<'a> {
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
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
#[doc = "Reader of field `EXTR`"]
pub type EXTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTR`"]
pub struct EXTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTR_W<'a> {
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
#[doc = "Reader of field `CMR`"]
pub type CMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMR`"]
pub struct CMR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMR_W<'a> {
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
#[doc = "Reader of field `DPSLP`"]
pub type DPSLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPSLP`"]
pub struct DPSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_W<'a> {
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
#[doc = "Reader of field `VBAT`"]
pub type VBAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBAT`"]
pub struct VBAT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VBPOR`"]
pub type VBPOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBPOR`"]
pub struct VBPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBPOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BCFGFAIL`"]
pub type BCFGFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCFGFAIL`"]
pub struct BCFGFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BCFGFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BCFGERR`"]
pub type BCFGERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCFGERR`"]
pub struct BCFGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BCFGERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor(&self) -> BOR_R {
        BOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wdto(&self) -> WDTO_R {
        WDTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cmr(&self) -> CMR_R {
        CMR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dpslp(&self) -> DPSLP_R {
        DPSLP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vbat(&self) -> VBAT_R {
        VBAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vbpor(&self) -> VBPOR_R {
        VBPOR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bcfgfail(&self) -> BCFGFAIL_R {
        BCFGFAIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bcfgerr(&self) -> BCFGERR_R {
        BCFGERR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor(&mut self) -> BOR_W {
        BOR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wdto(&mut self) -> WDTO_W {
        WDTO_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extr(&mut self) -> EXTR_W {
        EXTR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cmr(&mut self) -> CMR_W {
        CMR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dpslp(&mut self) -> DPSLP_W {
        DPSLP_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vbat(&mut self) -> VBAT_W {
        VBAT_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vbpor(&mut self) -> VBPOR_W {
        VBPOR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bcfgfail(&mut self) -> BCFGFAIL_W {
        BCFGFAIL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bcfgerr(&mut self) -> BCFGERR_W {
        BCFGERR_W { w: self }
    }
}
