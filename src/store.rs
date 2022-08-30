use crate::*;
use kvs::adapters::paged::PagedAdapter;
use kvs::adapters::spi::{SpiAdapterConfig, SpiStoreAdapter};
use kvs::*;

pub const KVS_BUCKETS: usize = 1024;
pub const KVS_SLOTS: usize = 32;
pub const KVS_MAX_HOPS: usize = 64;
pub const KVS_MAGIC: u32 = 0x10c0;
pub const FLASH_ADDR_BYTES: usize = 2;
pub const FLASH_PAGE_SIZE: usize = 128;

pub type FlashStoreError = kvs::Error<kvs::adapters::spi::Error<SpiDev, FlashChipSelect>>;

pub type FlashStore = KVStore<
    PagedAdapter<SpiStoreAdapter<SpiDev, FlashChipSelect, FLASH_ADDR_BYTES>, FLASH_PAGE_SIZE>,
    KVS_BUCKETS,
    KVS_SLOTS,
>;

pub fn store(
    spi_dev: stm32::SPI1,
    scl: FlashSck,
    miso: FlashMiso,
    mosi: FlashMosi,
    cs: FlashChipSelect,
    speed: Hertz,
    rcc: &mut Rcc,
) -> Result<FlashStore, FlashStoreError> {
    let cfg = SpiAdapterConfig::new(0xffff);
    let store_cfg = StoreConfig::new(KVS_MAGIC, KVS_MAX_HOPS);
    let flash = flash(spi_dev, scl, miso, mosi, speed, rcc);
    let adapter = PagedAdapter::new(SpiStoreAdapter::new(flash, cs, cfg));
    FlashStore::open(adapter, store_cfg, true)
}
