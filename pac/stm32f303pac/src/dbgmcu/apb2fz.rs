#[doc = "Register `APB2FZ` reader"]
pub type R = crate::R<Apb2fzSpec>;
#[doc = "Register `APB2FZ` writer"]
pub type W = crate::W<Apb2fzSpec>;
#[doc = "Field `DBG_TIM15_STOP` reader - Debug Timer 15 stopped when Core is halted"]
pub type DbgTim15StopR = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - Debug Timer 15 stopped when Core is halted"]
pub type DbgTim15StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - Debug Timer 16 stopped when Core is halted"]
pub type DbgTim16StopR = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - Debug Timer 16 stopped when Core is halted"]
pub type DbgTim16StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STO` reader - Debug Timer 17 stopped when Core is halted"]
pub type DbgTim17StoR = crate::BitReader;
#[doc = "Field `DBG_TIM17_STO` writer - Debug Timer 17 stopped when Core is halted"]
pub type DbgTim17StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM19_STOP` reader - Debug Timer 19 stopped when Core is halted"]
pub type DbgTim19StopR = crate::BitReader;
#[doc = "Field `DBG_TIM19_STOP` writer - Debug Timer 19 stopped when Core is halted"]
pub type DbgTim19StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DbgTim15StopR {
        DbgTim15StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DbgTim16StopR {
        DbgTim16StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&self) -> DbgTim17StoR {
        DbgTim17StoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DbgTim19StopR {
        DbgTim19StopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Debug Timer 15 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DbgTim15StopW<'_, Apb2fzSpec> {
        DbgTim15StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Timer 16 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DbgTim16StopW<'_, Apb2fzSpec> {
        DbgTim16StopW::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 17 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_sto(&mut self) -> DbgTim17StoW<'_, Apb2fzSpec> {
        DbgTim17StoW::new(self, 4)
    }
    #[doc = "Bit 5 - Debug Timer 19 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim19_stop(&mut self) -> DbgTim19StopW<'_, Apb2fzSpec> {
        DbgTim19StopW::new(self, 5)
    }
}
#[doc = "APB High Freeze Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2fzSpec;
impl crate::RegisterSpec for Apb2fzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2fz::R`](R) reader structure"]
impl crate::Readable for Apb2fzSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2fz::W`](W) writer structure"]
impl crate::Writable for Apb2fzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2FZ to value 0"]
impl crate::Resettable for Apb2fzSpec {}
