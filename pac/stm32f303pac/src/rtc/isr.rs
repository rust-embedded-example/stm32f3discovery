#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type AlrawfR = crate::BitReader;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub type AlrbwfR = crate::BitReader;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WutwfR = crate::BitReader;
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type ShpfR = crate::BitReader;
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type ShpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type InitsR = crate::BitReader;
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITF` reader - Initialization flag"]
pub type InitfR = crate::BitReader;
#[doc = "Field `INIT` reader - Initialization mode"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization mode"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type AlrafR = crate::BitReader;
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type AlrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type AlrbfR = crate::BitReader;
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type AlrbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WutfR = crate::BitReader;
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TsovfR = crate::BitReader;
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TsovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub type Tamp1fR = crate::BitReader;
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub type Tamp1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2F` reader - RTC_TAMP2 detection flag"]
pub type Tamp2fR = crate::BitReader;
#[doc = "Field `TAMP2F` writer - RTC_TAMP2 detection flag"]
pub type Tamp2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3F` reader - RTC_TAMP3 detection flag"]
pub type Tamp3fR = crate::BitReader;
#[doc = "Field `TAMP3F` writer - RTC_TAMP3 detection flag"]
pub type Tamp3fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RecalpfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> AlrawfR {
        AlrawfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> AlrbwfR {
        AlrbwfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WutwfR {
        WutwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> ShpfR {
        ShpfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> InitsR {
        InitsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> InitfR {
        InitfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> AlrafR {
        AlrafR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> AlrbfR {
        AlrbfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WutfR {
        WutfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TsovfR {
        TsovfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> Tamp1fR {
        Tamp1fR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> Tamp2fR {
        Tamp2fR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    pub fn tamp3f(&self) -> Tamp3fR {
        Tamp3fR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RecalpfR {
        RecalpfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&mut self) -> ShpfW<'_, IsrSpec> {
        ShpfW::new(self, 3)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RsfW<'_, IsrSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, IsrSpec> {
        InitW::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&mut self) -> AlrafW<'_, IsrSpec> {
        AlrafW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&mut self) -> AlrbfW<'_, IsrSpec> {
        AlrbfW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&mut self) -> WutfW<'_, IsrSpec> {
        WutfW::new(self, 10)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TsfW<'_, IsrSpec> {
        TsfW::new(self, 11)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&mut self) -> TsovfW<'_, IsrSpec> {
        TsovfW::new(self, 12)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&mut self) -> Tamp1fW<'_, IsrSpec> {
        Tamp1fW::new(self, 13)
    }
    #[doc = "Bit 14 - RTC_TAMP2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&mut self) -> Tamp2fW<'_, IsrSpec> {
        Tamp2fW::new(self, 14)
    }
    #[doc = "Bit 15 - RTC_TAMP3 detection flag"]
    #[inline(always)]
    pub fn tamp3f(&mut self) -> Tamp3fW<'_, IsrSpec> {
        Tamp3fW::new(self, 15)
    }
}
#[doc = "initialization and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0x07"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0x07;
}
