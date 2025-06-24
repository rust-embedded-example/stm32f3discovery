#[doc = "Register `RDH1R` reader"]
pub type R = crate::R<Rdh1rSpec>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA5` reader - DATA5"]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA6` reader - DATA6"]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA7` reader - DATA7"]
pub type Data7R = crate::FieldReader;
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
#[doc = "receive FIFO mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdh1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rdh1rSpec;
impl crate::RegisterSpec for Rdh1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdh1r::R`](R) reader structure"]
impl crate::Readable for Rdh1rSpec {}
#[doc = "`reset()` method sets RDH1R to value 0"]
impl crate::Resettable for Rdh1rSpec {}
