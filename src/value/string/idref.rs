use static_regular_grammar::RegularGrammar;

use crate::ParseRdf;

/// ID.
///
/// ```abnf
/// NCName        = (Letter / "_") *NCNameChar
/// NCNameChar    = Letter / Digit / "." / "-" / "_" / CombiningChar / Extender
///
/// Letter        = BaseChar / Ideographic
/// BaseChar      = %x0041-005A / %x0061-007A / %x00C0-00D6 / %x00D8-00F6 / %x00F8-00FF / %x0100-0131 / %x0134-013E / %x0141-0148 / %x014A-017E / %x0180-01C3 / %x01CD-01F0 / %x01F4-01F5 / %x01FA-0217 / %x0250-02A8 / %x02BB-02C1 / %x0386 / %x0388-038A / %x038C / %x038E-03A1 / %x03A3-03CE / %x03D0-03D6 / %x03DA / %x03DC / %x03DE / %x03E0 / %x03E2-03F3 / %x0401-040C / %x040E-044F / %x0451-045C / %x045E-0481 / %x0490-04C4 / %x04C7-04C8 / %x04CB-04CC / %x04D0-04EB / %x04EE-04F5 / %x04F8-04F9 / %x0531-0556 / %x0559 / %x0561-0586 / %x05D0-05EA / %x05F0-05F2 / %x0621-063A / %x0641-064A / %x0671-06B7 / %x06BA-06BE / %x06C0-06CE / %x06D0-06D3 / %x06D5 / %x06E5-06E6 / %x0905-0939 / %x093D / %x0958-0961 / %x0985-098C / %x098F-0990 / %x0993-09A8 / %x09AA-09B0 / %x09B2 / %x09B6-09B9 / %x09DC-09DD / %x09DF-09E1 / %x09F0-09F1 / %x0A05-0A0A / %x0A0F-0A10 / %x0A13-0A28 / %x0A2A-0A30 / %x0A32-0A33 / %x0A35-0A36 / %x0A38-0A39 / %x0A59-0A5C / %x0A5E / %x0A72-0A74 / %x0A85-0A8B / %x0A8D / %x0A8F-0A91 / %x0A93-0AA8 / %x0AAA-0AB0 / %x0AB2-0AB3 / %x0AB5-0AB9 / %x0ABD / %x0AE0 / %x0B05-0B0C / %x0B0F-0B10 / %x0B13-0B28 / %x0B2A-0B30 / %x0B32-0B33 / %x0B36-0B39 / %x0B3D / %x0B5C-0B5D / %x0B5F-0B61 / %x0B85-0B8A / %x0B8E-0B90 / %x0B92-0B95 / %x0B99-0B9A / %x0B9C / %x0B9E-0B9F / %x0BA3-0BA4 / %x0BA8-0BAA / %x0BAE-0BB5 / %x0BB7-0BB9 / %x0C05-0C0C / %x0C0E-0C10 / %x0C12-0C28 / %x0C2A-0C33 / %x0C35-0C39 / %x0C60-0C61 / %x0C85-0C8C / %x0C8E-0C90 / %x0C92-0CA8 / %x0CAA-0CB3 / %x0CB5-0CB9 / %x0CDE / %x0CE0-0CE1 / %x0D05-0D0C / %x0D0E-0D10 / %x0D12-0D28 / %x0D2A-0D39 / %x0D60-0D61 / %x0E01-0E2E / %x0E30 / %x0E32-0E33 / %x0E40-0E45 / %x0E81-0E82 / %x0E84 / %x0E87-0E88 / %x0E8A / %x0E8D / %x0E94-0E97 / %x0E99-0E9F / %x0EA1-0EA3 / %x0EA5 / %x0EA7 / %x0EAA-0EAB / %x0EAD-0EAE / %x0EB0 / %x0EB2-0EB3 / %x0EBD / %x0EC0-0EC4 / %x0F40-0F47 / %x0F49-0F69 / %x10A0-10C5 / %x10D0-10F6 / %x1100 / %x1102-1103 / %x1105-1107 / %x1109 / %x110B-110C / %x110E-1112 / %x113C / %x113E / %x1140 / %x114C / %x114E / %x1150 / %x1154-1155 / %x1159 / %x115F-1161 / %x1163 / %x1165 / %x1167 / %x1169 / %x116D-116E / %x1172-1173 / %x1175 / %x119E / %x11A8 / %x11AB / %x11AE-11AF / %x11B7-11B8 / %x11BA / %x11BC-11C2 / %x11EB / %x11F0 / %x11F9 / %x1E00-1E9B / %x1EA0-1EF9 / %x1F00-1F15 / %x1F18-1F1D / %x1F20-1F45 / %x1F48-1F4D / %x1F50-1F57 / %x1F59 / %x1F5B / %x1F5D / %x1F5F-1F7D / %x1F80-1FB4 / %x1FB6-1FBC / %x1FBE / %x1FC2-1FC4 / %x1FC6-1FCC / %x1FD0-1FD3 / %x1FD6-1FDB / %x1FE0-1FEC / %x1FF2-1FF4 / %x1FF6-1FFC / %x2126 / %x212A-212B / %x212E / %x2180-2182 / %x3041-3094 / %x30A1-30FA / %x3105-312C / %xAC00-D7A3
/// Ideographic   = %x4E00-9FA5 / %x3007 / %x3021-3029
/// CombiningChar = %x0300-0345 / %x0360-0361 / %x0483-0486 / %x0591-05A1 / %x05A3-05B9 / %x05BB-05BD / %x05BF / %x05C1-05C2 / %x05C4 / %x064B-0652 / %x0670 / %x06D6-06DC / %x06DD-06DF / %x06E0-06E4 / %x06E7-06E8 / %x06EA-06ED / %x0901-0903 / %x093C / %x093E-094C / %x094D / %x0951-0954 / %x0962-0963 / %x0981-0983 / %x09BC / %x09BE / %x09BF / %x09C0-09C4 / %x09C7-09C8 / %x09CB-09CD / %x09D7 / %x09E2-09E3 / %x0A02 / %x0A3C / %x0A3E / %x0A3F / %x0A40-0A42 / %x0A47-0A48 / %x0A4B-0A4D / %x0A70-0A71 / %x0A81-0A83 / %x0ABC / %x0ABE-0AC5 / %x0AC7-0AC9 / %x0ACB-0ACD / %x0B01-0B03 / %x0B3C / %x0B3E-0B43 / %x0B47-0B48 / %x0B4B-0B4D / %x0B56-0B57 / %x0B82-0B83 / %x0BBE-0BC2 / %x0BC6-0BC8 / %x0BCA-0BCD / %x0BD7 / %x0C01-0C03 / %x0C3E-0C44 / %x0C46-0C48 / %x0C4A-0C4D / %x0C55-0C56 / %x0C82-0C83 / %x0CBE-0CC4 / %x0CC6-0CC8 / %x0CCA-0CCD / %x0CD5-0CD6 / %x0D02-0D03 / %x0D3E-0D43 / %x0D46-0D48 / %x0D4A-0D4D / %x0D57 / %x0E31 / %x0E34-0E3A / %x0E47-0E4E / %x0EB1 / %x0EB4-0EB9 / %x0EBB-0EBC / %x0EC8-0ECD / %x0F18-0F19 / %x0F35 / %x0F37 / %x0F39 / %x0F3E / %x0F3F / %x0F71-0F84 / %x0F86-0F8B / %x0F90-0F95 / %x0F97 / %x0F99-0FAD / %x0FB1-0FB7 / %x0FB9 / %x20D0-20DC / %x20E1 / %x302A-302F / %x3099 / %x309A
/// Digit         = %x0030-0039 / %x0660-0669 / %x06F0-06F9 / %x0966-096F / %x09E6-09EF / %x0A66-0A6F / %x0AE6-0AEF / %x0B66-0B6F / %x0BE7-0BEF / %x0C66-0C6F / %x0CE6-0CEF / %x0D66-0D6F / %x0E50-0E59 / %x0ED0-0ED9 / %x0F20-0F29
/// Extender      = %x00B7 / %x02D0 / %x02D1 / %x0387 / %x0640 / %x0E46 / %x0EC6 / %x3005 / %x3031-3035 / %x309D-309E / %x30FC-30FE
/// ```
///
#[derive(RegularGrammar, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[grammar(
	sized(IdRefBuf, derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)),
	cache = "automata/ncname.automaton.cbor"
)]
pub struct IdRef(str);

impl ParseRdf for IdRefBuf {
	type LexicalForm = crate::lexical::IdRef;
}
