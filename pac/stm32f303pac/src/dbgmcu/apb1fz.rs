#[doc = "Register `APB1FZ` reader"]
pub type R = crate::R<Apb1fzSpec>;
#[doc = "Register `APB1FZ` writer"]
pub type W = crate::W<Apb1fzSpec>;
#[doc = "Field `DBG_TIM2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DbgTim2StopR = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DbgTim2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - Debug Timer 4 stopped when Core is halted"]
pub type DbgTim4StopR = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - Debug Timer 4 stopped when Core is halted"]
pub type DbgTim4StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - Debug Timer 5 stopped when Core is halted"]
pub type DbgTim5StopR = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - Debug Timer 5 stopped when Core is halted"]
pub type DbgTim5StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DbgTim6StopR = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DbgTim6StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - Debug Timer 7 stopped when Core is halted"]
pub type DbgTim7StopR = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - Debug Timer 7 stopped when Core is halted"]
pub type DbgTim7StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM12_STOP` reader - Debug Timer 12 stopped when Core is halted"]
pub type DbgTim12StopR = crate::BitReader;
#[doc = "Field `DBG_TIM12_STOP` writer - Debug Timer 12 stopped when Core is halted"]
pub type DbgTim12StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM13_STOP` reader - Debug Timer 13 stopped when Core is halted"]
pub type DbgTim13StopR = crate::BitReader;
#[doc = "Field `DBG_TIM13_STOP` writer - Debug Timer 13 stopped when Core is halted"]
pub type DbgTim13StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER14_STOP` reader - Debug Timer 14 stopped when Core is halted"]
pub type DbgTimer14StopR = crate::BitReader;
#[doc = "Field `DBG_TIMER14_STOP` writer - Debug Timer 14 stopped when Core is halted"]
pub type DbgTimer14StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM18_STOP` reader - Debug Timer 18 stopped when Core is halted"]
pub type DbgTim18StopR = crate::BitReader;
#[doc = "Field `DBG_TIM18_STOP` writer - Debug Timer 18 stopped when Core is halted"]
pub type DbgTim18StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopR = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DbgRtcStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopR = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DbgWwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN_STOP` reader - Debug CAN stopped when core is halted"]
pub type DbgCanStopR = crate::BitReader;
#[doc = "Field `DBG_CAN_STOP` writer - Debug CAN stopped when core is halted"]
pub type DbgCanStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DbgTim2StopR {
        DbgTim2StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DbgTim4StopR {
        DbgTim4StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DbgTim5StopR {
        DbgTim5StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DbgTim6StopR {
        DbgTim6StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DbgTim7StopR {
        DbgTim7StopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DbgTim12StopR {
        DbgTim12StopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DbgTim13StopR {
        DbgTim13StopR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&self) -> DbgTimer14StopR {
        DbgTimer14StopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&self) -> DbgTim18StopR {
        DbgTim18StopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
        DbgRtcStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
        DbgWwdgStopR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2c1SmbusTimeoutR {
        I2c1SmbusTimeoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2c2SmbusTimeoutR {
        I2c2SmbusTimeoutR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DbgCanStopR {
        DbgCanStopR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DbgTim2StopW<'_, Apb1fzSpec> {
        DbgTim2StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<'_, Apb1fzSpec> {
        DbgTim3StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DbgTim4StopW<'_, Apb1fzSpec> {
        DbgTim4StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DbgTim5StopW<'_, Apb1fzSpec> {
        DbgTim5StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DbgTim6StopW<'_, Apb1fzSpec> {
        DbgTim6StopW::new(self, 4)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DbgTim7StopW<'_, Apb1fzSpec> {
        DbgTim7StopW::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DbgTim12StopW<'_, Apb1fzSpec> {
        DbgTim12StopW::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DbgTim13StopW<'_, Apb1fzSpec> {
        DbgTim13StopW::new(self, 7)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&mut self) -> DbgTimer14StopW<'_, Apb1fzSpec> {
        DbgTimer14StopW::new(self, 8)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&mut self) -> DbgTim18StopW<'_, Apb1fzSpec> {
        DbgTim18StopW::new(self, 9)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DbgRtcStopW<'_, Apb1fzSpec> {
        DbgRtcStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DbgWwdgStopW<'_, Apb1fzSpec> {
        DbgWwdgStopW::new(self, 11)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<'_, Apb1fzSpec> {
        DbgIwdgStopW::new(self, 12)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&mut self) -> I2c1SmbusTimeoutW<'_, Apb1fzSpec> {
        I2c1SmbusTimeoutW::new(self, 21)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&mut self) -> I2c2SmbusTimeoutW<'_, Apb1fzSpec> {
        I2c2SmbusTimeoutW::new(self, 22)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DbgCanStopW<'_, Apb1fzSpec> {
        DbgCanStopW::new(self, 25)
    }
}
#[doc = "APB Low Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1fzSpec;
impl crate::RegisterSpec for Apb1fzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1fz::R`](R) reader structure"]
impl crate::Readable for Apb1fzSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1fz::W`](W) writer structure"]
impl crate::Writable for Apb1fzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1FZ to value 0"]
impl crate::Resettable for Apb1fzSpec {}
