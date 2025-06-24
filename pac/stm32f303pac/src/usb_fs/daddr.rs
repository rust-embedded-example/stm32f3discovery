#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DaddrSpec>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DaddrSpec>;
#[doc = "Field `ADD` reader - Device address"]
pub type AddR = crate::BitReader;
#[doc = "Field `ADD` writer - Device address"]
pub type AddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1` reader - Device address"]
pub type Add1R = crate::BitReader;
#[doc = "Field `ADD1` writer - Device address"]
pub type Add1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD2` reader - Device address"]
pub type Add2R = crate::BitReader;
#[doc = "Field `ADD2` writer - Device address"]
pub type Add2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD3` reader - Device address"]
pub type Add3R = crate::BitReader;
#[doc = "Field `ADD3` writer - Device address"]
pub type Add3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD4` reader - Device address"]
pub type Add4R = crate::BitReader;
#[doc = "Field `ADD4` writer - Device address"]
pub type Add4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD5` reader - Device address"]
pub type Add5R = crate::BitReader;
#[doc = "Field `ADD5` writer - Device address"]
pub type Add5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD6` reader - Device address"]
pub type Add6R = crate::BitReader;
#[doc = "Field `ADD6` writer - Device address"]
pub type Add6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EF` reader - Enable function"]
pub type EfR = crate::BitReader;
#[doc = "Field `EF` writer - Enable function"]
pub type EfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&self) -> Add1R {
        Add1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&self) -> Add2R {
        Add2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&self) -> Add3R {
        Add3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&self) -> Add4R {
        Add4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&self) -> Add5R {
        Add5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&self) -> Add6R {
        Add6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&self) -> EfR {
        EfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<'_, DaddrSpec> {
        AddW::new(self, 0)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&mut self) -> Add1W<'_, DaddrSpec> {
        Add1W::new(self, 1)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&mut self) -> Add2W<'_, DaddrSpec> {
        Add2W::new(self, 2)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&mut self) -> Add3W<'_, DaddrSpec> {
        Add3W::new(self, 3)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&mut self) -> Add4W<'_, DaddrSpec> {
        Add4W::new(self, 4)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&mut self) -> Add5W<'_, DaddrSpec> {
        Add5W::new(self, 5)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&mut self) -> Add6W<'_, DaddrSpec> {
        Add6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&mut self) -> EfW<'_, DaddrSpec> {
        EfW::new(self, 7)
    }
}
#[doc = "device address\n\nYou can [`read`](crate::Reg::read) this register and get [`daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaddrSpec;
impl crate::RegisterSpec for DaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DaddrSpec {}
