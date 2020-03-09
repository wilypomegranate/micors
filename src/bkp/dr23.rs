#[doc = "Reader of register DR23"]
pub type R = crate::R<u32, super::DR23>;
#[doc = "Writer for register DR23"]
pub type W = crate::W<u32, super::DR23>;
#[doc = "Register DR23 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D23`"]
pub type D23_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D23`"]
pub struct D23_W<'a> {
    w: &'a mut W,
}
impl<'a> D23_W<'a> {
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
    pub fn d23(&self) -> D23_R {
        D23_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d23(&mut self) -> D23_W {
        D23_W { w: self }
    }
}
