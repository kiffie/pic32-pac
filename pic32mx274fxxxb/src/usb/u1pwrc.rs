#[doc = "Reader of register U1PWRC"]
pub type R = crate::R<u32, super::U1PWRC>;
#[doc = "Writer for register U1PWRC"]
pub type W = crate::W<u32, super::U1PWRC>;
#[doc = "Register U1PWRC `reset()`'s with value 0"]
impl crate::ResetValue for super::U1PWRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBPWR`"]
pub type USBPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBPWR`"]
pub struct USBPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPWR_W<'a> {
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
#[doc = "Reader of field `USUSPEND`"]
pub type USUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USUSPEND`"]
pub struct USUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USUSPEND_W<'a> {
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
#[doc = "Reader of field `USBBUSY`"]
pub type USBBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBBUSY`"]
pub struct USBBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBUSY_W<'a> {
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
#[doc = "Reader of field `USLPGRD`"]
pub type USLPGRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USLPGRD`"]
pub struct USLPGRD_W<'a> {
    w: &'a mut W,
}
impl<'a> USLPGRD_W<'a> {
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
#[doc = "Reader of field `UACTPND`"]
pub type UACTPND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UACTPND`"]
pub struct UACTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> UACTPND_W<'a> {
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
    pub fn usbpwr(&self) -> USBPWR_R {
        USBPWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ususpend(&self) -> USUSPEND_R {
        USUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbbusy(&self) -> USBBUSY_R {
        USBBUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uslpgrd(&self) -> USLPGRD_R {
        USLPGRD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uactpnd(&self) -> UACTPND_R {
        UACTPND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usbpwr(&mut self) -> USBPWR_W {
        USBPWR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ususpend(&mut self) -> USUSPEND_W {
        USUSPEND_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbbusy(&mut self) -> USBBUSY_W {
        USBBUSY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uslpgrd(&mut self) -> USLPGRD_W {
        USLPGRD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uactpnd(&mut self) -> UACTPND_W {
        UACTPND_W { w: self }
    }
}
