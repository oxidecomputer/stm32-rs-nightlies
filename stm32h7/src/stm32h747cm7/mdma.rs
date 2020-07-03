#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDMA Global Interrupt/Status Register"]
    pub gisr0: GISR0,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - MDMA channel x interrupt/status register"]
    pub c0isr: C0ISR,
    #[doc = "0x44 - MDMA channel x interrupt flag clear register"]
    pub c0ifcr: C0IFCR,
    #[doc = "0x48 - MDMA Channel x error status register"]
    pub c0esr: C0ESR,
    #[doc = "0x4c - This register is used to control the concerned channel."]
    pub c0cr: C0CR,
    #[doc = "0x50 - This register is used to configure the concerned channel."]
    pub c0tcr: C0TCR,
    #[doc = "0x54 - MDMA Channel x block number of data register"]
    pub c0bndtr: C0BNDTR,
    #[doc = "0x58 - MDMA channel x source address register"]
    pub c0sar: C0SAR,
    #[doc = "0x5c - MDMA channel x destination address register"]
    pub c0dar: C0DAR,
    #[doc = "0x60 - MDMA channel x Block Repeat address Update register"]
    pub c0brur: C0BRUR,
    #[doc = "0x64 - MDMA channel x Link Address register"]
    pub c0lar: C0LAR,
    #[doc = "0x68 - MDMA channel x Trigger and Bus selection Register"]
    pub c0tbr: C0TBR,
    _reserved12: [u8; 4usize],
    #[doc = "0x70 - MDMA channel x Mask address register"]
    pub c0mar: C0MAR,
    #[doc = "0x74 - MDMA channel x Mask Data register"]
    pub c0mdr: C0MDR,
    _reserved14: [u8; 8usize],
    #[doc = "0x80 - MDMA channel x interrupt/status register"]
    pub c1isr: C1ISR,
    #[doc = "0x84 - MDMA channel x interrupt flag clear register"]
    pub c1ifcr: C1IFCR,
    #[doc = "0x88 - MDMA Channel x error status register"]
    pub c1esr: C1ESR,
    #[doc = "0x8c - This register is used to control the concerned channel."]
    pub c1cr: C1CR,
    #[doc = "0x90 - This register is used to configure the concerned channel."]
    pub c1tcr: C1TCR,
    #[doc = "0x94 - MDMA Channel x block number of data register"]
    pub c1bndtr: C1BNDTR,
    #[doc = "0x98 - MDMA channel x source address register"]
    pub c1sar: C1SAR,
    #[doc = "0x9c - MDMA channel x destination address register"]
    pub c1dar: C1DAR,
    #[doc = "0xa0 - MDMA channel x Block Repeat address Update register"]
    pub c1brur: C1BRUR,
    #[doc = "0xa4 - MDMA channel x Link Address register"]
    pub c1lar: C1LAR,
    #[doc = "0xa8 - MDMA channel x Trigger and Bus selection Register"]
    pub c1tbr: C1TBR,
    _reserved25: [u8; 4usize],
    #[doc = "0xb0 - MDMA channel x Mask address register"]
    pub c1mar: C1MAR,
    #[doc = "0xb4 - MDMA channel x Mask Data register"]
    pub c1mdr: C1MDR,
    _reserved27: [u8; 8usize],
    #[doc = "0xc0 - MDMA channel x interrupt/status register"]
    pub c2isr: C2ISR,
    #[doc = "0xc4 - MDMA channel x interrupt flag clear register"]
    pub c2ifcr: C2IFCR,
    #[doc = "0xc8 - MDMA Channel x error status register"]
    pub c2esr: C2ESR,
    #[doc = "0xcc - This register is used to control the concerned channel."]
    pub c2cr: C2CR,
    #[doc = "0xd0 - This register is used to configure the concerned channel."]
    pub c2tcr: C2TCR,
    #[doc = "0xd4 - MDMA Channel x block number of data register"]
    pub c2bndtr: C2BNDTR,
    #[doc = "0xd8 - MDMA channel x source address register"]
    pub c2sar: C2SAR,
    #[doc = "0xdc - MDMA channel x destination address register"]
    pub c2dar: C2DAR,
    #[doc = "0xe0 - MDMA channel x Block Repeat address Update register"]
    pub c2brur: C2BRUR,
    #[doc = "0xe4 - MDMA channel x Link Address register"]
    pub c2lar: C2LAR,
    #[doc = "0xe8 - MDMA channel x Trigger and Bus selection Register"]
    pub c2tbr: C2TBR,
    _reserved38: [u8; 4usize],
    #[doc = "0xf0 - MDMA channel x Mask address register"]
    pub c2mar: C2MAR,
    #[doc = "0xf4 - MDMA channel x Mask Data register"]
    pub c2mdr: C2MDR,
    _reserved40: [u8; 8usize],
    #[doc = "0x100 - MDMA channel x interrupt/status register"]
    pub c3isr: C3ISR,
    #[doc = "0x104 - MDMA channel x interrupt flag clear register"]
    pub c3ifcr: C3IFCR,
    #[doc = "0x108 - MDMA Channel x error status register"]
    pub c3esr: C3ESR,
    #[doc = "0x10c - This register is used to control the concerned channel."]
    pub c3cr: C3CR,
    #[doc = "0x110 - This register is used to configure the concerned channel."]
    pub c3tcr: C3TCR,
    #[doc = "0x114 - MDMA Channel x block number of data register"]
    pub c3bndtr: C3BNDTR,
    #[doc = "0x118 - MDMA channel x source address register"]
    pub c3sar: C3SAR,
    #[doc = "0x11c - MDMA channel x destination address register"]
    pub c3dar: C3DAR,
    #[doc = "0x120 - MDMA channel x Block Repeat address Update register"]
    pub c3brur: C3BRUR,
    #[doc = "0x124 - MDMA channel x Link Address register"]
    pub c3lar: C3LAR,
    #[doc = "0x128 - MDMA channel x Trigger and Bus selection Register"]
    pub c3tbr: C3TBR,
    _reserved51: [u8; 4usize],
    #[doc = "0x130 - MDMA channel x Mask address register"]
    pub c3mar: C3MAR,
    #[doc = "0x134 - MDMA channel x Mask Data register"]
    pub c3mdr: C3MDR,
    _reserved53: [u8; 8usize],
    #[doc = "0x140 - MDMA channel x interrupt/status register"]
    pub c4isr: C4ISR,
    #[doc = "0x144 - MDMA channel x interrupt flag clear register"]
    pub c4ifcr: C4IFCR,
    #[doc = "0x148 - MDMA Channel x error status register"]
    pub c4esr: C4ESR,
    #[doc = "0x14c - This register is used to control the concerned channel."]
    pub c4cr: C4CR,
    #[doc = "0x150 - This register is used to configure the concerned channel."]
    pub c4tcr: C4TCR,
    #[doc = "0x154 - MDMA Channel x block number of data register"]
    pub c4bndtr: C4BNDTR,
    #[doc = "0x158 - MDMA channel x source address register"]
    pub c4sar: C4SAR,
    #[doc = "0x15c - MDMA channel x destination address register"]
    pub c4dar: C4DAR,
    #[doc = "0x160 - MDMA channel x Block Repeat address Update register"]
    pub c4brur: C4BRUR,
    #[doc = "0x164 - MDMA channel x Link Address register"]
    pub c4lar: C4LAR,
    #[doc = "0x168 - MDMA channel x Trigger and Bus selection Register"]
    pub c4tbr: C4TBR,
    _reserved64: [u8; 4usize],
    #[doc = "0x170 - MDMA channel x Mask address register"]
    pub c4mar: C4MAR,
    #[doc = "0x174 - MDMA channel x Mask Data register"]
    pub c4mdr: C4MDR,
    _reserved66: [u8; 8usize],
    #[doc = "0x180 - MDMA channel x interrupt/status register"]
    pub c5isr: C5ISR,
    #[doc = "0x184 - MDMA channel x interrupt flag clear register"]
    pub c5ifcr: C5IFCR,
    #[doc = "0x188 - MDMA Channel x error status register"]
    pub c5esr: C5ESR,
    #[doc = "0x18c - This register is used to control the concerned channel."]
    pub c5cr: C5CR,
    #[doc = "0x190 - This register is used to configure the concerned channel."]
    pub c5tcr: C5TCR,
    #[doc = "0x194 - MDMA Channel x block number of data register"]
    pub c5bndtr: C5BNDTR,
    #[doc = "0x198 - MDMA channel x source address register"]
    pub c5sar: C5SAR,
    #[doc = "0x19c - MDMA channel x destination address register"]
    pub c5dar: C5DAR,
    #[doc = "0x1a0 - MDMA channel x Block Repeat address Update register"]
    pub c5brur: C5BRUR,
    #[doc = "0x1a4 - MDMA channel x Link Address register"]
    pub c5lar: C5LAR,
    #[doc = "0x1a8 - MDMA channel x Trigger and Bus selection Register"]
    pub c5tbr: C5TBR,
    _reserved77: [u8; 4usize],
    #[doc = "0x1b0 - MDMA channel x Mask address register"]
    pub c5mar: C5MAR,
    #[doc = "0x1b4 - MDMA channel x Mask Data register"]
    pub c5mdr: C5MDR,
    _reserved79: [u8; 8usize],
    #[doc = "0x1c0 - MDMA channel x interrupt/status register"]
    pub c6isr: C6ISR,
    #[doc = "0x1c4 - MDMA channel x interrupt flag clear register"]
    pub c6ifcr: C6IFCR,
    #[doc = "0x1c8 - MDMA Channel x error status register"]
    pub c6esr: C6ESR,
    #[doc = "0x1cc - This register is used to control the concerned channel."]
    pub c6cr: C6CR,
    #[doc = "0x1d0 - This register is used to configure the concerned channel."]
    pub c6tcr: C6TCR,
    #[doc = "0x1d4 - MDMA Channel x block number of data register"]
    pub c6bndtr: C6BNDTR,
    #[doc = "0x1d8 - MDMA channel x source address register"]
    pub c6sar: C6SAR,
    #[doc = "0x1dc - MDMA channel x destination address register"]
    pub c6dar: C6DAR,
    #[doc = "0x1e0 - MDMA channel x Block Repeat address Update register"]
    pub c6brur: C6BRUR,
    #[doc = "0x1e4 - MDMA channel x Link Address register"]
    pub c6lar: C6LAR,
    #[doc = "0x1e8 - MDMA channel x Trigger and Bus selection Register"]
    pub c6tbr: C6TBR,
    _reserved90: [u8; 4usize],
    #[doc = "0x1f0 - MDMA channel x Mask address register"]
    pub c6mar: C6MAR,
    #[doc = "0x1f4 - MDMA channel x Mask Data register"]
    pub c6mdr: C6MDR,
    _reserved92: [u8; 8usize],
    #[doc = "0x200 - MDMA channel x interrupt/status register"]
    pub c7isr: C7ISR,
    #[doc = "0x204 - MDMA channel x interrupt flag clear register"]
    pub c7ifcr: C7IFCR,
    #[doc = "0x208 - MDMA Channel x error status register"]
    pub c7esr: C7ESR,
    #[doc = "0x20c - This register is used to control the concerned channel."]
    pub c7cr: C7CR,
    #[doc = "0x210 - This register is used to configure the concerned channel."]
    pub c7tcr: C7TCR,
    #[doc = "0x214 - MDMA Channel x block number of data register"]
    pub c7bndtr: C7BNDTR,
    #[doc = "0x218 - MDMA channel x source address register"]
    pub c7sar: C7SAR,
    #[doc = "0x21c - MDMA channel x destination address register"]
    pub c7dar: C7DAR,
    #[doc = "0x220 - MDMA channel x Block Repeat address Update register"]
    pub c7brur: C7BRUR,
    #[doc = "0x224 - MDMA channel x Link Address register"]
    pub c7lar: C7LAR,
    #[doc = "0x228 - MDMA channel x Trigger and Bus selection Register"]
    pub c7tbr: C7TBR,
    _reserved103: [u8; 4usize],
    #[doc = "0x230 - MDMA channel x Mask address register"]
    pub c7mar: C7MAR,
    #[doc = "0x234 - MDMA channel x Mask Data register"]
    pub c7mdr: C7MDR,
    _reserved105: [u8; 8usize],
    #[doc = "0x240 - MDMA channel x interrupt/status register"]
    pub c8isr: C8ISR,
    #[doc = "0x244 - MDMA channel x interrupt flag clear register"]
    pub c8ifcr: C8IFCR,
    #[doc = "0x248 - MDMA Channel x error status register"]
    pub c8esr: C8ESR,
    #[doc = "0x24c - This register is used to control the concerned channel."]
    pub c8cr: C8CR,
    #[doc = "0x250 - This register is used to configure the concerned channel."]
    pub c8tcr: C8TCR,
    #[doc = "0x254 - MDMA Channel x block number of data register"]
    pub c8bndtr: C8BNDTR,
    #[doc = "0x258 - MDMA channel x source address register"]
    pub c8sar: C8SAR,
    #[doc = "0x25c - MDMA channel x destination address register"]
    pub c8dar: C8DAR,
    #[doc = "0x260 - MDMA channel x Block Repeat address Update register"]
    pub c8brur: C8BRUR,
    #[doc = "0x264 - MDMA channel x Link Address register"]
    pub c8lar: C8LAR,
    #[doc = "0x268 - MDMA channel x Trigger and Bus selection Register"]
    pub c8tbr: C8TBR,
    _reserved116: [u8; 4usize],
    #[doc = "0x270 - MDMA channel x Mask address register"]
    pub c8mar: C8MAR,
    #[doc = "0x274 - MDMA channel x Mask Data register"]
    pub c8mdr: C8MDR,
    _reserved118: [u8; 8usize],
    #[doc = "0x280 - MDMA channel x interrupt/status register"]
    pub c9isr: C9ISR,
    #[doc = "0x284 - MDMA channel x interrupt flag clear register"]
    pub c9ifcr: C9IFCR,
    #[doc = "0x288 - MDMA Channel x error status register"]
    pub c9esr: C9ESR,
    #[doc = "0x28c - This register is used to control the concerned channel."]
    pub c9cr: C9CR,
    #[doc = "0x290 - This register is used to configure the concerned channel."]
    pub c9tcr: C9TCR,
    #[doc = "0x294 - MDMA Channel x block number of data register"]
    pub c9bndtr: C9BNDTR,
    #[doc = "0x298 - MDMA channel x source address register"]
    pub c9sar: C9SAR,
    #[doc = "0x29c - MDMA channel x destination address register"]
    pub c9dar: C9DAR,
    #[doc = "0x2a0 - MDMA channel x Block Repeat address Update register"]
    pub c9brur: C9BRUR,
    #[doc = "0x2a4 - MDMA channel x Link Address register"]
    pub c9lar: C9LAR,
    #[doc = "0x2a8 - MDMA channel x Trigger and Bus selection Register"]
    pub c9tbr: C9TBR,
    _reserved129: [u8; 4usize],
    #[doc = "0x2b0 - MDMA channel x Mask address register"]
    pub c9mar: C9MAR,
    #[doc = "0x2b4 - MDMA channel x Mask Data register"]
    pub c9mdr: C9MDR,
    _reserved131: [u8; 8usize],
    #[doc = "0x2c0 - MDMA channel x interrupt/status register"]
    pub c10isr: C10ISR,
    #[doc = "0x2c4 - MDMA channel x interrupt flag clear register"]
    pub c10ifcr: C10IFCR,
    #[doc = "0x2c8 - MDMA Channel x error status register"]
    pub c10esr: C10ESR,
    #[doc = "0x2cc - This register is used to control the concerned channel."]
    pub c10cr: C10CR,
    #[doc = "0x2d0 - This register is used to configure the concerned channel."]
    pub c10tcr: C10TCR,
    #[doc = "0x2d4 - MDMA Channel x block number of data register"]
    pub c10bndtr: C10BNDTR,
    #[doc = "0x2d8 - MDMA channel x source address register"]
    pub c10sar: C10SAR,
    #[doc = "0x2dc - MDMA channel x destination address register"]
    pub c10dar: C10DAR,
    #[doc = "0x2e0 - MDMA channel x Block Repeat address Update register"]
    pub c10brur: C10BRUR,
    #[doc = "0x2e4 - MDMA channel x Link Address register"]
    pub c10lar: C10LAR,
    #[doc = "0x2e8 - MDMA channel x Trigger and Bus selection Register"]
    pub c10tbr: C10TBR,
    _reserved142: [u8; 4usize],
    #[doc = "0x2f0 - MDMA channel x Mask address register"]
    pub c10mar: C10MAR,
    #[doc = "0x2f4 - MDMA channel x Mask Data register"]
    pub c10mdr: C10MDR,
    _reserved144: [u8; 8usize],
    #[doc = "0x300 - MDMA channel x interrupt/status register"]
    pub c11isr: C11ISR,
    #[doc = "0x304 - MDMA channel x interrupt flag clear register"]
    pub c11ifcr: C11IFCR,
    #[doc = "0x308 - MDMA Channel x error status register"]
    pub c11esr: C11ESR,
    #[doc = "0x30c - This register is used to control the concerned channel."]
    pub c11cr: C11CR,
    #[doc = "0x310 - This register is used to configure the concerned channel."]
    pub c11tcr: C11TCR,
    #[doc = "0x314 - MDMA Channel x block number of data register"]
    pub c11bndtr: C11BNDTR,
    #[doc = "0x318 - MDMA channel x source address register"]
    pub c11sar: C11SAR,
    #[doc = "0x31c - MDMA channel x destination address register"]
    pub c11dar: C11DAR,
    #[doc = "0x320 - MDMA channel x Block Repeat address Update register"]
    pub c11brur: C11BRUR,
    #[doc = "0x324 - MDMA channel x Link Address register"]
    pub c11lar: C11LAR,
    #[doc = "0x328 - MDMA channel x Trigger and Bus selection Register"]
    pub c11tbr: C11TBR,
    _reserved155: [u8; 4usize],
    #[doc = "0x330 - MDMA channel x Mask address register"]
    pub c11mar: C11MAR,
    #[doc = "0x334 - MDMA channel x Mask Data register"]
    pub c11mdr: C11MDR,
    _reserved157: [u8; 8usize],
    #[doc = "0x340 - MDMA channel x interrupt/status register"]
    pub c12isr: C12ISR,
    #[doc = "0x344 - MDMA channel x interrupt flag clear register"]
    pub c12ifcr: C12IFCR,
    #[doc = "0x348 - MDMA Channel x error status register"]
    pub c12esr: C12ESR,
    #[doc = "0x34c - This register is used to control the concerned channel."]
    pub c12cr: C12CR,
    #[doc = "0x350 - This register is used to configure the concerned channel."]
    pub c12tcr: C12TCR,
    #[doc = "0x354 - MDMA Channel x block number of data register"]
    pub c12bndtr: C12BNDTR,
    #[doc = "0x358 - MDMA channel x source address register"]
    pub c12sar: C12SAR,
    #[doc = "0x35c - MDMA channel x destination address register"]
    pub c12dar: C12DAR,
    #[doc = "0x360 - MDMA channel x Block Repeat address Update register"]
    pub c12brur: C12BRUR,
    #[doc = "0x364 - MDMA channel x Link Address register"]
    pub c12lar: C12LAR,
    #[doc = "0x368 - MDMA channel x Trigger and Bus selection Register"]
    pub c12tbr: C12TBR,
    _reserved168: [u8; 4usize],
    #[doc = "0x370 - MDMA channel x Mask address register"]
    pub c12mar: C12MAR,
    #[doc = "0x374 - MDMA channel x Mask Data register"]
    pub c12mdr: C12MDR,
    _reserved170: [u8; 8usize],
    #[doc = "0x380 - MDMA channel x interrupt/status register"]
    pub c13isr: C13ISR,
    #[doc = "0x384 - MDMA channel x interrupt flag clear register"]
    pub c13ifcr: C13IFCR,
    #[doc = "0x388 - MDMA Channel x error status register"]
    pub c13esr: C13ESR,
    #[doc = "0x38c - This register is used to control the concerned channel."]
    pub c13cr: C13CR,
    #[doc = "0x390 - This register is used to configure the concerned channel."]
    pub c13tcr: C13TCR,
    #[doc = "0x394 - MDMA Channel x block number of data register"]
    pub c13bndtr: C13BNDTR,
    #[doc = "0x398 - MDMA channel x source address register"]
    pub c13sar: C13SAR,
    #[doc = "0x39c - MDMA channel x destination address register"]
    pub c13dar: C13DAR,
    #[doc = "0x3a0 - MDMA channel x Block Repeat address Update register"]
    pub c13brur: C13BRUR,
    #[doc = "0x3a4 - MDMA channel x Link Address register"]
    pub c13lar: C13LAR,
    #[doc = "0x3a8 - MDMA channel x Trigger and Bus selection Register"]
    pub c13tbr: C13TBR,
    _reserved181: [u8; 4usize],
    #[doc = "0x3b0 - MDMA channel x Mask address register"]
    pub c13mar: C13MAR,
    #[doc = "0x3b4 - MDMA channel x Mask Data register"]
    pub c13mdr: C13MDR,
    _reserved183: [u8; 8usize],
    #[doc = "0x3c0 - MDMA channel x interrupt/status register"]
    pub c14isr: C14ISR,
    #[doc = "0x3c4 - MDMA channel x interrupt flag clear register"]
    pub c14ifcr: C14IFCR,
    #[doc = "0x3c8 - MDMA Channel x error status register"]
    pub c14esr: C14ESR,
    #[doc = "0x3cc - This register is used to control the concerned channel."]
    pub c14cr: C14CR,
    #[doc = "0x3d0 - This register is used to configure the concerned channel."]
    pub c14tcr: C14TCR,
    #[doc = "0x3d4 - MDMA Channel x block number of data register"]
    pub c14bndtr: C14BNDTR,
    #[doc = "0x3d8 - MDMA channel x source address register"]
    pub c14sar: C14SAR,
    #[doc = "0x3dc - MDMA channel x destination address register"]
    pub c14dar: C14DAR,
    #[doc = "0x3e0 - MDMA channel x Block Repeat address Update register"]
    pub c14brur: C14BRUR,
    #[doc = "0x3e4 - MDMA channel x Link Address register"]
    pub c14lar: C14LAR,
    #[doc = "0x3e8 - MDMA channel x Trigger and Bus selection Register"]
    pub c14tbr: C14TBR,
    _reserved194: [u8; 4usize],
    #[doc = "0x3f0 - MDMA channel x Mask address register"]
    pub c14mar: C14MAR,
    #[doc = "0x3f4 - MDMA channel x Mask Data register"]
    pub c14mdr: C14MDR,
    _reserved196: [u8; 8usize],
    #[doc = "0x400 - MDMA channel x interrupt/status register"]
    pub c15isr: C15ISR,
    #[doc = "0x404 - MDMA channel x interrupt flag clear register"]
    pub c15ifcr: C15IFCR,
    #[doc = "0x408 - MDMA Channel x error status register"]
    pub c15esr: C15ESR,
    #[doc = "0x40c - This register is used to control the concerned channel."]
    pub c15cr: C15CR,
    #[doc = "0x410 - This register is used to configure the concerned channel."]
    pub c15tcr: C15TCR,
    #[doc = "0x414 - MDMA Channel x block number of data register"]
    pub c15bndtr: C15BNDTR,
    #[doc = "0x418 - MDMA channel x source address register"]
    pub c15sar: C15SAR,
    #[doc = "0x41c - MDMA channel x destination address register"]
    pub c15dar: C15DAR,
    #[doc = "0x420 - MDMA channel x Block Repeat address Update register"]
    pub c15brur: C15BRUR,
    #[doc = "0x424 - MDMA channel x Link Address register"]
    pub c15lar: C15LAR,
    #[doc = "0x428 - MDMA channel x Trigger and Bus selection Register"]
    pub c15tbr: C15TBR,
    _reserved207: [u8; 4usize],
    #[doc = "0x430 - MDMA channel x Mask address register"]
    pub c15mar: C15MAR,
    #[doc = "0x434 - MDMA channel x Mask Data register"]
    pub c15mdr: C15MDR,
}
#[doc = "MDMA Global Interrupt/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gisr0](gisr0) module"]
pub type GISR0 = crate::Reg<u32, _GISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GISR0;
#[doc = "`read()` method returns [gisr0::R](gisr0::R) reader structure"]
impl crate::Readable for GISR0 {}
#[doc = "MDMA Global Interrupt/Status Register"]
pub mod gisr0;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0isr](c0isr) module"]
pub type C0ISR = crate::Reg<u32, _C0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0ISR;
#[doc = "`read()` method returns [c0isr::R](c0isr::R) reader structure"]
impl crate::Readable for C0ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c0isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0ifcr](c0ifcr) module"]
pub type C0IFCR = crate::Reg<u32, _C0IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0IFCR;
#[doc = "`write(|w| ..)` method takes [c0ifcr::W](c0ifcr::W) writer structure"]
impl crate::Writable for C0IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c0ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0esr](c0esr) module"]
pub type C0ESR = crate::Reg<u32, _C0ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0ESR;
#[doc = "`read()` method returns [c0esr::R](c0esr::R) reader structure"]
impl crate::Readable for C0ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c0esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0cr](c0cr) module"]
pub type C0CR = crate::Reg<u32, _C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0CR;
#[doc = "`read()` method returns [c0cr::R](c0cr::R) reader structure"]
impl crate::Readable for C0CR {}
#[doc = "`write(|w| ..)` method takes [c0cr::W](c0cr::W) writer structure"]
impl crate::Writable for C0CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c0cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0tcr](c0tcr) module"]
pub type C0TCR = crate::Reg<u32, _C0TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0TCR;
#[doc = "`read()` method returns [c0tcr::R](c0tcr::R) reader structure"]
impl crate::Readable for C0TCR {}
#[doc = "`write(|w| ..)` method takes [c0tcr::W](c0tcr::W) writer structure"]
impl crate::Writable for C0TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c0tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0bndtr](c0bndtr) module"]
pub type C0BNDTR = crate::Reg<u32, _C0BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0BNDTR;
#[doc = "`read()` method returns [c0bndtr::R](c0bndtr::R) reader structure"]
impl crate::Readable for C0BNDTR {}
#[doc = "`write(|w| ..)` method takes [c0bndtr::W](c0bndtr::W) writer structure"]
impl crate::Writable for C0BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c0bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0sar](c0sar) module"]
pub type C0SAR = crate::Reg<u32, _C0SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0SAR;
#[doc = "`read()` method returns [c0sar::R](c0sar::R) reader structure"]
impl crate::Readable for C0SAR {}
#[doc = "`write(|w| ..)` method takes [c0sar::W](c0sar::W) writer structure"]
impl crate::Writable for C0SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c0sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0dar](c0dar) module"]
pub type C0DAR = crate::Reg<u32, _C0DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0DAR;
#[doc = "`read()` method returns [c0dar::R](c0dar::R) reader structure"]
impl crate::Readable for C0DAR {}
#[doc = "`write(|w| ..)` method takes [c0dar::W](c0dar::W) writer structure"]
impl crate::Writable for C0DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c0dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0brur](c0brur) module"]
pub type C0BRUR = crate::Reg<u32, _C0BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0BRUR;
#[doc = "`read()` method returns [c0brur::R](c0brur::R) reader structure"]
impl crate::Readable for C0BRUR {}
#[doc = "`write(|w| ..)` method takes [c0brur::W](c0brur::W) writer structure"]
impl crate::Writable for C0BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c0brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0lar](c0lar) module"]
pub type C0LAR = crate::Reg<u32, _C0LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0LAR;
#[doc = "`read()` method returns [c0lar::R](c0lar::R) reader structure"]
impl crate::Readable for C0LAR {}
#[doc = "`write(|w| ..)` method takes [c0lar::W](c0lar::W) writer structure"]
impl crate::Writable for C0LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c0lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0tbr](c0tbr) module"]
pub type C0TBR = crate::Reg<u32, _C0TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0TBR;
#[doc = "`read()` method returns [c0tbr::R](c0tbr::R) reader structure"]
impl crate::Readable for C0TBR {}
#[doc = "`write(|w| ..)` method takes [c0tbr::W](c0tbr::W) writer structure"]
impl crate::Writable for C0TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c0tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0mar](c0mar) module"]
pub type C0MAR = crate::Reg<u32, _C0MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0MAR;
#[doc = "`read()` method returns [c0mar::R](c0mar::R) reader structure"]
impl crate::Readable for C0MAR {}
#[doc = "`write(|w| ..)` method takes [c0mar::W](c0mar::W) writer structure"]
impl crate::Writable for C0MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c0mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0mdr](c0mdr) module"]
pub type C0MDR = crate::Reg<u32, _C0MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0MDR;
#[doc = "`read()` method returns [c0mdr::R](c0mdr::R) reader structure"]
impl crate::Readable for C0MDR {}
#[doc = "`write(|w| ..)` method takes [c0mdr::W](c0mdr::W) writer structure"]
impl crate::Writable for C0MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c0mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1isr](c1isr) module"]
pub type C1ISR = crate::Reg<u32, _C1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1ISR;
#[doc = "`read()` method returns [c1isr::R](c1isr::R) reader structure"]
impl crate::Readable for C1ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c1isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1ifcr](c1ifcr) module"]
pub type C1IFCR = crate::Reg<u32, _C1IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IFCR;
#[doc = "`write(|w| ..)` method takes [c1ifcr::W](c1ifcr::W) writer structure"]
impl crate::Writable for C1IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c1ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1esr](c1esr) module"]
pub type C1ESR = crate::Reg<u32, _C1ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1ESR;
#[doc = "`read()` method returns [c1esr::R](c1esr::R) reader structure"]
impl crate::Readable for C1ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c1esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1cr](c1cr) module"]
pub type C1CR = crate::Reg<u32, _C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1CR;
#[doc = "`read()` method returns [c1cr::R](c1cr::R) reader structure"]
impl crate::Readable for C1CR {}
#[doc = "`write(|w| ..)` method takes [c1cr::W](c1cr::W) writer structure"]
impl crate::Writable for C1CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c1cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1tcr](c1tcr) module"]
pub type C1TCR = crate::Reg<u32, _C1TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1TCR;
#[doc = "`read()` method returns [c1tcr::R](c1tcr::R) reader structure"]
impl crate::Readable for C1TCR {}
#[doc = "`write(|w| ..)` method takes [c1tcr::W](c1tcr::W) writer structure"]
impl crate::Writable for C1TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c1tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1bndtr](c1bndtr) module"]
pub type C1BNDTR = crate::Reg<u32, _C1BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1BNDTR;
#[doc = "`read()` method returns [c1bndtr::R](c1bndtr::R) reader structure"]
impl crate::Readable for C1BNDTR {}
#[doc = "`write(|w| ..)` method takes [c1bndtr::W](c1bndtr::W) writer structure"]
impl crate::Writable for C1BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c1bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1sar](c1sar) module"]
pub type C1SAR = crate::Reg<u32, _C1SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1SAR;
#[doc = "`read()` method returns [c1sar::R](c1sar::R) reader structure"]
impl crate::Readable for C1SAR {}
#[doc = "`write(|w| ..)` method takes [c1sar::W](c1sar::W) writer structure"]
impl crate::Writable for C1SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c1sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1dar](c1dar) module"]
pub type C1DAR = crate::Reg<u32, _C1DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1DAR;
#[doc = "`read()` method returns [c1dar::R](c1dar::R) reader structure"]
impl crate::Readable for C1DAR {}
#[doc = "`write(|w| ..)` method takes [c1dar::W](c1dar::W) writer structure"]
impl crate::Writable for C1DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c1dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1brur](c1brur) module"]
pub type C1BRUR = crate::Reg<u32, _C1BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1BRUR;
#[doc = "`read()` method returns [c1brur::R](c1brur::R) reader structure"]
impl crate::Readable for C1BRUR {}
#[doc = "`write(|w| ..)` method takes [c1brur::W](c1brur::W) writer structure"]
impl crate::Writable for C1BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c1brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1lar](c1lar) module"]
pub type C1LAR = crate::Reg<u32, _C1LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1LAR;
#[doc = "`read()` method returns [c1lar::R](c1lar::R) reader structure"]
impl crate::Readable for C1LAR {}
#[doc = "`write(|w| ..)` method takes [c1lar::W](c1lar::W) writer structure"]
impl crate::Writable for C1LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c1lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1tbr](c1tbr) module"]
pub type C1TBR = crate::Reg<u32, _C1TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1TBR;
#[doc = "`read()` method returns [c1tbr::R](c1tbr::R) reader structure"]
impl crate::Readable for C1TBR {}
#[doc = "`write(|w| ..)` method takes [c1tbr::W](c1tbr::W) writer structure"]
impl crate::Writable for C1TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c1tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1mar](c1mar) module"]
pub type C1MAR = crate::Reg<u32, _C1MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1MAR;
#[doc = "`read()` method returns [c1mar::R](c1mar::R) reader structure"]
impl crate::Readable for C1MAR {}
#[doc = "`write(|w| ..)` method takes [c1mar::W](c1mar::W) writer structure"]
impl crate::Writable for C1MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c1mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1mdr](c1mdr) module"]
pub type C1MDR = crate::Reg<u32, _C1MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1MDR;
#[doc = "`read()` method returns [c1mdr::R](c1mdr::R) reader structure"]
impl crate::Readable for C1MDR {}
#[doc = "`write(|w| ..)` method takes [c1mdr::W](c1mdr::W) writer structure"]
impl crate::Writable for C1MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c1mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2isr](c2isr) module"]
pub type C2ISR = crate::Reg<u32, _C2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2ISR;
#[doc = "`read()` method returns [c2isr::R](c2isr::R) reader structure"]
impl crate::Readable for C2ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c2isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ifcr](c2ifcr) module"]
pub type C2IFCR = crate::Reg<u32, _C2IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IFCR;
#[doc = "`write(|w| ..)` method takes [c2ifcr::W](c2ifcr::W) writer structure"]
impl crate::Writable for C2IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c2ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2esr](c2esr) module"]
pub type C2ESR = crate::Reg<u32, _C2ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2ESR;
#[doc = "`read()` method returns [c2esr::R](c2esr::R) reader structure"]
impl crate::Readable for C2ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c2esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](c2cr) module"]
pub type C2CR = crate::Reg<u32, _C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR;
#[doc = "`read()` method returns [c2cr::R](c2cr::R) reader structure"]
impl crate::Readable for C2CR {}
#[doc = "`write(|w| ..)` method takes [c2cr::W](c2cr::W) writer structure"]
impl crate::Writable for C2CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c2cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2tcr](c2tcr) module"]
pub type C2TCR = crate::Reg<u32, _C2TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2TCR;
#[doc = "`read()` method returns [c2tcr::R](c2tcr::R) reader structure"]
impl crate::Readable for C2TCR {}
#[doc = "`write(|w| ..)` method takes [c2tcr::W](c2tcr::W) writer structure"]
impl crate::Writable for C2TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c2tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2bndtr](c2bndtr) module"]
pub type C2BNDTR = crate::Reg<u32, _C2BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2BNDTR;
#[doc = "`read()` method returns [c2bndtr::R](c2bndtr::R) reader structure"]
impl crate::Readable for C2BNDTR {}
#[doc = "`write(|w| ..)` method takes [c2bndtr::W](c2bndtr::W) writer structure"]
impl crate::Writable for C2BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c2bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2sar](c2sar) module"]
pub type C2SAR = crate::Reg<u32, _C2SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2SAR;
#[doc = "`read()` method returns [c2sar::R](c2sar::R) reader structure"]
impl crate::Readable for C2SAR {}
#[doc = "`write(|w| ..)` method takes [c2sar::W](c2sar::W) writer structure"]
impl crate::Writable for C2SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c2sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2dar](c2dar) module"]
pub type C2DAR = crate::Reg<u32, _C2DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2DAR;
#[doc = "`read()` method returns [c2dar::R](c2dar::R) reader structure"]
impl crate::Readable for C2DAR {}
#[doc = "`write(|w| ..)` method takes [c2dar::W](c2dar::W) writer structure"]
impl crate::Writable for C2DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c2dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2brur](c2brur) module"]
pub type C2BRUR = crate::Reg<u32, _C2BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2BRUR;
#[doc = "`read()` method returns [c2brur::R](c2brur::R) reader structure"]
impl crate::Readable for C2BRUR {}
#[doc = "`write(|w| ..)` method takes [c2brur::W](c2brur::W) writer structure"]
impl crate::Writable for C2BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c2brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2lar](c2lar) module"]
pub type C2LAR = crate::Reg<u32, _C2LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2LAR;
#[doc = "`read()` method returns [c2lar::R](c2lar::R) reader structure"]
impl crate::Readable for C2LAR {}
#[doc = "`write(|w| ..)` method takes [c2lar::W](c2lar::W) writer structure"]
impl crate::Writable for C2LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c2lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2tbr](c2tbr) module"]
pub type C2TBR = crate::Reg<u32, _C2TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2TBR;
#[doc = "`read()` method returns [c2tbr::R](c2tbr::R) reader structure"]
impl crate::Readable for C2TBR {}
#[doc = "`write(|w| ..)` method takes [c2tbr::W](c2tbr::W) writer structure"]
impl crate::Writable for C2TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c2tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2mar](c2mar) module"]
pub type C2MAR = crate::Reg<u32, _C2MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2MAR;
#[doc = "`read()` method returns [c2mar::R](c2mar::R) reader structure"]
impl crate::Readable for C2MAR {}
#[doc = "`write(|w| ..)` method takes [c2mar::W](c2mar::W) writer structure"]
impl crate::Writable for C2MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c2mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2mdr](c2mdr) module"]
pub type C2MDR = crate::Reg<u32, _C2MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2MDR;
#[doc = "`read()` method returns [c2mdr::R](c2mdr::R) reader structure"]
impl crate::Readable for C2MDR {}
#[doc = "`write(|w| ..)` method takes [c2mdr::W](c2mdr::W) writer structure"]
impl crate::Writable for C2MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c2mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3isr](c3isr) module"]
pub type C3ISR = crate::Reg<u32, _C3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3ISR;
#[doc = "`read()` method returns [c3isr::R](c3isr::R) reader structure"]
impl crate::Readable for C3ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c3isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3ifcr](c3ifcr) module"]
pub type C3IFCR = crate::Reg<u32, _C3IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3IFCR;
#[doc = "`write(|w| ..)` method takes [c3ifcr::W](c3ifcr::W) writer structure"]
impl crate::Writable for C3IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c3ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3esr](c3esr) module"]
pub type C3ESR = crate::Reg<u32, _C3ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3ESR;
#[doc = "`read()` method returns [c3esr::R](c3esr::R) reader structure"]
impl crate::Readable for C3ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c3esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3cr](c3cr) module"]
pub type C3CR = crate::Reg<u32, _C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3CR;
#[doc = "`read()` method returns [c3cr::R](c3cr::R) reader structure"]
impl crate::Readable for C3CR {}
#[doc = "`write(|w| ..)` method takes [c3cr::W](c3cr::W) writer structure"]
impl crate::Writable for C3CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c3cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3tcr](c3tcr) module"]
pub type C3TCR = crate::Reg<u32, _C3TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3TCR;
#[doc = "`read()` method returns [c3tcr::R](c3tcr::R) reader structure"]
impl crate::Readable for C3TCR {}
#[doc = "`write(|w| ..)` method takes [c3tcr::W](c3tcr::W) writer structure"]
impl crate::Writable for C3TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c3tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3bndtr](c3bndtr) module"]
pub type C3BNDTR = crate::Reg<u32, _C3BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3BNDTR;
#[doc = "`read()` method returns [c3bndtr::R](c3bndtr::R) reader structure"]
impl crate::Readable for C3BNDTR {}
#[doc = "`write(|w| ..)` method takes [c3bndtr::W](c3bndtr::W) writer structure"]
impl crate::Writable for C3BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c3bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3sar](c3sar) module"]
pub type C3SAR = crate::Reg<u32, _C3SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3SAR;
#[doc = "`read()` method returns [c3sar::R](c3sar::R) reader structure"]
impl crate::Readable for C3SAR {}
#[doc = "`write(|w| ..)` method takes [c3sar::W](c3sar::W) writer structure"]
impl crate::Writable for C3SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c3sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3dar](c3dar) module"]
pub type C3DAR = crate::Reg<u32, _C3DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3DAR;
#[doc = "`read()` method returns [c3dar::R](c3dar::R) reader structure"]
impl crate::Readable for C3DAR {}
#[doc = "`write(|w| ..)` method takes [c3dar::W](c3dar::W) writer structure"]
impl crate::Writable for C3DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c3dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3brur](c3brur) module"]
pub type C3BRUR = crate::Reg<u32, _C3BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3BRUR;
#[doc = "`read()` method returns [c3brur::R](c3brur::R) reader structure"]
impl crate::Readable for C3BRUR {}
#[doc = "`write(|w| ..)` method takes [c3brur::W](c3brur::W) writer structure"]
impl crate::Writable for C3BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c3brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3lar](c3lar) module"]
pub type C3LAR = crate::Reg<u32, _C3LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3LAR;
#[doc = "`read()` method returns [c3lar::R](c3lar::R) reader structure"]
impl crate::Readable for C3LAR {}
#[doc = "`write(|w| ..)` method takes [c3lar::W](c3lar::W) writer structure"]
impl crate::Writable for C3LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c3lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3tbr](c3tbr) module"]
pub type C3TBR = crate::Reg<u32, _C3TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3TBR;
#[doc = "`read()` method returns [c3tbr::R](c3tbr::R) reader structure"]
impl crate::Readable for C3TBR {}
#[doc = "`write(|w| ..)` method takes [c3tbr::W](c3tbr::W) writer structure"]
impl crate::Writable for C3TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c3tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3mar](c3mar) module"]
pub type C3MAR = crate::Reg<u32, _C3MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3MAR;
#[doc = "`read()` method returns [c3mar::R](c3mar::R) reader structure"]
impl crate::Readable for C3MAR {}
#[doc = "`write(|w| ..)` method takes [c3mar::W](c3mar::W) writer structure"]
impl crate::Writable for C3MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c3mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3mdr](c3mdr) module"]
pub type C3MDR = crate::Reg<u32, _C3MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3MDR;
#[doc = "`read()` method returns [c3mdr::R](c3mdr::R) reader structure"]
impl crate::Readable for C3MDR {}
#[doc = "`write(|w| ..)` method takes [c3mdr::W](c3mdr::W) writer structure"]
impl crate::Writable for C3MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c3mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4isr](c4isr) module"]
pub type C4ISR = crate::Reg<u32, _C4ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4ISR;
#[doc = "`read()` method returns [c4isr::R](c4isr::R) reader structure"]
impl crate::Readable for C4ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c4isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4ifcr](c4ifcr) module"]
pub type C4IFCR = crate::Reg<u32, _C4IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4IFCR;
#[doc = "`write(|w| ..)` method takes [c4ifcr::W](c4ifcr::W) writer structure"]
impl crate::Writable for C4IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c4ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4esr](c4esr) module"]
pub type C4ESR = crate::Reg<u32, _C4ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4ESR;
#[doc = "`read()` method returns [c4esr::R](c4esr::R) reader structure"]
impl crate::Readable for C4ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c4esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4cr](c4cr) module"]
pub type C4CR = crate::Reg<u32, _C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4CR;
#[doc = "`read()` method returns [c4cr::R](c4cr::R) reader structure"]
impl crate::Readable for C4CR {}
#[doc = "`write(|w| ..)` method takes [c4cr::W](c4cr::W) writer structure"]
impl crate::Writable for C4CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c4cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4tcr](c4tcr) module"]
pub type C4TCR = crate::Reg<u32, _C4TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4TCR;
#[doc = "`read()` method returns [c4tcr::R](c4tcr::R) reader structure"]
impl crate::Readable for C4TCR {}
#[doc = "`write(|w| ..)` method takes [c4tcr::W](c4tcr::W) writer structure"]
impl crate::Writable for C4TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c4tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4bndtr](c4bndtr) module"]
pub type C4BNDTR = crate::Reg<u32, _C4BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4BNDTR;
#[doc = "`read()` method returns [c4bndtr::R](c4bndtr::R) reader structure"]
impl crate::Readable for C4BNDTR {}
#[doc = "`write(|w| ..)` method takes [c4bndtr::W](c4bndtr::W) writer structure"]
impl crate::Writable for C4BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c4bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4sar](c4sar) module"]
pub type C4SAR = crate::Reg<u32, _C4SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4SAR;
#[doc = "`read()` method returns [c4sar::R](c4sar::R) reader structure"]
impl crate::Readable for C4SAR {}
#[doc = "`write(|w| ..)` method takes [c4sar::W](c4sar::W) writer structure"]
impl crate::Writable for C4SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c4sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4dar](c4dar) module"]
pub type C4DAR = crate::Reg<u32, _C4DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4DAR;
#[doc = "`read()` method returns [c4dar::R](c4dar::R) reader structure"]
impl crate::Readable for C4DAR {}
#[doc = "`write(|w| ..)` method takes [c4dar::W](c4dar::W) writer structure"]
impl crate::Writable for C4DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c4dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4brur](c4brur) module"]
pub type C4BRUR = crate::Reg<u32, _C4BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4BRUR;
#[doc = "`read()` method returns [c4brur::R](c4brur::R) reader structure"]
impl crate::Readable for C4BRUR {}
#[doc = "`write(|w| ..)` method takes [c4brur::W](c4brur::W) writer structure"]
impl crate::Writable for C4BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c4brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4lar](c4lar) module"]
pub type C4LAR = crate::Reg<u32, _C4LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4LAR;
#[doc = "`read()` method returns [c4lar::R](c4lar::R) reader structure"]
impl crate::Readable for C4LAR {}
#[doc = "`write(|w| ..)` method takes [c4lar::W](c4lar::W) writer structure"]
impl crate::Writable for C4LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c4lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4tbr](c4tbr) module"]
pub type C4TBR = crate::Reg<u32, _C4TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4TBR;
#[doc = "`read()` method returns [c4tbr::R](c4tbr::R) reader structure"]
impl crate::Readable for C4TBR {}
#[doc = "`write(|w| ..)` method takes [c4tbr::W](c4tbr::W) writer structure"]
impl crate::Writable for C4TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c4tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4mar](c4mar) module"]
pub type C4MAR = crate::Reg<u32, _C4MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4MAR;
#[doc = "`read()` method returns [c4mar::R](c4mar::R) reader structure"]
impl crate::Readable for C4MAR {}
#[doc = "`write(|w| ..)` method takes [c4mar::W](c4mar::W) writer structure"]
impl crate::Writable for C4MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c4mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4mdr](c4mdr) module"]
pub type C4MDR = crate::Reg<u32, _C4MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4MDR;
#[doc = "`read()` method returns [c4mdr::R](c4mdr::R) reader structure"]
impl crate::Readable for C4MDR {}
#[doc = "`write(|w| ..)` method takes [c4mdr::W](c4mdr::W) writer structure"]
impl crate::Writable for C4MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c4mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5isr](c5isr) module"]
pub type C5ISR = crate::Reg<u32, _C5ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5ISR;
#[doc = "`read()` method returns [c5isr::R](c5isr::R) reader structure"]
impl crate::Readable for C5ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c5isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5ifcr](c5ifcr) module"]
pub type C5IFCR = crate::Reg<u32, _C5IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5IFCR;
#[doc = "`write(|w| ..)` method takes [c5ifcr::W](c5ifcr::W) writer structure"]
impl crate::Writable for C5IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c5ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5esr](c5esr) module"]
pub type C5ESR = crate::Reg<u32, _C5ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5ESR;
#[doc = "`read()` method returns [c5esr::R](c5esr::R) reader structure"]
impl crate::Readable for C5ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c5esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5cr](c5cr) module"]
pub type C5CR = crate::Reg<u32, _C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5CR;
#[doc = "`read()` method returns [c5cr::R](c5cr::R) reader structure"]
impl crate::Readable for C5CR {}
#[doc = "`write(|w| ..)` method takes [c5cr::W](c5cr::W) writer structure"]
impl crate::Writable for C5CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c5cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5tcr](c5tcr) module"]
pub type C5TCR = crate::Reg<u32, _C5TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5TCR;
#[doc = "`read()` method returns [c5tcr::R](c5tcr::R) reader structure"]
impl crate::Readable for C5TCR {}
#[doc = "`write(|w| ..)` method takes [c5tcr::W](c5tcr::W) writer structure"]
impl crate::Writable for C5TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c5tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5bndtr](c5bndtr) module"]
pub type C5BNDTR = crate::Reg<u32, _C5BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5BNDTR;
#[doc = "`read()` method returns [c5bndtr::R](c5bndtr::R) reader structure"]
impl crate::Readable for C5BNDTR {}
#[doc = "`write(|w| ..)` method takes [c5bndtr::W](c5bndtr::W) writer structure"]
impl crate::Writable for C5BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c5bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5sar](c5sar) module"]
pub type C5SAR = crate::Reg<u32, _C5SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5SAR;
#[doc = "`read()` method returns [c5sar::R](c5sar::R) reader structure"]
impl crate::Readable for C5SAR {}
#[doc = "`write(|w| ..)` method takes [c5sar::W](c5sar::W) writer structure"]
impl crate::Writable for C5SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c5sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5dar](c5dar) module"]
pub type C5DAR = crate::Reg<u32, _C5DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5DAR;
#[doc = "`read()` method returns [c5dar::R](c5dar::R) reader structure"]
impl crate::Readable for C5DAR {}
#[doc = "`write(|w| ..)` method takes [c5dar::W](c5dar::W) writer structure"]
impl crate::Writable for C5DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c5dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5brur](c5brur) module"]
pub type C5BRUR = crate::Reg<u32, _C5BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5BRUR;
#[doc = "`read()` method returns [c5brur::R](c5brur::R) reader structure"]
impl crate::Readable for C5BRUR {}
#[doc = "`write(|w| ..)` method takes [c5brur::W](c5brur::W) writer structure"]
impl crate::Writable for C5BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c5brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5lar](c5lar) module"]
pub type C5LAR = crate::Reg<u32, _C5LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5LAR;
#[doc = "`read()` method returns [c5lar::R](c5lar::R) reader structure"]
impl crate::Readable for C5LAR {}
#[doc = "`write(|w| ..)` method takes [c5lar::W](c5lar::W) writer structure"]
impl crate::Writable for C5LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c5lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5tbr](c5tbr) module"]
pub type C5TBR = crate::Reg<u32, _C5TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5TBR;
#[doc = "`read()` method returns [c5tbr::R](c5tbr::R) reader structure"]
impl crate::Readable for C5TBR {}
#[doc = "`write(|w| ..)` method takes [c5tbr::W](c5tbr::W) writer structure"]
impl crate::Writable for C5TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c5tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5mar](c5mar) module"]
pub type C5MAR = crate::Reg<u32, _C5MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5MAR;
#[doc = "`read()` method returns [c5mar::R](c5mar::R) reader structure"]
impl crate::Readable for C5MAR {}
#[doc = "`write(|w| ..)` method takes [c5mar::W](c5mar::W) writer structure"]
impl crate::Writable for C5MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c5mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5mdr](c5mdr) module"]
pub type C5MDR = crate::Reg<u32, _C5MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5MDR;
#[doc = "`read()` method returns [c5mdr::R](c5mdr::R) reader structure"]
impl crate::Readable for C5MDR {}
#[doc = "`write(|w| ..)` method takes [c5mdr::W](c5mdr::W) writer structure"]
impl crate::Writable for C5MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c5mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6isr](c6isr) module"]
pub type C6ISR = crate::Reg<u32, _C6ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6ISR;
#[doc = "`read()` method returns [c6isr::R](c6isr::R) reader structure"]
impl crate::Readable for C6ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c6isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6ifcr](c6ifcr) module"]
pub type C6IFCR = crate::Reg<u32, _C6IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6IFCR;
#[doc = "`write(|w| ..)` method takes [c6ifcr::W](c6ifcr::W) writer structure"]
impl crate::Writable for C6IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c6ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6esr](c6esr) module"]
pub type C6ESR = crate::Reg<u32, _C6ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6ESR;
#[doc = "`read()` method returns [c6esr::R](c6esr::R) reader structure"]
impl crate::Readable for C6ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c6esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6cr](c6cr) module"]
pub type C6CR = crate::Reg<u32, _C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6CR;
#[doc = "`read()` method returns [c6cr::R](c6cr::R) reader structure"]
impl crate::Readable for C6CR {}
#[doc = "`write(|w| ..)` method takes [c6cr::W](c6cr::W) writer structure"]
impl crate::Writable for C6CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c6cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6tcr](c6tcr) module"]
pub type C6TCR = crate::Reg<u32, _C6TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6TCR;
#[doc = "`read()` method returns [c6tcr::R](c6tcr::R) reader structure"]
impl crate::Readable for C6TCR {}
#[doc = "`write(|w| ..)` method takes [c6tcr::W](c6tcr::W) writer structure"]
impl crate::Writable for C6TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c6tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6bndtr](c6bndtr) module"]
pub type C6BNDTR = crate::Reg<u32, _C6BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6BNDTR;
#[doc = "`read()` method returns [c6bndtr::R](c6bndtr::R) reader structure"]
impl crate::Readable for C6BNDTR {}
#[doc = "`write(|w| ..)` method takes [c6bndtr::W](c6bndtr::W) writer structure"]
impl crate::Writable for C6BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c6bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6sar](c6sar) module"]
pub type C6SAR = crate::Reg<u32, _C6SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6SAR;
#[doc = "`read()` method returns [c6sar::R](c6sar::R) reader structure"]
impl crate::Readable for C6SAR {}
#[doc = "`write(|w| ..)` method takes [c6sar::W](c6sar::W) writer structure"]
impl crate::Writable for C6SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c6sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6dar](c6dar) module"]
pub type C6DAR = crate::Reg<u32, _C6DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6DAR;
#[doc = "`read()` method returns [c6dar::R](c6dar::R) reader structure"]
impl crate::Readable for C6DAR {}
#[doc = "`write(|w| ..)` method takes [c6dar::W](c6dar::W) writer structure"]
impl crate::Writable for C6DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c6dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6brur](c6brur) module"]
pub type C6BRUR = crate::Reg<u32, _C6BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6BRUR;
#[doc = "`read()` method returns [c6brur::R](c6brur::R) reader structure"]
impl crate::Readable for C6BRUR {}
#[doc = "`write(|w| ..)` method takes [c6brur::W](c6brur::W) writer structure"]
impl crate::Writable for C6BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c6brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6lar](c6lar) module"]
pub type C6LAR = crate::Reg<u32, _C6LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6LAR;
#[doc = "`read()` method returns [c6lar::R](c6lar::R) reader structure"]
impl crate::Readable for C6LAR {}
#[doc = "`write(|w| ..)` method takes [c6lar::W](c6lar::W) writer structure"]
impl crate::Writable for C6LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c6lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6tbr](c6tbr) module"]
pub type C6TBR = crate::Reg<u32, _C6TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6TBR;
#[doc = "`read()` method returns [c6tbr::R](c6tbr::R) reader structure"]
impl crate::Readable for C6TBR {}
#[doc = "`write(|w| ..)` method takes [c6tbr::W](c6tbr::W) writer structure"]
impl crate::Writable for C6TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c6tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6mar](c6mar) module"]
pub type C6MAR = crate::Reg<u32, _C6MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6MAR;
#[doc = "`read()` method returns [c6mar::R](c6mar::R) reader structure"]
impl crate::Readable for C6MAR {}
#[doc = "`write(|w| ..)` method takes [c6mar::W](c6mar::W) writer structure"]
impl crate::Writable for C6MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c6mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6mdr](c6mdr) module"]
pub type C6MDR = crate::Reg<u32, _C6MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6MDR;
#[doc = "`read()` method returns [c6mdr::R](c6mdr::R) reader structure"]
impl crate::Readable for C6MDR {}
#[doc = "`write(|w| ..)` method takes [c6mdr::W](c6mdr::W) writer structure"]
impl crate::Writable for C6MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c6mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7isr](c7isr) module"]
pub type C7ISR = crate::Reg<u32, _C7ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7ISR;
#[doc = "`read()` method returns [c7isr::R](c7isr::R) reader structure"]
impl crate::Readable for C7ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c7isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7ifcr](c7ifcr) module"]
pub type C7IFCR = crate::Reg<u32, _C7IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7IFCR;
#[doc = "`write(|w| ..)` method takes [c7ifcr::W](c7ifcr::W) writer structure"]
impl crate::Writable for C7IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c7ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7esr](c7esr) module"]
pub type C7ESR = crate::Reg<u32, _C7ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7ESR;
#[doc = "`read()` method returns [c7esr::R](c7esr::R) reader structure"]
impl crate::Readable for C7ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c7esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7cr](c7cr) module"]
pub type C7CR = crate::Reg<u32, _C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7CR;
#[doc = "`read()` method returns [c7cr::R](c7cr::R) reader structure"]
impl crate::Readable for C7CR {}
#[doc = "`write(|w| ..)` method takes [c7cr::W](c7cr::W) writer structure"]
impl crate::Writable for C7CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c7cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7tcr](c7tcr) module"]
pub type C7TCR = crate::Reg<u32, _C7TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7TCR;
#[doc = "`read()` method returns [c7tcr::R](c7tcr::R) reader structure"]
impl crate::Readable for C7TCR {}
#[doc = "`write(|w| ..)` method takes [c7tcr::W](c7tcr::W) writer structure"]
impl crate::Writable for C7TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c7tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7bndtr](c7bndtr) module"]
pub type C7BNDTR = crate::Reg<u32, _C7BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7BNDTR;
#[doc = "`read()` method returns [c7bndtr::R](c7bndtr::R) reader structure"]
impl crate::Readable for C7BNDTR {}
#[doc = "`write(|w| ..)` method takes [c7bndtr::W](c7bndtr::W) writer structure"]
impl crate::Writable for C7BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c7bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7sar](c7sar) module"]
pub type C7SAR = crate::Reg<u32, _C7SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7SAR;
#[doc = "`read()` method returns [c7sar::R](c7sar::R) reader structure"]
impl crate::Readable for C7SAR {}
#[doc = "`write(|w| ..)` method takes [c7sar::W](c7sar::W) writer structure"]
impl crate::Writable for C7SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c7sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7dar](c7dar) module"]
pub type C7DAR = crate::Reg<u32, _C7DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7DAR;
#[doc = "`read()` method returns [c7dar::R](c7dar::R) reader structure"]
impl crate::Readable for C7DAR {}
#[doc = "`write(|w| ..)` method takes [c7dar::W](c7dar::W) writer structure"]
impl crate::Writable for C7DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c7dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7brur](c7brur) module"]
pub type C7BRUR = crate::Reg<u32, _C7BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7BRUR;
#[doc = "`read()` method returns [c7brur::R](c7brur::R) reader structure"]
impl crate::Readable for C7BRUR {}
#[doc = "`write(|w| ..)` method takes [c7brur::W](c7brur::W) writer structure"]
impl crate::Writable for C7BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c7brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7lar](c7lar) module"]
pub type C7LAR = crate::Reg<u32, _C7LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7LAR;
#[doc = "`read()` method returns [c7lar::R](c7lar::R) reader structure"]
impl crate::Readable for C7LAR {}
#[doc = "`write(|w| ..)` method takes [c7lar::W](c7lar::W) writer structure"]
impl crate::Writable for C7LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c7lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7tbr](c7tbr) module"]
pub type C7TBR = crate::Reg<u32, _C7TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7TBR;
#[doc = "`read()` method returns [c7tbr::R](c7tbr::R) reader structure"]
impl crate::Readable for C7TBR {}
#[doc = "`write(|w| ..)` method takes [c7tbr::W](c7tbr::W) writer structure"]
impl crate::Writable for C7TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c7tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7mar](c7mar) module"]
pub type C7MAR = crate::Reg<u32, _C7MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7MAR;
#[doc = "`read()` method returns [c7mar::R](c7mar::R) reader structure"]
impl crate::Readable for C7MAR {}
#[doc = "`write(|w| ..)` method takes [c7mar::W](c7mar::W) writer structure"]
impl crate::Writable for C7MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c7mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7mdr](c7mdr) module"]
pub type C7MDR = crate::Reg<u32, _C7MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7MDR;
#[doc = "`read()` method returns [c7mdr::R](c7mdr::R) reader structure"]
impl crate::Readable for C7MDR {}
#[doc = "`write(|w| ..)` method takes [c7mdr::W](c7mdr::W) writer structure"]
impl crate::Writable for C7MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c7mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8isr](c8isr) module"]
pub type C8ISR = crate::Reg<u32, _C8ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8ISR;
#[doc = "`read()` method returns [c8isr::R](c8isr::R) reader structure"]
impl crate::Readable for C8ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c8isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8ifcr](c8ifcr) module"]
pub type C8IFCR = crate::Reg<u32, _C8IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8IFCR;
#[doc = "`write(|w| ..)` method takes [c8ifcr::W](c8ifcr::W) writer structure"]
impl crate::Writable for C8IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c8ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8esr](c8esr) module"]
pub type C8ESR = crate::Reg<u32, _C8ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8ESR;
#[doc = "`read()` method returns [c8esr::R](c8esr::R) reader structure"]
impl crate::Readable for C8ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c8esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8cr](c8cr) module"]
pub type C8CR = crate::Reg<u32, _C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8CR;
#[doc = "`read()` method returns [c8cr::R](c8cr::R) reader structure"]
impl crate::Readable for C8CR {}
#[doc = "`write(|w| ..)` method takes [c8cr::W](c8cr::W) writer structure"]
impl crate::Writable for C8CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c8cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8tcr](c8tcr) module"]
pub type C8TCR = crate::Reg<u32, _C8TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8TCR;
#[doc = "`read()` method returns [c8tcr::R](c8tcr::R) reader structure"]
impl crate::Readable for C8TCR {}
#[doc = "`write(|w| ..)` method takes [c8tcr::W](c8tcr::W) writer structure"]
impl crate::Writable for C8TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c8tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8bndtr](c8bndtr) module"]
pub type C8BNDTR = crate::Reg<u32, _C8BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8BNDTR;
#[doc = "`read()` method returns [c8bndtr::R](c8bndtr::R) reader structure"]
impl crate::Readable for C8BNDTR {}
#[doc = "`write(|w| ..)` method takes [c8bndtr::W](c8bndtr::W) writer structure"]
impl crate::Writable for C8BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c8bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8sar](c8sar) module"]
pub type C8SAR = crate::Reg<u32, _C8SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8SAR;
#[doc = "`read()` method returns [c8sar::R](c8sar::R) reader structure"]
impl crate::Readable for C8SAR {}
#[doc = "`write(|w| ..)` method takes [c8sar::W](c8sar::W) writer structure"]
impl crate::Writable for C8SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c8sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8dar](c8dar) module"]
pub type C8DAR = crate::Reg<u32, _C8DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8DAR;
#[doc = "`read()` method returns [c8dar::R](c8dar::R) reader structure"]
impl crate::Readable for C8DAR {}
#[doc = "`write(|w| ..)` method takes [c8dar::W](c8dar::W) writer structure"]
impl crate::Writable for C8DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c8dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8brur](c8brur) module"]
pub type C8BRUR = crate::Reg<u32, _C8BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8BRUR;
#[doc = "`read()` method returns [c8brur::R](c8brur::R) reader structure"]
impl crate::Readable for C8BRUR {}
#[doc = "`write(|w| ..)` method takes [c8brur::W](c8brur::W) writer structure"]
impl crate::Writable for C8BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c8brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8lar](c8lar) module"]
pub type C8LAR = crate::Reg<u32, _C8LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8LAR;
#[doc = "`read()` method returns [c8lar::R](c8lar::R) reader structure"]
impl crate::Readable for C8LAR {}
#[doc = "`write(|w| ..)` method takes [c8lar::W](c8lar::W) writer structure"]
impl crate::Writable for C8LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c8lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8tbr](c8tbr) module"]
pub type C8TBR = crate::Reg<u32, _C8TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8TBR;
#[doc = "`read()` method returns [c8tbr::R](c8tbr::R) reader structure"]
impl crate::Readable for C8TBR {}
#[doc = "`write(|w| ..)` method takes [c8tbr::W](c8tbr::W) writer structure"]
impl crate::Writable for C8TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c8tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8mar](c8mar) module"]
pub type C8MAR = crate::Reg<u32, _C8MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8MAR;
#[doc = "`read()` method returns [c8mar::R](c8mar::R) reader structure"]
impl crate::Readable for C8MAR {}
#[doc = "`write(|w| ..)` method takes [c8mar::W](c8mar::W) writer structure"]
impl crate::Writable for C8MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c8mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c8mdr](c8mdr) module"]
pub type C8MDR = crate::Reg<u32, _C8MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8MDR;
#[doc = "`read()` method returns [c8mdr::R](c8mdr::R) reader structure"]
impl crate::Readable for C8MDR {}
#[doc = "`write(|w| ..)` method takes [c8mdr::W](c8mdr::W) writer structure"]
impl crate::Writable for C8MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c8mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9isr](c9isr) module"]
pub type C9ISR = crate::Reg<u32, _C9ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9ISR;
#[doc = "`read()` method returns [c9isr::R](c9isr::R) reader structure"]
impl crate::Readable for C9ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c9isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9ifcr](c9ifcr) module"]
pub type C9IFCR = crate::Reg<u32, _C9IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9IFCR;
#[doc = "`write(|w| ..)` method takes [c9ifcr::W](c9ifcr::W) writer structure"]
impl crate::Writable for C9IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c9ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9esr](c9esr) module"]
pub type C9ESR = crate::Reg<u32, _C9ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9ESR;
#[doc = "`read()` method returns [c9esr::R](c9esr::R) reader structure"]
impl crate::Readable for C9ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c9esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9cr](c9cr) module"]
pub type C9CR = crate::Reg<u32, _C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9CR;
#[doc = "`read()` method returns [c9cr::R](c9cr::R) reader structure"]
impl crate::Readable for C9CR {}
#[doc = "`write(|w| ..)` method takes [c9cr::W](c9cr::W) writer structure"]
impl crate::Writable for C9CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c9cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9tcr](c9tcr) module"]
pub type C9TCR = crate::Reg<u32, _C9TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9TCR;
#[doc = "`read()` method returns [c9tcr::R](c9tcr::R) reader structure"]
impl crate::Readable for C9TCR {}
#[doc = "`write(|w| ..)` method takes [c9tcr::W](c9tcr::W) writer structure"]
impl crate::Writable for C9TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c9tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9bndtr](c9bndtr) module"]
pub type C9BNDTR = crate::Reg<u32, _C9BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9BNDTR;
#[doc = "`read()` method returns [c9bndtr::R](c9bndtr::R) reader structure"]
impl crate::Readable for C9BNDTR {}
#[doc = "`write(|w| ..)` method takes [c9bndtr::W](c9bndtr::W) writer structure"]
impl crate::Writable for C9BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c9bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9sar](c9sar) module"]
pub type C9SAR = crate::Reg<u32, _C9SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9SAR;
#[doc = "`read()` method returns [c9sar::R](c9sar::R) reader structure"]
impl crate::Readable for C9SAR {}
#[doc = "`write(|w| ..)` method takes [c9sar::W](c9sar::W) writer structure"]
impl crate::Writable for C9SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c9sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9dar](c9dar) module"]
pub type C9DAR = crate::Reg<u32, _C9DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9DAR;
#[doc = "`read()` method returns [c9dar::R](c9dar::R) reader structure"]
impl crate::Readable for C9DAR {}
#[doc = "`write(|w| ..)` method takes [c9dar::W](c9dar::W) writer structure"]
impl crate::Writable for C9DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c9dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9brur](c9brur) module"]
pub type C9BRUR = crate::Reg<u32, _C9BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9BRUR;
#[doc = "`read()` method returns [c9brur::R](c9brur::R) reader structure"]
impl crate::Readable for C9BRUR {}
#[doc = "`write(|w| ..)` method takes [c9brur::W](c9brur::W) writer structure"]
impl crate::Writable for C9BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c9brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9lar](c9lar) module"]
pub type C9LAR = crate::Reg<u32, _C9LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9LAR;
#[doc = "`read()` method returns [c9lar::R](c9lar::R) reader structure"]
impl crate::Readable for C9LAR {}
#[doc = "`write(|w| ..)` method takes [c9lar::W](c9lar::W) writer structure"]
impl crate::Writable for C9LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c9lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9tbr](c9tbr) module"]
pub type C9TBR = crate::Reg<u32, _C9TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9TBR;
#[doc = "`read()` method returns [c9tbr::R](c9tbr::R) reader structure"]
impl crate::Readable for C9TBR {}
#[doc = "`write(|w| ..)` method takes [c9tbr::W](c9tbr::W) writer structure"]
impl crate::Writable for C9TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c9tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9mar](c9mar) module"]
pub type C9MAR = crate::Reg<u32, _C9MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9MAR;
#[doc = "`read()` method returns [c9mar::R](c9mar::R) reader structure"]
impl crate::Readable for C9MAR {}
#[doc = "`write(|w| ..)` method takes [c9mar::W](c9mar::W) writer structure"]
impl crate::Writable for C9MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c9mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c9mdr](c9mdr) module"]
pub type C9MDR = crate::Reg<u32, _C9MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9MDR;
#[doc = "`read()` method returns [c9mdr::R](c9mdr::R) reader structure"]
impl crate::Readable for C9MDR {}
#[doc = "`write(|w| ..)` method takes [c9mdr::W](c9mdr::W) writer structure"]
impl crate::Writable for C9MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c9mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10isr](c10isr) module"]
pub type C10ISR = crate::Reg<u32, _C10ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10ISR;
#[doc = "`read()` method returns [c10isr::R](c10isr::R) reader structure"]
impl crate::Readable for C10ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c10isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10ifcr](c10ifcr) module"]
pub type C10IFCR = crate::Reg<u32, _C10IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10IFCR;
#[doc = "`write(|w| ..)` method takes [c10ifcr::W](c10ifcr::W) writer structure"]
impl crate::Writable for C10IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c10ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10esr](c10esr) module"]
pub type C10ESR = crate::Reg<u32, _C10ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10ESR;
#[doc = "`read()` method returns [c10esr::R](c10esr::R) reader structure"]
impl crate::Readable for C10ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c10esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10cr](c10cr) module"]
pub type C10CR = crate::Reg<u32, _C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10CR;
#[doc = "`read()` method returns [c10cr::R](c10cr::R) reader structure"]
impl crate::Readable for C10CR {}
#[doc = "`write(|w| ..)` method takes [c10cr::W](c10cr::W) writer structure"]
impl crate::Writable for C10CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c10cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10tcr](c10tcr) module"]
pub type C10TCR = crate::Reg<u32, _C10TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10TCR;
#[doc = "`read()` method returns [c10tcr::R](c10tcr::R) reader structure"]
impl crate::Readable for C10TCR {}
#[doc = "`write(|w| ..)` method takes [c10tcr::W](c10tcr::W) writer structure"]
impl crate::Writable for C10TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c10tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10bndtr](c10bndtr) module"]
pub type C10BNDTR = crate::Reg<u32, _C10BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10BNDTR;
#[doc = "`read()` method returns [c10bndtr::R](c10bndtr::R) reader structure"]
impl crate::Readable for C10BNDTR {}
#[doc = "`write(|w| ..)` method takes [c10bndtr::W](c10bndtr::W) writer structure"]
impl crate::Writable for C10BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c10bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10sar](c10sar) module"]
pub type C10SAR = crate::Reg<u32, _C10SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10SAR;
#[doc = "`read()` method returns [c10sar::R](c10sar::R) reader structure"]
impl crate::Readable for C10SAR {}
#[doc = "`write(|w| ..)` method takes [c10sar::W](c10sar::W) writer structure"]
impl crate::Writable for C10SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c10sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10dar](c10dar) module"]
pub type C10DAR = crate::Reg<u32, _C10DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10DAR;
#[doc = "`read()` method returns [c10dar::R](c10dar::R) reader structure"]
impl crate::Readable for C10DAR {}
#[doc = "`write(|w| ..)` method takes [c10dar::W](c10dar::W) writer structure"]
impl crate::Writable for C10DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c10dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10brur](c10brur) module"]
pub type C10BRUR = crate::Reg<u32, _C10BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10BRUR;
#[doc = "`read()` method returns [c10brur::R](c10brur::R) reader structure"]
impl crate::Readable for C10BRUR {}
#[doc = "`write(|w| ..)` method takes [c10brur::W](c10brur::W) writer structure"]
impl crate::Writable for C10BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c10brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10lar](c10lar) module"]
pub type C10LAR = crate::Reg<u32, _C10LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10LAR;
#[doc = "`read()` method returns [c10lar::R](c10lar::R) reader structure"]
impl crate::Readable for C10LAR {}
#[doc = "`write(|w| ..)` method takes [c10lar::W](c10lar::W) writer structure"]
impl crate::Writable for C10LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c10lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10tbr](c10tbr) module"]
pub type C10TBR = crate::Reg<u32, _C10TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10TBR;
#[doc = "`read()` method returns [c10tbr::R](c10tbr::R) reader structure"]
impl crate::Readable for C10TBR {}
#[doc = "`write(|w| ..)` method takes [c10tbr::W](c10tbr::W) writer structure"]
impl crate::Writable for C10TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c10tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10mar](c10mar) module"]
pub type C10MAR = crate::Reg<u32, _C10MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10MAR;
#[doc = "`read()` method returns [c10mar::R](c10mar::R) reader structure"]
impl crate::Readable for C10MAR {}
#[doc = "`write(|w| ..)` method takes [c10mar::W](c10mar::W) writer structure"]
impl crate::Writable for C10MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c10mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c10mdr](c10mdr) module"]
pub type C10MDR = crate::Reg<u32, _C10MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10MDR;
#[doc = "`read()` method returns [c10mdr::R](c10mdr::R) reader structure"]
impl crate::Readable for C10MDR {}
#[doc = "`write(|w| ..)` method takes [c10mdr::W](c10mdr::W) writer structure"]
impl crate::Writable for C10MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c10mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11isr](c11isr) module"]
pub type C11ISR = crate::Reg<u32, _C11ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11ISR;
#[doc = "`read()` method returns [c11isr::R](c11isr::R) reader structure"]
impl crate::Readable for C11ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c11isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11ifcr](c11ifcr) module"]
pub type C11IFCR = crate::Reg<u32, _C11IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11IFCR;
#[doc = "`write(|w| ..)` method takes [c11ifcr::W](c11ifcr::W) writer structure"]
impl crate::Writable for C11IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c11ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11esr](c11esr) module"]
pub type C11ESR = crate::Reg<u32, _C11ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11ESR;
#[doc = "`read()` method returns [c11esr::R](c11esr::R) reader structure"]
impl crate::Readable for C11ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c11esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11cr](c11cr) module"]
pub type C11CR = crate::Reg<u32, _C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11CR;
#[doc = "`read()` method returns [c11cr::R](c11cr::R) reader structure"]
impl crate::Readable for C11CR {}
#[doc = "`write(|w| ..)` method takes [c11cr::W](c11cr::W) writer structure"]
impl crate::Writable for C11CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c11cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11tcr](c11tcr) module"]
pub type C11TCR = crate::Reg<u32, _C11TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11TCR;
#[doc = "`read()` method returns [c11tcr::R](c11tcr::R) reader structure"]
impl crate::Readable for C11TCR {}
#[doc = "`write(|w| ..)` method takes [c11tcr::W](c11tcr::W) writer structure"]
impl crate::Writable for C11TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c11tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11bndtr](c11bndtr) module"]
pub type C11BNDTR = crate::Reg<u32, _C11BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11BNDTR;
#[doc = "`read()` method returns [c11bndtr::R](c11bndtr::R) reader structure"]
impl crate::Readable for C11BNDTR {}
#[doc = "`write(|w| ..)` method takes [c11bndtr::W](c11bndtr::W) writer structure"]
impl crate::Writable for C11BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c11bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11sar](c11sar) module"]
pub type C11SAR = crate::Reg<u32, _C11SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11SAR;
#[doc = "`read()` method returns [c11sar::R](c11sar::R) reader structure"]
impl crate::Readable for C11SAR {}
#[doc = "`write(|w| ..)` method takes [c11sar::W](c11sar::W) writer structure"]
impl crate::Writable for C11SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c11sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11dar](c11dar) module"]
pub type C11DAR = crate::Reg<u32, _C11DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11DAR;
#[doc = "`read()` method returns [c11dar::R](c11dar::R) reader structure"]
impl crate::Readable for C11DAR {}
#[doc = "`write(|w| ..)` method takes [c11dar::W](c11dar::W) writer structure"]
impl crate::Writable for C11DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c11dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11brur](c11brur) module"]
pub type C11BRUR = crate::Reg<u32, _C11BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11BRUR;
#[doc = "`read()` method returns [c11brur::R](c11brur::R) reader structure"]
impl crate::Readable for C11BRUR {}
#[doc = "`write(|w| ..)` method takes [c11brur::W](c11brur::W) writer structure"]
impl crate::Writable for C11BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c11brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11lar](c11lar) module"]
pub type C11LAR = crate::Reg<u32, _C11LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11LAR;
#[doc = "`read()` method returns [c11lar::R](c11lar::R) reader structure"]
impl crate::Readable for C11LAR {}
#[doc = "`write(|w| ..)` method takes [c11lar::W](c11lar::W) writer structure"]
impl crate::Writable for C11LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c11lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11tbr](c11tbr) module"]
pub type C11TBR = crate::Reg<u32, _C11TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11TBR;
#[doc = "`read()` method returns [c11tbr::R](c11tbr::R) reader structure"]
impl crate::Readable for C11TBR {}
#[doc = "`write(|w| ..)` method takes [c11tbr::W](c11tbr::W) writer structure"]
impl crate::Writable for C11TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c11tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11mar](c11mar) module"]
pub type C11MAR = crate::Reg<u32, _C11MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11MAR;
#[doc = "`read()` method returns [c11mar::R](c11mar::R) reader structure"]
impl crate::Readable for C11MAR {}
#[doc = "`write(|w| ..)` method takes [c11mar::W](c11mar::W) writer structure"]
impl crate::Writable for C11MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c11mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c11mdr](c11mdr) module"]
pub type C11MDR = crate::Reg<u32, _C11MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11MDR;
#[doc = "`read()` method returns [c11mdr::R](c11mdr::R) reader structure"]
impl crate::Readable for C11MDR {}
#[doc = "`write(|w| ..)` method takes [c11mdr::W](c11mdr::W) writer structure"]
impl crate::Writable for C11MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c11mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12isr](c12isr) module"]
pub type C12ISR = crate::Reg<u32, _C12ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12ISR;
#[doc = "`read()` method returns [c12isr::R](c12isr::R) reader structure"]
impl crate::Readable for C12ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c12isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12ifcr](c12ifcr) module"]
pub type C12IFCR = crate::Reg<u32, _C12IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12IFCR;
#[doc = "`write(|w| ..)` method takes [c12ifcr::W](c12ifcr::W) writer structure"]
impl crate::Writable for C12IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c12ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12esr](c12esr) module"]
pub type C12ESR = crate::Reg<u32, _C12ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12ESR;
#[doc = "`read()` method returns [c12esr::R](c12esr::R) reader structure"]
impl crate::Readable for C12ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c12esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12cr](c12cr) module"]
pub type C12CR = crate::Reg<u32, _C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12CR;
#[doc = "`read()` method returns [c12cr::R](c12cr::R) reader structure"]
impl crate::Readable for C12CR {}
#[doc = "`write(|w| ..)` method takes [c12cr::W](c12cr::W) writer structure"]
impl crate::Writable for C12CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c12cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12tcr](c12tcr) module"]
pub type C12TCR = crate::Reg<u32, _C12TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12TCR;
#[doc = "`read()` method returns [c12tcr::R](c12tcr::R) reader structure"]
impl crate::Readable for C12TCR {}
#[doc = "`write(|w| ..)` method takes [c12tcr::W](c12tcr::W) writer structure"]
impl crate::Writable for C12TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c12tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12bndtr](c12bndtr) module"]
pub type C12BNDTR = crate::Reg<u32, _C12BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12BNDTR;
#[doc = "`read()` method returns [c12bndtr::R](c12bndtr::R) reader structure"]
impl crate::Readable for C12BNDTR {}
#[doc = "`write(|w| ..)` method takes [c12bndtr::W](c12bndtr::W) writer structure"]
impl crate::Writable for C12BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c12bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12sar](c12sar) module"]
pub type C12SAR = crate::Reg<u32, _C12SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12SAR;
#[doc = "`read()` method returns [c12sar::R](c12sar::R) reader structure"]
impl crate::Readable for C12SAR {}
#[doc = "`write(|w| ..)` method takes [c12sar::W](c12sar::W) writer structure"]
impl crate::Writable for C12SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c12sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12dar](c12dar) module"]
pub type C12DAR = crate::Reg<u32, _C12DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12DAR;
#[doc = "`read()` method returns [c12dar::R](c12dar::R) reader structure"]
impl crate::Readable for C12DAR {}
#[doc = "`write(|w| ..)` method takes [c12dar::W](c12dar::W) writer structure"]
impl crate::Writable for C12DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c12dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12brur](c12brur) module"]
pub type C12BRUR = crate::Reg<u32, _C12BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12BRUR;
#[doc = "`read()` method returns [c12brur::R](c12brur::R) reader structure"]
impl crate::Readable for C12BRUR {}
#[doc = "`write(|w| ..)` method takes [c12brur::W](c12brur::W) writer structure"]
impl crate::Writable for C12BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c12brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12lar](c12lar) module"]
pub type C12LAR = crate::Reg<u32, _C12LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12LAR;
#[doc = "`read()` method returns [c12lar::R](c12lar::R) reader structure"]
impl crate::Readable for C12LAR {}
#[doc = "`write(|w| ..)` method takes [c12lar::W](c12lar::W) writer structure"]
impl crate::Writable for C12LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c12lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12tbr](c12tbr) module"]
pub type C12TBR = crate::Reg<u32, _C12TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12TBR;
#[doc = "`read()` method returns [c12tbr::R](c12tbr::R) reader structure"]
impl crate::Readable for C12TBR {}
#[doc = "`write(|w| ..)` method takes [c12tbr::W](c12tbr::W) writer structure"]
impl crate::Writable for C12TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c12tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12mar](c12mar) module"]
pub type C12MAR = crate::Reg<u32, _C12MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12MAR;
#[doc = "`read()` method returns [c12mar::R](c12mar::R) reader structure"]
impl crate::Readable for C12MAR {}
#[doc = "`write(|w| ..)` method takes [c12mar::W](c12mar::W) writer structure"]
impl crate::Writable for C12MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c12mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12mdr](c12mdr) module"]
pub type C12MDR = crate::Reg<u32, _C12MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12MDR;
#[doc = "`read()` method returns [c12mdr::R](c12mdr::R) reader structure"]
impl crate::Readable for C12MDR {}
#[doc = "`write(|w| ..)` method takes [c12mdr::W](c12mdr::W) writer structure"]
impl crate::Writable for C12MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c12mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13isr](c13isr) module"]
pub type C13ISR = crate::Reg<u32, _C13ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13ISR;
#[doc = "`read()` method returns [c13isr::R](c13isr::R) reader structure"]
impl crate::Readable for C13ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c13isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13ifcr](c13ifcr) module"]
pub type C13IFCR = crate::Reg<u32, _C13IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13IFCR;
#[doc = "`write(|w| ..)` method takes [c13ifcr::W](c13ifcr::W) writer structure"]
impl crate::Writable for C13IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c13ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13esr](c13esr) module"]
pub type C13ESR = crate::Reg<u32, _C13ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13ESR;
#[doc = "`read()` method returns [c13esr::R](c13esr::R) reader structure"]
impl crate::Readable for C13ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c13esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13cr](c13cr) module"]
pub type C13CR = crate::Reg<u32, _C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13CR;
#[doc = "`read()` method returns [c13cr::R](c13cr::R) reader structure"]
impl crate::Readable for C13CR {}
#[doc = "`write(|w| ..)` method takes [c13cr::W](c13cr::W) writer structure"]
impl crate::Writable for C13CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c13cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13tcr](c13tcr) module"]
pub type C13TCR = crate::Reg<u32, _C13TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13TCR;
#[doc = "`read()` method returns [c13tcr::R](c13tcr::R) reader structure"]
impl crate::Readable for C13TCR {}
#[doc = "`write(|w| ..)` method takes [c13tcr::W](c13tcr::W) writer structure"]
impl crate::Writable for C13TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c13tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13bndtr](c13bndtr) module"]
pub type C13BNDTR = crate::Reg<u32, _C13BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13BNDTR;
#[doc = "`read()` method returns [c13bndtr::R](c13bndtr::R) reader structure"]
impl crate::Readable for C13BNDTR {}
#[doc = "`write(|w| ..)` method takes [c13bndtr::W](c13bndtr::W) writer structure"]
impl crate::Writable for C13BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c13bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13sar](c13sar) module"]
pub type C13SAR = crate::Reg<u32, _C13SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13SAR;
#[doc = "`read()` method returns [c13sar::R](c13sar::R) reader structure"]
impl crate::Readable for C13SAR {}
#[doc = "`write(|w| ..)` method takes [c13sar::W](c13sar::W) writer structure"]
impl crate::Writable for C13SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c13sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13dar](c13dar) module"]
pub type C13DAR = crate::Reg<u32, _C13DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13DAR;
#[doc = "`read()` method returns [c13dar::R](c13dar::R) reader structure"]
impl crate::Readable for C13DAR {}
#[doc = "`write(|w| ..)` method takes [c13dar::W](c13dar::W) writer structure"]
impl crate::Writable for C13DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c13dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13brur](c13brur) module"]
pub type C13BRUR = crate::Reg<u32, _C13BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13BRUR;
#[doc = "`read()` method returns [c13brur::R](c13brur::R) reader structure"]
impl crate::Readable for C13BRUR {}
#[doc = "`write(|w| ..)` method takes [c13brur::W](c13brur::W) writer structure"]
impl crate::Writable for C13BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c13brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13lar](c13lar) module"]
pub type C13LAR = crate::Reg<u32, _C13LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13LAR;
#[doc = "`read()` method returns [c13lar::R](c13lar::R) reader structure"]
impl crate::Readable for C13LAR {}
#[doc = "`write(|w| ..)` method takes [c13lar::W](c13lar::W) writer structure"]
impl crate::Writable for C13LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c13lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13tbr](c13tbr) module"]
pub type C13TBR = crate::Reg<u32, _C13TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13TBR;
#[doc = "`read()` method returns [c13tbr::R](c13tbr::R) reader structure"]
impl crate::Readable for C13TBR {}
#[doc = "`write(|w| ..)` method takes [c13tbr::W](c13tbr::W) writer structure"]
impl crate::Writable for C13TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c13tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13mar](c13mar) module"]
pub type C13MAR = crate::Reg<u32, _C13MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13MAR;
#[doc = "`read()` method returns [c13mar::R](c13mar::R) reader structure"]
impl crate::Readable for C13MAR {}
#[doc = "`write(|w| ..)` method takes [c13mar::W](c13mar::W) writer structure"]
impl crate::Writable for C13MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c13mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c13mdr](c13mdr) module"]
pub type C13MDR = crate::Reg<u32, _C13MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13MDR;
#[doc = "`read()` method returns [c13mdr::R](c13mdr::R) reader structure"]
impl crate::Readable for C13MDR {}
#[doc = "`write(|w| ..)` method takes [c13mdr::W](c13mdr::W) writer structure"]
impl crate::Writable for C13MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c13mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14isr](c14isr) module"]
pub type C14ISR = crate::Reg<u32, _C14ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14ISR;
#[doc = "`read()` method returns [c14isr::R](c14isr::R) reader structure"]
impl crate::Readable for C14ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c14isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14ifcr](c14ifcr) module"]
pub type C14IFCR = crate::Reg<u32, _C14IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14IFCR;
#[doc = "`write(|w| ..)` method takes [c14ifcr::W](c14ifcr::W) writer structure"]
impl crate::Writable for C14IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c14ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14esr](c14esr) module"]
pub type C14ESR = crate::Reg<u32, _C14ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14ESR;
#[doc = "`read()` method returns [c14esr::R](c14esr::R) reader structure"]
impl crate::Readable for C14ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c14esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14cr](c14cr) module"]
pub type C14CR = crate::Reg<u32, _C14CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14CR;
#[doc = "`read()` method returns [c14cr::R](c14cr::R) reader structure"]
impl crate::Readable for C14CR {}
#[doc = "`write(|w| ..)` method takes [c14cr::W](c14cr::W) writer structure"]
impl crate::Writable for C14CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c14cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14tcr](c14tcr) module"]
pub type C14TCR = crate::Reg<u32, _C14TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14TCR;
#[doc = "`read()` method returns [c14tcr::R](c14tcr::R) reader structure"]
impl crate::Readable for C14TCR {}
#[doc = "`write(|w| ..)` method takes [c14tcr::W](c14tcr::W) writer structure"]
impl crate::Writable for C14TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c14tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14bndtr](c14bndtr) module"]
pub type C14BNDTR = crate::Reg<u32, _C14BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14BNDTR;
#[doc = "`read()` method returns [c14bndtr::R](c14bndtr::R) reader structure"]
impl crate::Readable for C14BNDTR {}
#[doc = "`write(|w| ..)` method takes [c14bndtr::W](c14bndtr::W) writer structure"]
impl crate::Writable for C14BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c14bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14sar](c14sar) module"]
pub type C14SAR = crate::Reg<u32, _C14SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14SAR;
#[doc = "`read()` method returns [c14sar::R](c14sar::R) reader structure"]
impl crate::Readable for C14SAR {}
#[doc = "`write(|w| ..)` method takes [c14sar::W](c14sar::W) writer structure"]
impl crate::Writable for C14SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c14sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14dar](c14dar) module"]
pub type C14DAR = crate::Reg<u32, _C14DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14DAR;
#[doc = "`read()` method returns [c14dar::R](c14dar::R) reader structure"]
impl crate::Readable for C14DAR {}
#[doc = "`write(|w| ..)` method takes [c14dar::W](c14dar::W) writer structure"]
impl crate::Writable for C14DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c14dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14brur](c14brur) module"]
pub type C14BRUR = crate::Reg<u32, _C14BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14BRUR;
#[doc = "`read()` method returns [c14brur::R](c14brur::R) reader structure"]
impl crate::Readable for C14BRUR {}
#[doc = "`write(|w| ..)` method takes [c14brur::W](c14brur::W) writer structure"]
impl crate::Writable for C14BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c14brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14lar](c14lar) module"]
pub type C14LAR = crate::Reg<u32, _C14LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14LAR;
#[doc = "`read()` method returns [c14lar::R](c14lar::R) reader structure"]
impl crate::Readable for C14LAR {}
#[doc = "`write(|w| ..)` method takes [c14lar::W](c14lar::W) writer structure"]
impl crate::Writable for C14LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c14lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14tbr](c14tbr) module"]
pub type C14TBR = crate::Reg<u32, _C14TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14TBR;
#[doc = "`read()` method returns [c14tbr::R](c14tbr::R) reader structure"]
impl crate::Readable for C14TBR {}
#[doc = "`write(|w| ..)` method takes [c14tbr::W](c14tbr::W) writer structure"]
impl crate::Writable for C14TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c14tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14mar](c14mar) module"]
pub type C14MAR = crate::Reg<u32, _C14MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14MAR;
#[doc = "`read()` method returns [c14mar::R](c14mar::R) reader structure"]
impl crate::Readable for C14MAR {}
#[doc = "`write(|w| ..)` method takes [c14mar::W](c14mar::W) writer structure"]
impl crate::Writable for C14MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c14mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c14mdr](c14mdr) module"]
pub type C14MDR = crate::Reg<u32, _C14MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C14MDR;
#[doc = "`read()` method returns [c14mdr::R](c14mdr::R) reader structure"]
impl crate::Readable for C14MDR {}
#[doc = "`write(|w| ..)` method takes [c14mdr::W](c14mdr::W) writer structure"]
impl crate::Writable for C14MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c14mdr;
#[doc = "MDMA channel x interrupt/status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15isr](c15isr) module"]
pub type C15ISR = crate::Reg<u32, _C15ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15ISR;
#[doc = "`read()` method returns [c15isr::R](c15isr::R) reader structure"]
impl crate::Readable for C15ISR {}
#[doc = "MDMA channel x interrupt/status register"]
pub mod c15isr;
#[doc = "MDMA channel x interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15ifcr](c15ifcr) module"]
pub type C15IFCR = crate::Reg<u32, _C15IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15IFCR;
#[doc = "`write(|w| ..)` method takes [c15ifcr::W](c15ifcr::W) writer structure"]
impl crate::Writable for C15IFCR {}
#[doc = "MDMA channel x interrupt flag clear register"]
pub mod c15ifcr;
#[doc = "MDMA Channel x error status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15esr](c15esr) module"]
pub type C15ESR = crate::Reg<u32, _C15ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15ESR;
#[doc = "`read()` method returns [c15esr::R](c15esr::R) reader structure"]
impl crate::Readable for C15ESR {}
#[doc = "MDMA Channel x error status register"]
pub mod c15esr;
#[doc = "This register is used to control the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15cr](c15cr) module"]
pub type C15CR = crate::Reg<u32, _C15CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15CR;
#[doc = "`read()` method returns [c15cr::R](c15cr::R) reader structure"]
impl crate::Readable for C15CR {}
#[doc = "`write(|w| ..)` method takes [c15cr::W](c15cr::W) writer structure"]
impl crate::Writable for C15CR {}
#[doc = "This register is used to control the concerned channel."]
pub mod c15cr;
#[doc = "This register is used to configure the concerned channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15tcr](c15tcr) module"]
pub type C15TCR = crate::Reg<u32, _C15TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15TCR;
#[doc = "`read()` method returns [c15tcr::R](c15tcr::R) reader structure"]
impl crate::Readable for C15TCR {}
#[doc = "`write(|w| ..)` method takes [c15tcr::W](c15tcr::W) writer structure"]
impl crate::Writable for C15TCR {}
#[doc = "This register is used to configure the concerned channel."]
pub mod c15tcr;
#[doc = "MDMA Channel x block number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15bndtr](c15bndtr) module"]
pub type C15BNDTR = crate::Reg<u32, _C15BNDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15BNDTR;
#[doc = "`read()` method returns [c15bndtr::R](c15bndtr::R) reader structure"]
impl crate::Readable for C15BNDTR {}
#[doc = "`write(|w| ..)` method takes [c15bndtr::W](c15bndtr::W) writer structure"]
impl crate::Writable for C15BNDTR {}
#[doc = "MDMA Channel x block number of data register"]
pub mod c15bndtr;
#[doc = "MDMA channel x source address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15sar](c15sar) module"]
pub type C15SAR = crate::Reg<u32, _C15SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15SAR;
#[doc = "`read()` method returns [c15sar::R](c15sar::R) reader structure"]
impl crate::Readable for C15SAR {}
#[doc = "`write(|w| ..)` method takes [c15sar::W](c15sar::W) writer structure"]
impl crate::Writable for C15SAR {}
#[doc = "MDMA channel x source address register"]
pub mod c15sar;
#[doc = "MDMA channel x destination address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15dar](c15dar) module"]
pub type C15DAR = crate::Reg<u32, _C15DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15DAR;
#[doc = "`read()` method returns [c15dar::R](c15dar::R) reader structure"]
impl crate::Readable for C15DAR {}
#[doc = "`write(|w| ..)` method takes [c15dar::W](c15dar::W) writer structure"]
impl crate::Writable for C15DAR {}
#[doc = "MDMA channel x destination address register"]
pub mod c15dar;
#[doc = "MDMA channel x Block Repeat address Update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15brur](c15brur) module"]
pub type C15BRUR = crate::Reg<u32, _C15BRUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15BRUR;
#[doc = "`read()` method returns [c15brur::R](c15brur::R) reader structure"]
impl crate::Readable for C15BRUR {}
#[doc = "`write(|w| ..)` method takes [c15brur::W](c15brur::W) writer structure"]
impl crate::Writable for C15BRUR {}
#[doc = "MDMA channel x Block Repeat address Update register"]
pub mod c15brur;
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15lar](c15lar) module"]
pub type C15LAR = crate::Reg<u32, _C15LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15LAR;
#[doc = "`read()` method returns [c15lar::R](c15lar::R) reader structure"]
impl crate::Readable for C15LAR {}
#[doc = "`write(|w| ..)` method takes [c15lar::W](c15lar::W) writer structure"]
impl crate::Writable for C15LAR {}
#[doc = "MDMA channel x Link Address register"]
pub mod c15lar;
#[doc = "MDMA channel x Trigger and Bus selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15tbr](c15tbr) module"]
pub type C15TBR = crate::Reg<u32, _C15TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15TBR;
#[doc = "`read()` method returns [c15tbr::R](c15tbr::R) reader structure"]
impl crate::Readable for C15TBR {}
#[doc = "`write(|w| ..)` method takes [c15tbr::W](c15tbr::W) writer structure"]
impl crate::Writable for C15TBR {}
#[doc = "MDMA channel x Trigger and Bus selection Register"]
pub mod c15tbr;
#[doc = "MDMA channel x Mask address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15mar](c15mar) module"]
pub type C15MAR = crate::Reg<u32, _C15MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15MAR;
#[doc = "`read()` method returns [c15mar::R](c15mar::R) reader structure"]
impl crate::Readable for C15MAR {}
#[doc = "`write(|w| ..)` method takes [c15mar::W](c15mar::W) writer structure"]
impl crate::Writable for C15MAR {}
#[doc = "MDMA channel x Mask address register"]
pub mod c15mar;
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c15mdr](c15mdr) module"]
pub type C15MDR = crate::Reg<u32, _C15MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C15MDR;
#[doc = "`read()` method returns [c15mdr::R](c15mdr::R) reader structure"]
impl crate::Readable for C15MDR {}
#[doc = "`write(|w| ..)` method takes [c15mdr::W](c15mdr::W) writer structure"]
impl crate::Writable for C15MDR {}
#[doc = "MDMA channel x Mask Data register"]
pub mod c15mdr;
