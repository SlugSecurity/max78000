#[doc = "Register `VREGO_D` reader"]
pub type R = crate::R<VREGO_D_SPEC>;
#[doc = "Register `VREGO_D` writer"]
pub type W = crate::W<VREGO_D_SPEC>;
#[doc = "Field `VSETD` reader - Regulator Output Voltage Setting"]
pub type VSETD_R = crate::FieldReader;
#[doc = "Field `VSETD` writer - Regulator Output Voltage Setting"]
pub type VSETD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `RANGED` reader - Regulator Output Range Set"]
pub type RANGED_R = crate::BitReader<RANGED_A>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RANGED_A {
    #[doc = "0: Low output voltage range"]
    LOW = 0,
    #[doc = "1: High output voltage range"]
    HIGH = 1,
}
impl From<RANGED_A> for bool {
    #[inline(always)]
    fn from(variant: RANGED_A) -> Self {
        variant as u8 != 0
    }
}
impl RANGED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGED_A {
        match self.bits {
            false => RANGED_A::LOW,
            true => RANGED_A::HIGH,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RANGED_A::LOW
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RANGED_A::HIGH
    }
}
#[doc = "Field `RANGED` writer - Regulator Output Range Set"]
pub type RANGED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RANGED_A>;
impl<'a, REG, const O: u8> RANGED_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(RANGED_A::LOW)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(RANGED_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetd(&self) -> VSETD_R {
        VSETD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn ranged(&self) -> RANGED_R {
        RANGED_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    #[must_use]
    pub fn vsetd(&mut self) -> VSETD_W<VREGO_D_SPEC, 0> {
        VSETD_W::new(self)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    #[must_use]
    pub fn ranged(&mut self) -> RANGED_W<VREGO_D_SPEC, 7> {
        RANGED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Buck Voltage Regulator D Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vrego_d::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vrego_d::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREGO_D_SPEC;
impl crate::RegisterSpec for VREGO_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_d::R`](R) reader structure"]
impl crate::Readable for VREGO_D_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vrego_d::W`](W) writer structure"]
impl crate::Writable for VREGO_D_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGO_D to value 0"]
impl crate::Resettable for VREGO_D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
