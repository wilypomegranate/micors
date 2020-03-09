#[doc = "Reader of register RTCCR"]
pub type R = crate::R<u32, super::RTCCR>;
#[doc = "Writer for register RTCCR"]
pub type W = crate::W<u32, super::RTCCR>;
#[doc = "Register RTCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `CCO`"]
pub type CCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCO`"]
pub struct CCO_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_W<'a> {
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
#[doc = "Reader of field `ASOE`"]
pub type ASOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASOE`"]
pub struct ASOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOE_W<'a> {
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
#[doc = "Reader of field `ASOS`"]
pub type ASOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASOS`"]
pub struct ASOS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOS_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W {
        CCO_W { w: self }
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W {
        ASOE_W { w: self }
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W {
        ASOS_W { w: self }
    }
}
