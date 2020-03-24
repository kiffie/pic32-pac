#[doc = "Reader of register U1CONINV"]
pub type R = crate::R<u32, super::U1CONINV>;
#[doc = "Writer for register U1CONINV"]
pub type W = crate::W<u32, super::U1CONINV>;
#[doc = "Register U1CONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::U1CONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBEN_SOFEN`"]
pub type USBEN_SOFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBEN_SOFEN`"]
pub struct USBEN_SOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_SOFEN_W<'a> {
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
#[doc = "Reader of field `PPBRST`"]
pub type PPBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPBRST`"]
pub struct PPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PPBRST_W<'a> {
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
#[doc = "Reader of field `RESUME`"]
pub type RESUME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME`"]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Reader of field `HOSTEN`"]
pub type HOSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTEN`"]
pub struct HOSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTEN_W<'a> {
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
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Reader of field `PKTDIS_TOKBUSY`"]
pub type PKTDIS_TOKBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKTDIS_TOKBUSY`"]
pub struct PKTDIS_TOKBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDIS_TOKBUSY_W<'a> {
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
#[doc = "Reader of field `SE0`"]
pub type SE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE0`"]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
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
#[doc = "Reader of field `JSTATE`"]
pub type JSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JSTATE`"]
pub struct JSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTATE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usben_sofen(&self) -> USBEN_SOFEN_R {
        USBEN_SOFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ppbrst(&self) -> PPBRST_R {
        PPBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hosten(&self) -> HOSTEN_R {
        HOSTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pktdis_tokbusy(&self) -> PKTDIS_TOKBUSY_R {
        PKTDIS_TOKBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usben_sofen(&mut self) -> USBEN_SOFEN_W {
        USBEN_SOFEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ppbrst(&mut self) -> PPBRST_W {
        PPBRST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hosten(&mut self) -> HOSTEN_W {
        HOSTEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pktdis_tokbusy(&mut self) -> PKTDIS_TOKBUSY_W {
        PKTDIS_TOKBUSY_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jstate(&mut self) -> JSTATE_W {
        JSTATE_W { w: self }
    }
}
