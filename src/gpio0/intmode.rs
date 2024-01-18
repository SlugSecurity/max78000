#[doc = "Register `INTMODE` reader"]
pub type R = crate::R<INTMODE_SPEC>;
#[doc = "Register `INTMODE` writer"]
pub type W = crate::W<INTMODE_SPEC>;
#[doc = "Field `GPIO_INTMODE` reader - Mask of all of the pins on the port."]
pub type GPIO_INTMODE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INTMODE` writer - Mask of all of the pins on the port."]
pub type GPIO_INTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intmode(&self) -> GPIO_INTMODE_R {
        GPIO_INTMODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_intmode(&mut self) -> GPIO_INTMODE_W<INTMODE_SPEC> {
        GPIO_INTMODE_W::new(self, 0)
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
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMODE_SPEC;
impl crate::RegisterSpec for INTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmode::R`](R) reader structure"]
impl crate::Readable for INTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmode::W`](W) writer structure"]
impl crate::Writable for INTMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMODE to value 0"]
impl crate::Resettable for INTMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
