#[doc = "Reader of register DR15"]
pub type R = crate::R<u32, super::DR15>;
#[doc = "Writer for register DR15"]
pub type W = crate::W<u32, super::DR15>;
#[doc = "Register DR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D15`"]
pub type D15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D15`"]
pub struct D15_W<'a> {
    w: &'a mut W,
}
impl<'a> D15_W<'a> {
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
    pub fn d15(&self) -> D15_R {
        D15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d15(&mut self) -> D15_W {
        D15_W { w: self }
    }
}
