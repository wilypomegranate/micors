#[doc = "Reader of register DR2"]
pub type R = crate::R<u32, super::DR2>;
#[doc = "Writer for register DR2"]
pub type W = crate::W<u32, super::DR2>;
#[doc = "Register DR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D2`"]
pub type D2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D2`"]
pub struct D2_W<'a> {
    w: &'a mut W,
}
impl<'a> D2_W<'a> {
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
    pub fn d2(&self) -> D2_R {
        D2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d2(&mut self) -> D2_W {
        D2_W { w: self }
    }
}
