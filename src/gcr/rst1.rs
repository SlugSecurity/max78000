#[doc = "Register `RST1` reader"]
pub type R = crate::R<RST1_SPEC>;
#[doc = "Register `RST1` writer"]
pub type W = crate::W<RST1_SPEC>;
#[doc = "Field `I2C1` reader - I2C1 Reset."]
pub type I2C1_R = crate::BitReader<RESET_READ_A>;
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `I2C1` writer - I2C1 Reset."]
pub type I2C1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> I2C1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `PT` reader - PT Reset."]
pub type PT_R = crate::BitReader<RESET_READ_A>;
#[doc = "PT Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl PT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `PT` writer - PT Reset."]
pub type PT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> PT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `OWM` reader - OWM Reset."]
pub type OWM_R = crate::BitReader<RESET_READ_A>;
#[doc = "OWM Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl OWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `OWM` writer - OWM Reset."]
pub type OWM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> OWM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `CRC` reader - CRC Reset."]
pub type CRC_R = crate::BitReader<RESET_READ_A>;
#[doc = "CRC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `CRC` writer - CRC Reset."]
pub type CRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> CRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `AES` reader - AES Reset."]
pub type AES_R = crate::BitReader<RESET_READ_A>;
#[doc = "AES Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl AES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `AES` writer - AES Reset."]
pub type AES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> AES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `SPI0` reader - SPI 0 Reset."]
pub type SPI0_R = crate::BitReader<RESET_READ_A>;
#[doc = "SPI 0 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `SPI0` writer - SPI 0 Reset."]
pub type SPI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> SPI0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `SMPHR` reader - SMPHR Reset."]
pub type SMPHR_R = crate::BitReader<RESET_READ_A>;
#[doc = "SMPHR Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPHR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `SMPHR` writer - SMPHR Reset."]
pub type SMPHR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> SMPHR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `I2S` reader - I2S Reset."]
pub type I2S_R = crate::BitReader<RESET_READ_A>;
#[doc = "I2S Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `I2S` writer - I2S Reset."]
pub type I2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> I2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `I2C2` reader - I2C2 Reset."]
pub type I2C2_R = crate::BitReader<RESET_READ_A>;
#[doc = "I2C2 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `I2C2` writer - I2C2 Reset."]
pub type I2C2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> I2C2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `DVS` reader - DVS Reset."]
pub type DVS_R = crate::BitReader<RESET_READ_A>;
#[doc = "DVS Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl DVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `DVS` writer - DVS Reset."]
pub type DVS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> DVS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `SIMO` reader - SIMO Reset."]
pub type SIMO_R = crate::BitReader<RESET_READ_A>;
#[doc = "SIMO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl SIMO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `SIMO` writer - SIMO Reset."]
pub type SIMO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> SIMO_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
#[doc = "Field `CPU1` reader - CPU1 Reset."]
pub type CPU1_R = crate::BitReader<RESET_READ_A>;
#[doc = "CPU1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_READ_A {
    #[doc = "0: Reset complete."]
    RESET_DONE = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    BUSY = 1,
}
impl From<RESET_READ_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_READ_A {
        match self.bits {
            false => RESET_READ_A::RESET_DONE,
            true => RESET_READ_A::BUSY,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == RESET_READ_A::RESET_DONE
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RESET_READ_A::BUSY
    }
}
#[doc = "Field `CPU1` writer - CPU1 Reset."]
pub type CPU1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RESET_READ_A>;
impl<'a, REG, const O: u8> CPU1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::RESET_DONE)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_READ_A::BUSY)
    }
}
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SMPHR_R {
        SMPHR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    pub fn simo(&self) -> SIMO_R {
        SIMO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<RST1_SPEC, 0> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<RST1_SPEC, 1> {
        PT_W::new(self)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    #[must_use]
    pub fn owm(&mut self) -> OWM_W<RST1_SPEC, 7> {
        OWM_W::new(self)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<RST1_SPEC, 9> {
        CRC_W::new(self)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<RST1_SPEC, 10> {
        AES_W::new(self)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<RST1_SPEC, 11> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    #[must_use]
    pub fn smphr(&mut self) -> SMPHR_W<RST1_SPEC, 16> {
        SMPHR_W::new(self)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<RST1_SPEC, 19> {
        I2S_W::new(self)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<RST1_SPEC, 20> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dvs(&mut self) -> DVS_W<RST1_SPEC, 24> {
        DVS_W::new(self)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn simo(&mut self) -> SIMO_W<RST1_SPEC, 25> {
        SIMO_W::new(self)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    #[must_use]
    pub fn cpu1(&mut self) -> CPU1_W<RST1_SPEC, 31> {
        CPU1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reset 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST1_SPEC;
impl crate::RegisterSpec for RST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for RST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for RST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST1 to value 0"]
impl crate::Resettable for RST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
