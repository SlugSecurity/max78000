#[doc = "Register `FIFO8[%s]` reader"]
pub type R = crate::R<FIFO8_SPEC>;
#[doc = "Register `FIFO8[%s]` writer"]
pub type W = crate::W<FIFO8_SPEC>;
#[doc = "Field `DATA` reader - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Read to pull from RX FIFO, write to put into TX FIFO."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read to pull from RX FIFO, write to put into TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<FIFO8_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO8_SPEC;
impl crate::RegisterSpec for FIFO8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fifo8::R`](R) reader structure"]
impl crate::Readable for FIFO8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo8::W`](W) writer structure"]
impl crate::Writable for FIFO8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO8[%s]
to value 0"]
impl crate::Resettable for FIFO8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
