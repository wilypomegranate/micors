#[doc = "Reader of register DR20"]
pub type R = crate::R<u32, super::DR20>;
#[doc = "Writer for register DR20"]
pub type W = crate::W<u32, super::DR20>;
#[doc = "Register DR20 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D20`"]
pub type D20_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D20`"]
pub struct D20_W<'a> {
    w: &'a mut W,
}
impl<'a> D20_W<'a> {
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
    pub fn d20(&self) -> D20_R {
        D20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d20(&mut self) -> D20_W {
        D20_W { w: self }
    }
}
