#[doc = "Register `HSCLK` reader"]
pub type R = crate::R<HSCLK_SPEC>;
#[doc = "Register `HSCLK` writer"]
pub type W = crate::W<HSCLK_SPEC>;
#[doc = "Field `LO` reader - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LO_R = crate::FieldReader;
#[doc = "Field `LO` writer - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HI` reader - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HI_R = crate::FieldReader;
#[doc = "Field `HI` writer - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<HSCLK_SPEC, 0> {
        LO_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<HSCLK_SPEC, 8> {
        HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock high Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCLK_SPEC;
impl crate::RegisterSpec for HSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsclk::R`](R) reader structure"]
impl crate::Readable for HSCLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsclk::W`](W) writer structure"]
impl crate::Writable for HSCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCLK to value 0"]
impl crate::Resettable for HSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
