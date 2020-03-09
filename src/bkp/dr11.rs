#[doc = "Reader of register DR11"]
pub type R = crate::R<u32, super::DR11>;
#[doc = "Writer for register DR11"]
pub type W = crate::W<u32, super::DR11>;
#[doc = "Register DR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR11`"]
pub type DR11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR11`"]
pub struct DR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DR11_W<'a> {
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
    pub fn dr11(&self) -> DR11_R {
        DR11_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn dr11(&mut self) -> DR11_W {
        DR11_W { w: self }
    }
}
