#[doc = "Register `BTR` reader"]
pub type R = crate::R<BtrSpec>;
#[doc = "Register `BTR` writer"]
pub type W = crate::W<BtrSpec>;
#[doc = "Field `BRP` reader - BRP"]
pub type BrpR = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TS1` reader - TS1"]
pub type Ts1R = crate::FieldReader;
#[doc = "Field `TS1` writer - TS1"]
pub type Ts1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS2` reader - TS2"]
pub type Ts2R = crate::FieldReader;
#[doc = "Field `TS2` writer - TS2"]
pub type Ts2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - SJW"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - SJW"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBKM` reader - LBKM"]
pub type LbkmR = crate::BitReader;
#[doc = "Field `LBKM` writer - LBKM"]
pub type LbkmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SILM` reader - SILM"]
pub type SilmR = crate::BitReader;
#[doc = "Field `SILM` writer - SILM"]
pub type SilmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> Ts1R {
        Ts1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> Ts2R {
        Ts2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LbkmR {
        LbkmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SilmR {
        SilmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, BtrSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&mut self) -> Ts1W<'_, BtrSpec> {
        Ts1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&mut self) -> Ts2W<'_, BtrSpec> {
        Ts2W::new(self, 20)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, BtrSpec> {
        SjwW::new(self, 24)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&mut self) -> LbkmW<'_, BtrSpec> {
        LbkmW::new(self, 30)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&mut self) -> SilmW<'_, BtrSpec> {
        SilmW::new(self, 31)
    }
}
#[doc = "bit timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`btr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtrSpec;
impl crate::RegisterSpec for BtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr::R`](R) reader structure"]
impl crate::Readable for BtrSpec {}
#[doc = "`write(|w| ..)` method takes [`btr::W`](W) writer structure"]
impl crate::Writable for BtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTR to value 0x0123_0000"]
impl crate::Resettable for BtrSpec {
    const RESET_VALUE: u32 = 0x0123_0000;
}
