#[doc = "Register `EN2` reader"]
pub type R = crate::R<EN2_SPEC>;
#[doc = "Register `EN2` writer"]
pub type W = crate::W<EN2_SPEC>;
#[doc = "Field `GPIO_EN2` reader - Mask of all of the pins on the port."]
pub type GPIO_EN2_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_EN2` writer - Mask of all of the pins on the port."]
pub type GPIO_EN2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en2(&self) -> GPIO_EN2_R {
        GPIO_EN2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en2(&mut self) -> GPIO_EN2_W<EN2_SPEC> {
        GPIO_EN2_W::new(self, 0)
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
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN2_SPEC;
impl crate::RegisterSpec for EN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en2::R`](R) reader structure"]
impl crate::Readable for EN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en2::W`](W) writer structure"]
impl crate::Writable for EN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN2 to value 0"]
impl crate::Resettable for EN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
