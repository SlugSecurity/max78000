#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `RDY` reader - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: TRNG Busy"]
    BUSY = 0,
    #[doc = "1: 32 bit random data is ready"]
    READY = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::BUSY,
            true => RDY_A::READY,
        }
    }
    #[doc = "TRNG Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RDY_A::BUSY
    }
    #[doc = "32 bit random data is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RDY_A::READY
    }
}
#[doc = "Field `RDY` writer - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
pub type RDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RDY_A>;
impl<'a, REG, const O: u8> RDY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRNG Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::BUSY)
    }
    #[doc = "32 bit random data is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(RDY_A::READY)
    }
}
impl R {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<STATUS_SPEC, 0> {
        RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
