#[doc = "Reader of register DR30"]
pub type R = crate::R<u32, super::DR30>;
#[doc = "Writer for register DR30"]
pub type W = crate::W<u32, super::DR30>;
#[doc = "Register DR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D30`"]
pub type D30_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D30`"]
pub struct D30_W<'a> {
    w: &'a mut W,
}
impl<'a> D30_W<'a> {
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
    pub fn d30(&self) -> D30_R {
        D30_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d30(&mut self) -> D30_W {
        D30_W { w: self }
    }
}
