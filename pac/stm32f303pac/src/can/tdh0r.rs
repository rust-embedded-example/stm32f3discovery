#[doc = "Register `TDH0R` reader"]
pub type R = crate::R<Tdh0rSpec>;
#[doc = "Register `TDH0R` writer"]
pub type W = crate::W<Tdh0rSpec>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - DATA4"]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - DATA5"]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - DATA6"]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - DATA7"]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - DATA7"]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<'_, Tdh0rSpec> {
        Data4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<'_, Tdh0rSpec> {
        Data5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<'_, Tdh0rSpec> {
        Data6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<'_, Tdh0rSpec> {
        Data7W::new(self, 24)
    }
}
#[doc = "mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdh0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdh0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdh0rSpec;
impl crate::RegisterSpec for Tdh0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdh0r::R`](R) reader structure"]
impl crate::Readable for Tdh0rSpec {}
#[doc = "`write(|w| ..)` method takes [`tdh0r::W`](W) writer structure"]
impl crate::Writable for Tdh0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDH0R to value 0"]
impl crate::Resettable for Tdh0rSpec {}
