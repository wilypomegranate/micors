#[doc = "Reader of register DR21"]
pub type R = crate::R<u32, super::DR21>;
#[doc = "Writer for register DR21"]
pub type W = crate::W<u32, super::DR21>;
#[doc = "Register DR21 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR21 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D21`"]
pub type D21_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D21`"]
pub struct D21_W<'a> {
    w: &'a mut W,
}
impl<'a> D21_W<'a> {
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
    pub fn d21(&self) -> D21_R {
        D21_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d21(&mut self) -> D21_W {
        D21_W { w: self }
    }
}
