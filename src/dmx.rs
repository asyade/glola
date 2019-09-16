//! Based on https://art-net.org.uk/structure/streaming-packets/artdmx-packet-definition/

/// Dont modify fields order they match with DMX512 definition and
#[derive(Clone)]
#[repr(packed)]
pub struct ArtDmx {
    /// The ID field is 8 bytes in length and contains the null terminated string of ASCII characters: Art-Net
    pub id: [u8; 8],
    /// The purpose of the field is to identify the packet as an Art-Net protocol packet. All Art-Net packets start with this field. Receivers must check this field is correct and discard the packet if it is not.
    /// The OpCode field is a 16-bit word transmitted low byte first. The OpCode identifies the packet type. In the ArtDmx packet this field is set to ‘OpDmx’.
    pub op_code: u16,
    /// The ProtVer field is a 16-bit word transmitted high byte first. It identifies the version of the Art-Net protocol. The protocol version is not related to Art-Net 1, II, 3 or 4
    ///. Only one protocol version has ever existed in production equipment which is 1410. Receivers should use the following good packet test: ProtVer >= 14
    /// The ProtVer may be increased in a future releases to identify significant functionality changes.
    pub proto_ver: [u8; 2],
    /// The Sequence field is an 8-bit number that is designed to show the order in which packets were originated. When Art-Net is transferred over media such as the Internet, it is possible for packets to arrive at their destination out of sequence. This field allows the receiver to trap such errors.
    /// The generating device increments this number for every packet sent to a specific Port-Address. The number increments from 110 to 25510 and then rolls over to 110 and repeats. This is because the value 010 is reserved to show that Sequence is not implemented.
    pub sequence: u8,
    /// The Physical field is an 8-bit number that defines the physical port that generated the packet. This number is limited to the range 010 to 310.
    ///It is intended to be purely informative and is not used to define the destination of the packet.
    pub physical: u8,
    /// Net and SubUni are combined to form the 15-bit Port-Address to which this packet is directed. The low 7-bits of the Net field define bits 14-8 of the 15-bit Port-Address.
    /// The 8-bits of the SubUni field define bits 7-0 of the 15-bit Port-Address.
    /// The diagram below shows the bit allocation within the Net field.
    ///
    /// *bit*  *net*
    /// 7  Not used, set to 0
    /// 6  Port-address bit-14
    /// 5  Port-address bit-13
    /// 4  Port-address bit-12
    /// 3  Port-address bit-11
    /// 2  Port-address bit-10
    /// 1  Port-address bit-9
    /// 0  Port-address bit-8
    ///
    /// The bit organisation of the Port-Address is shown in the diagram below.
    /// port-addr net switchUni
    /// *bits* *12-8* *7-0*
    pub sub_uni: u8,
    pub net: u8,
    /// The Length field is a 16-bit number transmitted most significant byte first. It defines the number of bytes encoded in the Data[] field. The Length field can be set to any even value in the range 210 – 51210.
    /// The requirement thaA value greater that 51210 is an error and receivers should discard the value.
    /// it be an even number is something of a historical anomaly. It was intended to ensure that when receivers are reading the payload using 16-bit reads, a buffer overrun would not occur on the last odd byte. In reality, implementers have widely ignored this requirement and so designers of receivers should not assume that this field will be an even number.
    pub lenght: u16,
    /// The Data field contains the data slot (channel levels). The size of this array is defined by the Length field.
    /// The first 8-bit entry in the Data field is always data-slot 1 or channel 1.
    pub data: [u8; 512],
}
