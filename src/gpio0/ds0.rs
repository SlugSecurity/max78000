#[doc = "Register `DS0` reader"]
pub type R = crate::R<DS0_SPEC>;
#[doc = "Register `DS0` writer"]
pub type W = crate::W<DS0_SPEC>;
#[doc = "Field `GPIO_DS0` reader - Mask of all of the pins on the port."]
pub type GPIO_DS0_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_DS0` writer - Mask of all of the pins on the port."]
pub type GPIO_DS0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds0(&self) -> GPIO_DS0_R {
        GPIO_DS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_ds0(&mut self) -> GPIO_DS0_W<DS0_SPEC> {
        GPIO_DS0_W::new(self, 0)
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
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DS0_SPEC;
impl crate::RegisterSpec for DS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds0::R`](R) reader structure"]
impl crate::Readable for DS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ds0::W`](W) writer structure"]
impl crate::Writable for DS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DS0 to value 0"]
impl crate::Resettable for DS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
