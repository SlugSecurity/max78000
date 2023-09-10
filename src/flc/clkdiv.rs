#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<CLKDIV_SPEC>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<CLKDIV_SPEC>;
#[doc = "Field `CLKDIV` reader - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKDIV_SPEC, 0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_SPEC;
impl crate::RegisterSpec for CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for CLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0x64"]
impl crate::Resettable for CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x64;
}
