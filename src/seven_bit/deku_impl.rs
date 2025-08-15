macro_rules! int7bit_deku_shim_implementation {
    (
        module_name: $module_name: ident,
        local_type: $local_type: ident,
        internal_type: $internal_type: ident,
    ) => {
        mod $module_name {
            use deku::ctx::Order;
            use deku::reader::Reader;
            use deku::writer::Writer;
            use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

            use crate::InternalValue;

            use super::$local_type;

            impl DekuReader<'_> for $local_type {
                /// Read 7-bit int as described here:
                fn from_reader_with_ctx<R>(
                    reader: &mut Reader<R>,
                    _ctx: (),
                ) -> Result<Self, DekuError>
                where
                    R: no_std_io::Read + no_std_io::Seek,
                    Self: Sized,
                {
                    let mut result: $internal_type = 0;
                    let mut shift = 0;
                    let mut buf = [0];
                    // read 28 bits

                    let bits_full_7 = $internal_type::BITS / 7;
                    let bits_rest = $internal_type::BITS % 7;

                    for _ in 0..bits_full_7 {
                        reader.read_bytes(1, &mut buf, Order::default())?;

                        result |= ((buf[0] & 0x7f) as $internal_type) << shift;

                        if buf[0] <= 0x7f {
                            return Ok(Self(result));
                        }

                        shift += 7;
                    }

                    // assert!(bits_rest > 0, "Unlikely to ever happen");

                    reader.read_bytes(1, &mut buf, Order::default())?;

                    // rest_max_value is 1 + actual maximum value.
                    let rest_max_value = 1 << bits_rest;

                    if buf[0] >= rest_max_value {
                        return Err(DekuError::Assertion("Integer overflow".into()));
                    }

                    result |= (buf[0] as $internal_type) << shift;

                    Ok(result.into())
                }
            }

            impl DekuWriter for $local_type {
                fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
                    &self,
                    writer: &mut Writer<W>,
                    _ctx: (),
                ) -> Result<(), DekuError> {
                    let mut buf = [0];
                    let mut value = self.internal_ref().clone();

                    while value > 0x7f {
                        buf[0] = 0x80 + (value & 0x7f) as u8;
                        writer.write_bytes(&mut buf)?;
                        value >>= 7;
                    }
                    buf[0] = (value & 0x7f) as u8;
                    writer.write_bytes(&mut buf)
                }
            }
        }
    };
}

pub(super) use int7bit_deku_shim_implementation;
