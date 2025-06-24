#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PG` reader - Programming"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page erase"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Page erase"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Mass erase"]
pub type MerR = crate::BitReader;
#[doc = "Field `MER` writer - Mass erase"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTPG` reader - Option byte programming"]
pub type OptpgR = crate::BitReader;
#[doc = "Field `OPTPG` writer - Option byte programming"]
pub type OptpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTER` reader - Option byte erase"]
pub type OpterR = crate::BitReader;
#[doc = "Field `OPTER` writer - Option byte erase"]
pub type OpterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - Start"]
pub type StrtR = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type StrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTWRE` reader - Option bytes write enable"]
pub type OptwreR = crate::BitReader;
#[doc = "Field `OPTWRE` writer - Option bytes write enable"]
pub type OptwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EopieR = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_OPTLOAD` reader - Force option byte loading"]
pub type ForceOptloadR = crate::BitReader;
#[doc = "Field `FORCE_OPTLOAD` writer - Force option byte loading"]
pub type ForceOptloadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn optpg(&self) -> OptpgR {
        OptpgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn opter(&self) -> OpterR {
        OpterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> StrtR {
        StrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn optwre(&self) -> OptwreR {
        OptwreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EopieR {
        EopieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force option byte loading"]
    #[inline(always)]
    pub fn force_optload(&self) -> ForceOptloadR {
        ForceOptloadR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PgW<CrSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<CrSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&mut self) -> MerW<CrSpec> {
        MerW::new(self, 2)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn optpg(&mut self) -> OptpgW<CrSpec> {
        OptpgW::new(self, 4)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn opter(&mut self) -> OpterW<CrSpec> {
        OpterW::new(self, 5)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&mut self) -> StrtW<CrSpec> {
        StrtW::new(self, 6)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<CrSpec> {
        LockW::new(self, 7)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn optwre(&mut self) -> OptwreW<CrSpec> {
        OptwreW::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<CrSpec> {
        ErrieW::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EopieW<CrSpec> {
        EopieW::new(self, 12)
    }
    #[doc = "Bit 13 - Force option byte loading"]
    #[inline(always)]
    pub fn force_optload(&mut self) -> ForceOptloadW<CrSpec> {
        ForceOptloadW::new(self, 13)
    }
}
#[doc = "Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x80"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x80;
}
