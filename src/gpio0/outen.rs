#[doc = "Register `OUTEN` reader"]
pub type R = crate::R<OUTEN_SPEC>;
#[doc = "Register `OUTEN` writer"]
pub type W = crate::W<OUTEN_SPEC>;
#[doc = "Field `EN` reader - Mask of all of the pins on the port."]
pub type EN_R = crate::FieldReader<u32>;
#[doc = "Field `EN` writer - Mask of all of the pins on the port."]
pub type EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<OUTEN_SPEC> {
        EN_W::new(self, 0)
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
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTEN_SPEC;
impl crate::RegisterSpec for OUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen::R`](R) reader structure"]
impl crate::Readable for OUTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outen::W`](W) writer structure"]
impl crate::Writable for OUTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OUTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
