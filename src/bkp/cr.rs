#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPE`"]
pub type TPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPE`"]
pub struct TPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPE_W<'a> {
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
#[doc = "Reader of field `TPAL`"]
pub type TPAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPAL`"]
pub struct TPAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPAL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W {
        TPE_W { w: self }
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W { w: self }
    }
}
