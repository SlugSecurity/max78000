#[doc = "Register `EN0` reader"]
pub type R = crate::R<EN0_SPEC>;
#[doc = "Register `EN0` writer"]
pub type W = crate::W<EN0_SPEC>;
#[doc = "Field `GPIO_EN` reader - Mask of all of the pins on the port."]
pub type GPIO_EN_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_EN` writer - Mask of all of the pins on the port."]
pub type GPIO_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_en(&mut self) -> GPIO_EN_W<EN0_SPEC> {
        GPIO_EN_W::new(self, 0)
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
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN0_SPEC;
impl crate::RegisterSpec for EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0::R`](R) reader structure"]
impl crate::Readable for EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en0::W`](W) writer structure"]
impl crate::Writable for EN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for EN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
