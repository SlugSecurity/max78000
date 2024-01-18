#[doc = "Register `RST` writer"]
pub type W = crate::W<RST_SPEC>;
#[doc = "Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RESET_AW {
    #[doc = "165: The first value to be written to reset the WDT."]
    SEQ0 = 165,
    #[doc = "90: The second value to be written to reset the WDT."]
    SEQ1 = 90,
    #[doc = "254: WDT Enable: first rst value."]
    SEQ2 = 254,
    #[doc = "237: WDT Enable: second rst value."]
    SEQ3 = 237,
    #[doc = "222: WDT Disable: first rst value."]
    SEQ4 = 222,
    #[doc = "173: WDT Disable: second rst value."]
    SEQ5 = 173,
}
impl From<RESET_AW> for u8 {
    #[inline(always)]
    fn from(variant: RESET_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RESET_AW {
    type Ux = u8;
}
#[doc = "Field `RESET` writer - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
pub type RESET_W<'a, REG> = crate::FieldWriter<'a, REG, 8, RESET_AW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The first value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq0(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ0)
    }
    #[doc = "The second value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq1(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ1)
    }
    #[doc = "WDT Enable: first rst value."]
    #[inline(always)]
    pub fn seq2(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ2)
    }
    #[doc = "WDT Enable: second rst value."]
    #[inline(always)]
    pub fn seq3(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ3)
    }
    #[doc = "WDT Disable: first rst value."]
    #[inline(always)]
    pub fn seq4(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ4)
    }
    #[doc = "WDT Disable: second rst value."]
    #[inline(always)]
    pub fn seq5(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_AW::SEQ5)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<RST_SPEC> {
        RESET_W::new(self, 0)
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
#[doc = "Windowed Watchdog Timer Reset Register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
