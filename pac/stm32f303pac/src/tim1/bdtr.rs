#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BdtrSpec>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BdtrSpec>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DtgR = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LockR = crate::FieldReader;
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OssiR = crate::BitReader;
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OssrR = crate::BitReader;
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKE` reader - Break enable"]
pub type BkeR = crate::BitReader;
#[doc = "Field `BKE` writer - Break enable"]
pub type BkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Break polarity"]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Break polarity"]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AoeR = crate::BitReader;
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOE` reader - Main output enable"]
pub type MoeR = crate::BitReader;
#[doc = "Field `MOE` writer - Main output enable"]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKF` reader - Break filter"]
pub type BkfR = crate::FieldReader;
#[doc = "Field `BKF` writer - Break filter"]
pub type BkfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2F` reader - Break 2 filter"]
pub type Bk2fR = crate::FieldReader;
#[doc = "Field `BK2F` writer - Break 2 filter"]
pub type Bk2fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BK2E` reader - Break 2 enable"]
pub type Bk2eR = crate::BitReader;
#[doc = "Field `BK2E` writer - Break 2 enable"]
pub type Bk2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2P` reader - Break 2 polarity"]
pub type Bk2pR = crate::BitReader;
#[doc = "Field `BK2P` writer - Break 2 polarity"]
pub type Bk2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DtgR {
        DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OssiR {
        OssiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OssrR {
        OssrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BkeR {
        BkeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AoeR {
        AoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&self) -> BkfR {
        BkfR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    pub fn bk2f(&self) -> Bk2fR {
        Bk2fR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable"]
    #[inline(always)]
    pub fn bk2e(&self) -> Bk2eR {
        Bk2eR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    pub fn bk2p(&self) -> Bk2pR {
        Bk2pR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DtgW<'_, BdtrSpec> {
        DtgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, BdtrSpec> {
        LockW::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OssiW<'_, BdtrSpec> {
        OssiW::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OssrW<'_, BdtrSpec> {
        OssrW::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&mut self) -> BkeW<'_, BdtrSpec> {
        BkeW::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, BdtrSpec> {
        BkpW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AoeW<'_, BdtrSpec> {
        AoeW::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&mut self) -> MoeW<'_, BdtrSpec> {
        MoeW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter"]
    #[inline(always)]
    pub fn bkf(&mut self) -> BkfW<'_, BdtrSpec> {
        BkfW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter"]
    #[inline(always)]
    pub fn bk2f(&mut self) -> Bk2fW<'_, BdtrSpec> {
        Bk2fW::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 enable"]
    #[inline(always)]
    pub fn bk2e(&mut self) -> Bk2eW<'_, BdtrSpec> {
        Bk2eW::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity"]
    #[inline(always)]
    pub fn bk2p(&mut self) -> Bk2pW<'_, BdtrSpec> {
        Bk2pW::new(self, 25)
    }
}
#[doc = "break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdtrSpec;
impl crate::RegisterSpec for BdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BdtrSpec {}
