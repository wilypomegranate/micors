#[doc = "Reader of register DR25"]
pub type R = crate::R<u32, super::DR25>;
#[doc = "Writer for register DR25"]
pub type W = crate::W<u32, super::DR25>;
#[doc = "Register DR25 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D25`"]
pub type D25_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D25`"]
pub struct D25_W<'a> {
    w: &'a mut W,
}
impl<'a> D25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d25(&self) -> D25_R {
        D25_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d25(&mut self) -> D25_W {
        D25_W { w: self }
    }
}
