#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` writer - Port x Reset bit y"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x Reset bit y"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR2` writer - Port x Reset bit y"]
pub type Br2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR3` writer - Port x Reset bit y"]
pub type Br3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR4` writer - Port x Reset bit y"]
pub type Br4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR5` writer - Port x Reset bit y"]
pub type Br5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR6` writer - Port x Reset bit y"]
pub type Br6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR7` writer - Port x Reset bit y"]
pub type Br7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR8` writer - Port x Reset bit y"]
pub type Br8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR9` writer - Port x Reset bit y"]
pub type Br9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR10` writer - Port x Reset bit y"]
pub type Br10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR11` writer - Port x Reset bit y"]
pub type Br11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR12` writer - Port x Reset bit y"]
pub type Br12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR13` writer - Port x Reset bit y"]
pub type Br13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR14` writer - Port x Reset bit y"]
pub type Br14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR15` writer - Port x Reset bit y"]
pub type Br15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<'_, BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<'_, BrrSpec> {
        Br1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br2(&mut self) -> Br2W<'_, BrrSpec> {
        Br2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br3(&mut self) -> Br3W<'_, BrrSpec> {
        Br3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br4(&mut self) -> Br4W<'_, BrrSpec> {
        Br4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br5(&mut self) -> Br5W<'_, BrrSpec> {
        Br5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br6(&mut self) -> Br6W<'_, BrrSpec> {
        Br6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br7(&mut self) -> Br7W<'_, BrrSpec> {
        Br7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br8(&mut self) -> Br8W<'_, BrrSpec> {
        Br8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br9(&mut self) -> Br9W<'_, BrrSpec> {
        Br9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br10(&mut self) -> Br10W<'_, BrrSpec> {
        Br10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br11(&mut self) -> Br11W<'_, BrrSpec> {
        Br11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br12(&mut self) -> Br12W<'_, BrrSpec> {
        Br12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br13(&mut self) -> Br13W<'_, BrrSpec> {
        Br13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br14(&mut self) -> Br14W<'_, BrrSpec> {
        Br14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x Reset bit y"]
    #[inline(always)]
    pub fn br15(&mut self) -> Br15W<'_, BrrSpec> {
        Br15W::new(self, 15)
    }
}
#[doc = "Port bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
