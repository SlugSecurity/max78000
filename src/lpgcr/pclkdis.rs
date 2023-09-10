#[doc = "Register `PCLKDIS` reader"]
pub type R = crate::R<PCLKDIS_SPEC>;
#[doc = "Register `PCLKDIS` writer"]
pub type W = crate::W<PCLKDIS_SPEC>;
#[doc = "Field `GPIO2` reader - Low Power GPIO 2 Clock Disable."]
pub type GPIO2_R = crate::BitReader<GPIO2_A>;
#[doc = "Low Power GPIO 2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<GPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_A {
        match self.bits {
            false => GPIO2_A::EN,
            true => GPIO2_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GPIO2_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO2_A::DIS
    }
}
#[doc = "Field `GPIO2` writer - Low Power GPIO 2 Clock Disable."]
pub type GPIO2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, GPIO2_A>;
impl<'a, REG, const O: u8> GPIO2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO2_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GPIO2_A::DIS)
    }
}
#[doc = "Field `WDT1` reader - Low Power Watchdog 1 Clock Disable."]
pub type WDT1_R = crate::BitReader<WDT1_A>;
#[doc = "Low Power Watchdog 1 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT1_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<WDT1_A> for bool {
    #[inline(always)]
    fn from(variant: WDT1_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT1_A {
        match self.bits {
            false => WDT1_A::EN,
            true => WDT1_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WDT1_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WDT1_A::DIS
    }
}
#[doc = "Field `WDT1` writer - Low Power Watchdog 1 Clock Disable."]
pub type WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDT1_A>;
impl<'a, REG, const O: u8> WDT1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WDT1_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WDT1_A::DIS)
    }
}
#[doc = "Field `TMR4` reader - Low Power Timer 4 Clock Disable."]
pub type TMR4_R = crate::BitReader<TMR4_A>;
#[doc = "Low Power Timer 4 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR4_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR4_A> for bool {
    #[inline(always)]
    fn from(variant: TMR4_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR4_A {
        match self.bits {
            false => TMR4_A::EN,
            true => TMR4_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR4_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR4_A::DIS
    }
}
#[doc = "Field `TMR4` writer - Low Power Timer 4 Clock Disable."]
pub type TMR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR4_A>;
impl<'a, REG, const O: u8> TMR4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TMR4_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TMR4_A::DIS)
    }
}
#[doc = "Field `TMR5` reader - Low Power Timer 5 Clock Disable."]
pub type TMR5_R = crate::BitReader<TMR5_A>;
#[doc = "Low Power Timer 5 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR5_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<TMR5_A> for bool {
    #[inline(always)]
    fn from(variant: TMR5_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR5_A {
        match self.bits {
            false => TMR5_A::EN,
            true => TMR5_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMR5_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMR5_A::DIS
    }
}
#[doc = "Field `TMR5` writer - Low Power Timer 5 Clock Disable."]
pub type TMR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR5_A>;
impl<'a, REG, const O: u8> TMR5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5_A::DIS)
    }
}
#[doc = "Field `UART3` reader - Low Power UART 3 Clock Disable."]
pub type UART3_R = crate::BitReader<UART3_A>;
#[doc = "Low Power UART 3 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART3_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<UART3_A> for bool {
    #[inline(always)]
    fn from(variant: UART3_A) -> Self {
        variant as u8 != 0
    }
}
impl UART3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3_A {
        match self.bits {
            false => UART3_A::EN,
            true => UART3_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART3_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART3_A::DIS
    }
}
#[doc = "Field `UART3` writer - Low Power UART 3 Clock Disable."]
pub type UART3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, UART3_A>;
impl<'a, REG, const O: u8> UART3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(UART3_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(UART3_A::DIS)
    }
}
#[doc = "Field `LPCOMP` reader - Low Power Comparator Clock Disable."]
pub type LPCOMP_R = crate::BitReader<LPCOMP_A>;
#[doc = "Low Power Comparator Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCOMP_A {
    #[doc = "0: enable it."]
    EN = 0,
    #[doc = "1: disable it."]
    DIS = 1,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::EN,
            true => LPCOMP_A::DIS,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LPCOMP_A::EN
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LPCOMP_A::DIS
    }
}
#[doc = "Field `LPCOMP` writer - Low Power Comparator Clock Disable."]
pub type LPCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LPCOMP_A>;
impl<'a, REG, const O: u8> LPCOMP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(LPCOMP_A::EN)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(LPCOMP_A::DIS)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power GPIO 2 Clock Disable."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog 1 Clock Disable."]
    #[inline(always)]
    pub fn wdt1(&self) -> WDT1_R {
        WDT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Clock Disable."]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Clock Disable."]
    #[inline(always)]
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power UART 3 Clock Disable."]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Power Comparator Clock Disable."]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power GPIO 2 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO2_W<PCLKDIS_SPEC, 0> {
        GPIO2_W::new(self)
    }
    #[doc = "Bit 1 - Low Power Watchdog 1 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt1(&mut self) -> WDT1_W<PCLKDIS_SPEC, 1> {
        WDT1_W::new(self)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr4(&mut self) -> TMR4_W<PCLKDIS_SPEC, 2> {
        TMR4_W::new(self)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tmr5(&mut self) -> TMR5_W<PCLKDIS_SPEC, 3> {
        TMR5_W::new(self)
    }
    #[doc = "Bit 4 - Low Power UART 3 Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<PCLKDIS_SPEC, 4> {
        UART3_W::new(self)
    }
    #[doc = "Bit 6 - Low Power Comparator Clock Disable."]
    #[inline(always)]
    #[must_use]
    pub fn lpcomp(&mut self) -> LPCOMP_W<PCLKDIS_SPEC, 6> {
        LPCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low Power Peripheral Clock Disable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKDIS_SPEC;
impl crate::RegisterSpec for PCLKDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis::R`](R) reader structure"]
impl crate::Readable for PCLKDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclkdis::W`](W) writer structure"]
impl crate::Writable for PCLKDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKDIS to value 0"]
impl crate::Resettable for PCLKDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
