#[doc = "Register `INTFL` reader"]
pub type R = crate::R<INTFL_SPEC>;
#[doc = "Field `GPIO_INTFL` reader - Mask of all of the pins on the port."]
pub type GPIO_INTFL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intfl(&self) -> GPIO_INTFL_R {
        GPIO_INTFL_R::new(self.bits)
    }
}
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for INTFL_SPEC {}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
