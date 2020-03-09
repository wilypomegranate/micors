#[doc = "Reader of register DR33"]
pub type R = crate::R<u32, super::DR33>;
#[doc = "Writer for register DR33"]
pub type W = crate::W<u32, super::DR33>;
#[doc = "Register DR33 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR33 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D33`"]
pub type D33_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D33`"]
pub struct D33_W<'a> {
    w: &'a mut W,
}
impl<'a> D33_W<'a> {
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
    pub fn d33(&self) -> D33_R {
        D33_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d33(&mut self) -> D33_W {
        D33_W { w: self }
    }
}
