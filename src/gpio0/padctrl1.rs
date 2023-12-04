#[doc = "Register `PADCTRL1` reader"]
pub type R = crate::R<PADCTRL1_SPEC>;
#[doc = "Register `PADCTRL1` writer"]
pub type W = crate::W<PADCTRL1_SPEC>;
#[doc = "Field `GPIO_PADCTRL1` reader - The two bits in GPIO_PADCTRL0 and GPIO_PADCTRL1 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_PADCTRL1` writer - The two bits in GPIO_PADCTRL0 and GPIO_PADCTRL1 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GPIO_PADCTRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PADCTRL0 and GPIO_PADCTRL1 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl1(&self) -> GPIO_PADCTRL1_R {
        GPIO_PADCTRL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PADCTRL0 and GPIO_PADCTRL1 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_padctrl1(&mut self) -> GPIO_PADCTRL1_W<PADCTRL1_SPEC> {
        GPIO_PADCTRL1_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCTRL1_SPEC;
impl crate::RegisterSpec for PADCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padctrl1::R`](R) reader structure"]
impl crate::Readable for PADCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padctrl1::W`](W) writer structure"]
impl crate::Writable for PADCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADCTRL1 to value 0"]
impl crate::Resettable for PADCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
