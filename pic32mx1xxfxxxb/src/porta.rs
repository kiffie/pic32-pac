#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELA register"]
    pub ansela: ANSELA,
    #[doc = "0x04 - ANSELACLR register"]
    pub anselaclr: ANSELACLR,
    #[doc = "0x08 - ANSELASET register"]
    pub anselaset: ANSELASET,
    #[doc = "0x0c - ANSELAINV register"]
    pub anselainv: ANSELAINV,
    #[doc = "0x10 - TRISA register"]
    pub trisa: TRISA,
    #[doc = "0x14 - TRISACLR register"]
    pub trisaclr: TRISACLR,
    #[doc = "0x18 - TRISASET register"]
    pub trisaset: TRISASET,
    #[doc = "0x1c - TRISAINV register"]
    pub trisainv: TRISAINV,
    #[doc = "0x20 - PORTA register"]
    pub porta: PORTA,
    #[doc = "0x24 - PORTACLR register"]
    pub portaclr: PORTACLR,
    #[doc = "0x28 - PORTASET register"]
    pub portaset: PORTASET,
    #[doc = "0x2c - PORTAINV register"]
    pub portainv: PORTAINV,
    #[doc = "0x30 - LATA register"]
    pub lata: LATA,
    #[doc = "0x34 - LATACLR register"]
    pub lataclr: LATACLR,
    #[doc = "0x38 - LATASET register"]
    pub lataset: LATASET,
    #[doc = "0x3c - LATAINV register"]
    pub latainv: LATAINV,
    #[doc = "0x40 - ODCA register"]
    pub odca: ODCA,
    #[doc = "0x44 - ODCACLR register"]
    pub odcaclr: ODCACLR,
    #[doc = "0x48 - ODCASET register"]
    pub odcaset: ODCASET,
    #[doc = "0x4c - ODCAINV register"]
    pub odcainv: ODCAINV,
    #[doc = "0x50 - CNPUA register"]
    pub cnpua: CNPUA,
    #[doc = "0x54 - CNPUACLR register"]
    pub cnpuaclr: CNPUACLR,
    #[doc = "0x58 - CNPUASET register"]
    pub cnpuaset: CNPUASET,
    #[doc = "0x5c - CNPUAINV register"]
    pub cnpuainv: CNPUAINV,
    #[doc = "0x60 - CNPDA register"]
    pub cnpda: CNPDA,
    #[doc = "0x64 - CNPDACLR register"]
    pub cnpdaclr: CNPDACLR,
    #[doc = "0x68 - CNPDASET register"]
    pub cnpdaset: CNPDASET,
    #[doc = "0x6c - CNPDAINV register"]
    pub cnpdainv: CNPDAINV,
    #[doc = "0x70 - CNCONA register"]
    pub cncona: CNCONA,
    #[doc = "0x74 - CNCONACLR register"]
    pub cnconaclr: CNCONACLR,
    #[doc = "0x78 - CNCONASET register"]
    pub cnconaset: CNCONASET,
    #[doc = "0x7c - CNCONAINV register"]
    pub cnconainv: CNCONAINV,
    #[doc = "0x80 - CNENA register"]
    pub cnena: CNENA,
    #[doc = "0x84 - CNENACLR register"]
    pub cnenaclr: CNENACLR,
    #[doc = "0x88 - CNENASET register"]
    pub cnenaset: CNENASET,
    #[doc = "0x8c - CNENAINV register"]
    pub cnenainv: CNENAINV,
    #[doc = "0x90 - CNSTATA register"]
    pub cnstata: CNSTATA,
    #[doc = "0x94 - CNSTATACLR register"]
    pub cnstataclr: CNSTATACLR,
    #[doc = "0x98 - CNSTATASET register"]
    pub cnstataset: CNSTATASET,
    #[doc = "0x9c - CNSTATAINV register"]
    pub cnstatainv: CNSTATAINV,
}
#[doc = "ANSELA register"]
pub struct ANSELA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELA register"]
pub mod ansela;
#[doc = "ANSELACLR register"]
pub struct ANSELACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELACLR register"]
pub mod anselaclr;
#[doc = "ANSELASET register"]
pub struct ANSELASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELASET register"]
pub mod anselaset;
#[doc = "ANSELAINV register"]
pub struct ANSELAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELAINV register"]
pub mod anselainv;
#[doc = "TRISA register"]
pub struct TRISA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISA register"]
pub mod trisa;
#[doc = "TRISACLR register"]
pub struct TRISACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISACLR register"]
pub mod trisaclr;
#[doc = "TRISASET register"]
pub struct TRISASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISASET register"]
pub mod trisaset;
#[doc = "TRISAINV register"]
pub struct TRISAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISAINV register"]
pub mod trisainv;
#[doc = "PORTA register"]
pub struct PORTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTA register"]
pub mod porta;
#[doc = "PORTACLR register"]
pub struct PORTACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTACLR register"]
pub mod portaclr;
#[doc = "PORTASET register"]
pub struct PORTASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTASET register"]
pub mod portaset;
#[doc = "PORTAINV register"]
pub struct PORTAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTAINV register"]
pub mod portainv;
#[doc = "LATA register"]
pub struct LATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATA register"]
pub mod lata;
#[doc = "LATACLR register"]
pub struct LATACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATACLR register"]
pub mod lataclr;
#[doc = "LATASET register"]
pub struct LATASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATASET register"]
pub mod lataset;
#[doc = "LATAINV register"]
pub struct LATAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATAINV register"]
pub mod latainv;
#[doc = "ODCA register"]
pub struct ODCA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCA register"]
pub mod odca;
#[doc = "ODCACLR register"]
pub struct ODCACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCACLR register"]
pub mod odcaclr;
#[doc = "ODCASET register"]
pub struct ODCASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCASET register"]
pub mod odcaset;
#[doc = "ODCAINV register"]
pub struct ODCAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCAINV register"]
pub mod odcainv;
#[doc = "CNPUA register"]
pub struct CNPUA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUA register"]
pub mod cnpua;
#[doc = "CNPUACLR register"]
pub struct CNPUACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUACLR register"]
pub mod cnpuaclr;
#[doc = "CNPUASET register"]
pub struct CNPUASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUASET register"]
pub mod cnpuaset;
#[doc = "CNPUAINV register"]
pub struct CNPUAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUAINV register"]
pub mod cnpuainv;
#[doc = "CNPDA register"]
pub struct CNPDA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDA register"]
pub mod cnpda;
#[doc = "CNPDACLR register"]
pub struct CNPDACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDACLR register"]
pub mod cnpdaclr;
#[doc = "CNPDASET register"]
pub struct CNPDASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDASET register"]
pub mod cnpdaset;
#[doc = "CNPDAINV register"]
pub struct CNPDAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDAINV register"]
pub mod cnpdainv;
#[doc = "CNCONA register"]
pub struct CNCONA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONA register"]
pub mod cncona;
#[doc = "CNCONACLR register"]
pub struct CNCONACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONACLR register"]
pub mod cnconaclr;
#[doc = "CNCONASET register"]
pub struct CNCONASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONASET register"]
pub mod cnconaset;
#[doc = "CNCONAINV register"]
pub struct CNCONAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONAINV register"]
pub mod cnconainv;
#[doc = "CNENA register"]
pub struct CNENA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENA register"]
pub mod cnena;
#[doc = "CNENACLR register"]
pub struct CNENACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENACLR register"]
pub mod cnenaclr;
#[doc = "CNENASET register"]
pub struct CNENASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENASET register"]
pub mod cnenaset;
#[doc = "CNENAINV register"]
pub struct CNENAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENAINV register"]
pub mod cnenainv;
#[doc = "CNSTATA register"]
pub struct CNSTATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATA register"]
pub mod cnstata;
#[doc = "CNSTATACLR register"]
pub struct CNSTATACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATACLR register"]
pub mod cnstataclr;
#[doc = "CNSTATASET register"]
pub struct CNSTATASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATASET register"]
pub mod cnstataset;
#[doc = "CNSTATAINV register"]
pub struct CNSTATAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATAINV register"]
pub mod cnstatainv;
