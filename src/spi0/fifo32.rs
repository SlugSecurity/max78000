#[doc = "Register `FIFO32` reader"]
pub type R = crate::R<FIFO32_SPEC>;
#[doc = "Register `FIFO32` writer"]
pub type W = crate::W<FIFO32_SPEC>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFO32_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO32_SPEC;
impl crate::RegisterSpec for FIFO32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo32::R`](R) reader structure"]
impl crate::Readable for FIFO32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo32::W`](W) writer structure"]
impl crate::Writable for FIFO32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO32 to value 0"]
impl crate::Resettable for FIFO32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
