#[doc = "Reader of register DR10"]
pub type R = crate::R<u32, super::DR10>;
#[doc = "Writer for register DR10"]
pub type W = crate::W<u32, super::DR10>;
#[doc = "Register DR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D10`"]
pub type D10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D10`"]
pub struct D10_W<'a> {
    w: &'a mut W,
}
impl<'a> D10_W<'a> {
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
    pub fn d10(&self) -> D10_R {
        D10_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d10(&mut self) -> D10_W {
        D10_W { w: self }
    }
}
