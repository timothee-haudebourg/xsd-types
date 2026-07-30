#!/usr/bin/env ruby

def variant(name, pattern)
	if pattern.nil? then
		name
	else
		"#{name}(#{pattern})"
	end
end

class Datatype
	attr_reader :name, :owned_name, :ref_name
	attr_reader :iri
	attr_reader :subclasses

	# `cmp` is the strongest comparison capability this datatype's own value
	# type has, one of:
	# - `:ord` — `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash` (most types).
	# - `:eq`  — `PartialEq`, `Eq`, `PartialOrd`, `Hash`, but no `Ord` (e.g.
	#            `Duration`, `DateTime`: only a partial order).
	# - `nil`  — `PartialEq`, `PartialOrd` only (`Float`/`Double`: `NaN` isn't
	#            even equal to itself, so `Eq`/`Ord`/`Hash` don't apply).
	def initialize(name, iri, copy, cmp, subclasses)
		if name.class == String then
			@name = name
			@owned_name = name
			@ref_name = name
		else
			@name = name[:variant]
			@owned_name = name[:owned]
			@ref_name = name[:ref] || name[:variant]
		end

		@iri = iri
		@copy = copy
		@cmp = cmp
		@subclasses = subclasses
	end

	def is_copy?
		@copy
	end

	# Whether this datatype's own value type has `Eq`/`Hash` at all.
	def has_eq?
		@cmp == :ord || @cmp == :eq
	end

	# Whether this datatype's own value type has a total order (`Ord`), as
	# opposed to merely a partial order (`PartialOrd`).
	def has_ord?
		@cmp == :ord
	end

	def generate_datatype_enum
		puts "/// [`#{@ref_name}`] datatype variants."
		puts "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]"
		puts "pub enum #{@name}Datatype {"

		puts "\t#{@name},"

		@subclasses.each do |c|
			if c.subclasses.empty? then
				puts "\t#{c.name},"
			else
				puts "\t#{c.name}(#{c.name}Datatype),"
			end
		end

		puts "}"

		puts "impl #{@name}Datatype {"
		puts "\tpub fn from_iri(iri: &Iri) -> Option<Self> {"
		puts "\t\tif iri == #{@iri} {"
		puts "\t\t\treturn Some(Self::#{@name})"
		puts "\t\t}"
		@subclasses.each do |c|
			if c.subclasses.empty? then
				puts "\t\tif iri == #{c.iri} {"
				puts "\t\t\treturn Some(Self::#{c.name})"
				puts "\t\t}"
			else
				puts "\t\tif let Some(t) = #{c.name}Datatype::from_iri(iri) {"
				puts "\t\t\treturn Some(Self::#{c.name}(t))"
				puts "\t\t}"
			end
		end
		puts "\t\tNone"
		puts "\t}"
		puts "\tpub fn iri(&self) -> &'static Iri {"
		puts "\t\tmatch self {"
		puts "\t\t\tSelf::#{@name} => #{@iri},"
		@subclasses.each do |c|
			if c.subclasses.empty? then
				puts "\t\t\tSelf::#{c.name} => #{c.iri},"
			else
				puts "\t\t\tSelf::#{c.name}(t) => t.iri(),"
			end
		end
		puts "\t\t}"
		puts "\t}"
		puts "\tpub fn parse(&self, value: &str) -> Result<#{@name}Value, ParseXsdError> {"
		puts "\t\tmatch self {"
		puts "\t\t\tSelf::#{@name} => ParseXsd::parse_xsd(value).map(#{@name}Value::#{@name}),"
		@subclasses.each do |c|
			if c.subclasses.empty? then
				puts "\t\t\tSelf::#{c.name} => ParseXsd::parse_xsd(value).map(#{@name}Value::#{c.name}),"
			else
				puts "\t\t\tSelf::#{c.name}(t) => t.parse(value).map(Into::into),"
			end
		end
		puts "\t\t}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_into_datatype("#{@name}Datatype", lambda { |value| "Self::#{c.name}(#{value})" })
				c.generate_try_from_datatype("#{@name}Datatype", lambda { |v, p| "#{@name}Datatype::#{v}(#{p})" })
			end
		end
	end

	def generate_value_enum
		puts "/// Any specialized [`#{@ref_name}`] value."
		traits = ["Debug", "Clone"]
		traits << "Copy" if self.is_copy?
		traits << "PartialEq"
		traits << "Eq" if self.has_eq?
		traits << "PartialOrd"
		traits << "Ord" if self.has_ord?
		traits << "Hash" if self.has_eq?
		puts "#[derive(#{traits.join(", ")})]"
		puts "pub enum #{@name}Value {"
		self.generate_value_variants
		puts "}"

		puts "impl #{@name}Value {"
		puts "\tpub fn datatype(&self) -> #{@name}Datatype {"
		puts "\t\tmatch self {"
		puts "\t\t\tSelf::#{@name}(_) => #{@name}Datatype::#{@name},"
		@subclasses.each do |c|
			c.generate_datatype_cases(lambda {|t| variant("#{@name}Datatype::#{c.name}", t)})
		end
		puts "\t\t}"
		puts "\t}"
		puts "}"

		puts "impl XsdValue for #{@name}Value {"
		puts "\tfn datatype(&self) -> Datatype {"
		puts "\t\tself.datatype().into()"
		puts "\t}"
		puts "}"

		puts "impl fmt::Display for #{@name}Value {"
		puts "\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
		puts "\t\tmatch self {"
		self.generate_fmt
		puts "\t\t}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_into_value("#{@name}Value")
				c.generate_try_from_value("#{@name}Value")
				c.generate_datatype_enum
			end
		end

		if self.any_subtype? { |c| !c.is_copy? } then
			puts "/// Any specialized [`#{@ref_name}`] value reference."
			traits = ["Debug", "Clone", "Copy", "PartialEq"]
			traits << "Eq" if self.has_eq?
			traits << "PartialOrd"
			traits << "Ord" if self.has_ord?
			traits << "Hash" if self.has_eq?
			puts "#[derive(#{traits.join(", ")})]"
			puts "pub enum #{@name}ValueRef<'a> {"
			self.generate_value_ref_variants
			puts "}"

			puts "impl #{@name}Value {"
			puts "\tpub fn as_ref(&self) -> #{@name}ValueRef<'_> {"
			puts "\t\tmatch self {"
			self.generate_value_variants_as_ref("#{@name}ValueRef")
			puts "\t\t}"
			puts "\t}"
			puts "}"

			puts "impl<'a> #{@name}ValueRef<'a> {"
			puts "\tpub fn datatype(&self) -> #{@name}Datatype {"
			puts "\t\tmatch self {"
			puts "\t\t\tSelf::#{@name}(_) => #{@name}Datatype::#{@name},"
			@subclasses.each do |c|
				c.generate_datatype_cases(lambda {|t| variant("#{@name}Datatype::#{c.name}", t)})
			end
			puts "\t\t}"
			puts "\t}"
			puts "\tpub fn into_owned(self) -> #{@name}Value {"
			puts "\t\tmatch self {"
			self.generate_value_ref_variants_to_owned("#{@name}Value")
			puts "\t\t}"
			puts "\t}"
			puts "}"

			puts "impl<'a> XsdValue for #{@name}ValueRef<'a> {"
			puts "\tfn datatype(&self) -> Datatype {"
			puts "\t\tself.datatype().into()"
			puts "\t}"
			puts "}"

			puts "impl<'a> fmt::Display for #{@name}ValueRef<'a> {"
			puts "\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
			puts "\t\tmatch self {"
			self.generate_fmt
			puts "\t\t}"
			puts "\t}"
			puts "}"

			@subclasses.each do |c|
				if !c.subclasses.empty? then
					c.generate_into_value_ref("#{@name}ValueRef")
					c.generate_try_from_value_ref("#{@name}ValueRef")
				end
			end
		end

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_value_enum
			end
		end
	end

	def generate_into_datatype(t, f)
		puts "impl From<#{@name}Datatype> for #{t} {"
		puts "\tfn from(value: #{@name}Datatype) -> Self {"
		puts "\t\t#{f.call("value")}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_into_datatype(t, lambda { |value| f.call("#{@name}Datatype::#{c.name}(#{value})") })
			end
		end
	end

	def generate_into_value(t)
		puts "impl From<#{@name}Value> for #{t} {"
		puts "\tfn from(value: #{@name}Value) -> Self {"
		puts "\t\tmatch value {"
		self.each_subtype do |t|
			puts "#{@name}Value::#{t.name}(value) => Self::#{t.name}(value),"
		end
		puts "\t\t}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_into_value(t)
			end
		end
	end

	def generate_into_value_ref(t)
		if self.any_subtype? { |t| !t.is_copy? } then
			puts "impl<'a> From<#{@name}ValueRef<'a>> for #{t}<'a> {"
			puts "\tfn from(value: #{@name}ValueRef<'a>) -> Self {"
			puts "\t\tmatch value {"
			self.each_subtype do |t|
				puts "#{@name}ValueRef::#{t.name}(value) => Self::#{t.name}(value),"
			end
			puts "\t\t}"
			puts "\t}"
			puts "}"

			@subclasses.each do |c|
				if !c.subclasses.empty? then
					c.generate_into_value_ref(t)
				end
			end
		end
	end

	def generate_try_from_value(v)
		puts "impl TryFrom<#{v}> for #{@name}Value {"
		puts "\ttype Error = #{v};"
		puts "\tfn try_from(value: #{v}) -> Result<Self, #{v}> {"
		puts "\t\tmatch value {"
		self.each_subtype do |s|
			puts "\t\t\t#{v}::#{s.name}(value) => Ok(Self::#{s.name}(value)),"
		end
		puts "\t\t\tother => Err(other)"
		puts "\t\t}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			c.generate_try_from_value(v) if !c.subclasses.empty?
		end
	end

	def generate_try_from_value_ref(v)
		if self.any_subtype? { |t| !t.is_copy? } then
			puts "impl<'a> TryFrom<#{v}<'a>> for #{@name}ValueRef<'a> {"
			puts "\ttype Error = #{v}<'a>;"
			puts "\tfn try_from(value: #{v}<'a>) -> Result<Self, #{v}<'a>> {"
			puts "\t\tmatch value {"
			self.each_subtype do |s|
				puts "\t\t\t#{v}::#{s.name}(value) => Ok(Self::#{s.name}(value)),"
			end
			puts "\t\t\tother => Err(other)"
			puts "\t\t}"
			puts "\t}"
			puts "}"

			@subclasses.each do |c|
				c.generate_try_from_value_ref(v) if !c.subclasses.empty?
			end
		end
	end

	def generate_try_from_datatype(d, f)
		puts "impl TryFrom<#{d}> for #{@name}Datatype {"
		puts "\ttype Error = #{d};"
		puts "\tfn try_from(value: #{d}) -> Result<Self, #{d}> {"
		puts "\t\tmatch value {"
		puts "\t\t\t#{f.call(@name, "value")} => Ok(value),"
		puts "\t\t\tother => Err(other)"
		puts "\t\t}"
		puts "\t}"
		puts "}"

		@subclasses.each do |c|
			if !c.subclasses.empty? then
				c.generate_try_from_datatype(d, lambda { |v, p| f.call(@name, "#{@name}Datatype::#{v}(#{p})") })
			end
		end
	end

	def each_subtype(&block)
		block.call(self)
		@subclasses.each { |c| c.each_subtype(&block) }
	end

	def any_subtype?(&block)
		block.call(self) || @subclasses.any? { |c| c.any_subtype?(&block) }
	end

	def generate_value_variants
		self.each_subtype do |t|
			puts "\t#{t.name}(#{t.owned_name}),"
		end
	end

	def generate_value_ref_variants
		self.each_subtype do |t|
			if t.is_copy? then
				puts "\t#{t.name}(#{t.ref_name}),"
			else
				puts "\t#{t.name}(&'a #{t.ref_name}),"
			end
		end
	end

	def generate_value_variants_as_ref(ref_ty)
		self.each_subtype do |t|
			if t.is_copy? then
				puts "\t\t\tSelf::#{t.name}(value) => #{ref_ty}::#{t.name}(*value),"
			else
				puts "\t\t\tSelf::#{t.name}(value) => #{ref_ty}::#{t.name}(value),"
			end
		end
	end

	def generate_value_ref_variants_to_owned(ty)
		self.each_subtype do |t|
			if t.is_copy? then
				puts "\t\t\tSelf::#{t.name}(value) => #{ty}::#{t.name}(value),"
			else
				puts "\t\t\tSelf::#{t.name}(value) => #{ty}::#{t.name}(value.to_owned()),"
			end
		end
	end

	def generate_datatype_cases(f)
		if @subclasses.empty? then
			puts "\t\t\tSelf::#{@name}(_) => #{f.call(nil)},"
		else
			puts "\t\t\tSelf::#{@name}(_) => #{f.call("#{@name}Datatype::#{@name}")},"
			@subclasses.each { |c| c.generate_datatype_cases(lambda { |v| f.call(variant("#{@name}Datatype::#{c.name}", v)) }) }
		end
	end

	def generate_fmt
		self.each_subtype do |t|
			puts "Self::#{t.name}(v) => v.fmt(f),"
		end
	end
end

datatypes = [
	Datatype.new("Boolean", "XSD_BOOLEAN", true, :ord, []),
	# `NaN` isn't even equal to itself, so `Float`/`Double` have neither `Eq`
	# nor `Ord` (only `PartialEq`/`PartialOrd`, matching `f32`/`f64` exactly).
	Datatype.new("Float", "XSD_FLOAT", true, nil, []),
	Datatype.new("Double", "XSD_DOUBLE", true, nil, []),
	Datatype.new("Decimal", "XSD_DECIMAL", false, :ord, [
		Datatype.new("Integer", "XSD_INTEGER", false, :ord, [
			Datatype.new("NonPositiveInteger", "XSD_NON_POSITIVE_INTEGER", false, :ord, [
				Datatype.new("NegativeInteger", "XSD_NEGATIVE_INTEGER", false, :ord, [])
			]),
			Datatype.new("NonNegativeInteger", "XSD_NON_NEGATIVE_INTEGER", false, :ord, [
				Datatype.new("PositiveInteger", "XSD_POSITIVE_INTEGER", false, :ord, []),
				Datatype.new("UnsignedLong", "XSD_UNSIGNED_LONG", true, :ord, [
					Datatype.new("UnsignedInt", "XSD_UNSIGNED_INT", true, :ord, [
						Datatype.new("UnsignedShort", "XSD_UNSIGNED_SHORT", true, :ord, [
							Datatype.new("UnsignedByte", "XSD_UNSIGNED_BYTE", true, :ord, [])
						])
					])
				])
			]),
			Datatype.new("Long", "XSD_LONG", true, :ord, [
				Datatype.new("Int", "XSD_INT", true, :ord, [
					Datatype.new("Short", "XSD_SHORT", true, :ord, [
						Datatype.new("Byte", "XSD_BYTE", true, :ord, [])
					])
				])
			])
		])
	]),
	Datatype.new({ variant: "String", owned: "String", ref: "str" }, "XSD_STRING", false, :ord, [
		Datatype.new({ variant: "NormalizedString", owned: "NormalizedString", ref: "NormalizedStr" }, "XSD_NORMALIZED_STRING", false, :ord, [
			Datatype.new({ variant: "Token", owned: "TokenBuf" }, "XSD_TOKEN", false, :ord, [
				Datatype.new({ variant: "Language", owned: "LanguageBuf" }, "XSD_LANGUAGE", false, :ord, []),
				Datatype.new({ variant: "Name", owned: "NameBuf" }, "XSD_NAME", false, :ord, [
					Datatype.new({ variant: "NCName", owned: "NCNameBuf" }, "XSD_NC_NAME", false, :ord, [
						Datatype.new({ variant: "Id", owned: "IdBuf" }, "XSD_ID", false, :ord, []),
						Datatype.new({ variant: "IdRef", owned: "IdRefBuf" }, "XSD_IDREF", false, :ord, []),
					])
				]),
				Datatype.new({ variant: "NMToken", owned: "NMTokenBuf" }, "XSD_NMTOKEN", false, :ord, []),
			])
		])
	]),
	# `Duration` itself, unlike `DayTimeDuration`/`YearMonthDuration`, only has a
	# partial order (comparison via four fixed reference `dateTime`s can be
	# indeterminate, e.g. `P1M <> P30D`); see `Duration`'s `PartialOrd` impl.
	Datatype.new("Duration", "XSD_DURATION", true, :eq, [
		Datatype.new("DayTimeDuration", "XSD_DAY_TIME_DURATION", true, :ord, []),
		Datatype.new("YearMonthDuration", "XSD_YEAR_MONTH_DURATION", true, :ord, []),
	]),
	# `DateTime` itself, unlike `DateTimeStamp`, only has a partial order (two
	# values can be incomparable when only one has a timezone offset).
	Datatype.new("DateTime", "XSD_DATE_TIME", true, :eq, [
		Datatype.new("DateTimeStamp", "XSD_DATE_TIME_STAMP", true, :ord, []),
	]),
	# `Time`/`Date`/`GMonthDay`/`GDay` only have a partial order, for the same
	# reason as `DateTime`. `GYearMonth`/`GYear`/`GMonth` are always totally
	# ordered instead: two distinct values are always at least 28 days apart,
	# which exceeds the `±14:00` offset uncertainty window (28 hours).
	Datatype.new("Time", "XSD_TIME", true, :eq, []),
	Datatype.new("Date", "XSD_DATE", true, :eq, []),
	Datatype.new("GYearMonth", "XSD_G_YEAR_MONTH", true, :ord, []),
	Datatype.new("GYear", "XSD_G_YEAR", true, :ord, []),
	Datatype.new("GMonthDay", "XSD_G_MONTH_DAY", true, :eq, []),
	Datatype.new("GDay", "XSD_G_DAY", true, :eq, []),
	Datatype.new("GMonth", "XSD_G_MONTH", true, :ord, []),
	Datatype.new({ variant: "Base64Binary", owned: "Base64BinaryBuf" }, "XSD_BASE64_BINARY", false, :ord, []),
	Datatype.new({ variant: "HexBinary", owned: "HexBinaryBuf" }, "XSD_HEX_BINARY", false, :ord, []),
	Datatype.new({ variant: "AnyUri", owned: "AnyUriBuf" }, "XSD_ANY_URI", false, :ord, []),
	Datatype.new({ variant: "QName", owned: "QNameBuf" }, "XSD_Q_NAME", false, :ord, []),
	# Datatype.new("Notation", "XSD_NOTATION", false, :ord, [])
]

def generate_datatype_enum(classes)
	puts "/// XSD datatype (primitive or not)."
	puts "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]"
	puts "pub enum Datatype {"

	classes.each do |c|
		if c.subclasses.empty? then
			puts "\t#{c.name},"
		else
			puts "\t#{c.name}(#{c.name}Datatype),"
		end
	end

	puts "}"

	puts "impl Datatype {"
	puts "\tpub fn from_iri(iri: &Iri) -> Option<Self> {"
	classes.each do |c|
		if c.subclasses.empty? then
			puts "\t\tif iri == #{c.iri} {"
			puts "\t\t\treturn Some(Self::#{c.name})"
			puts "\t\t}"
		else
			puts "\t\tif let Some(t) = #{c.name}Datatype::from_iri(iri) {"
			puts "\t\t\treturn Some(Self::#{c.name}(t))"
			puts "\t\t}"
		end
	end
	puts "\t\tNone"
	puts "\t}"
	puts "\tpub fn iri(&self) -> &'static Iri {"
	puts "\t\tmatch self {"
	classes.each do |c|
		if c.subclasses.empty? then
			puts "\t\t\tSelf::#{c.name} => #{c.iri},"
		else
			puts "\t\t\tSelf::#{c.name}(t) => t.iri(),"
		end
	end
	puts "\t\t}"
	puts "\t}"
	puts "\tpub fn parse(&self, value: &str) -> Result<Value, ParseXsdError> {"
	puts "\t\tmatch self {"
	classes.each do |c|
		if c.subclasses.empty? then
			puts "\t\t\tSelf::#{c.name} => ParseXsd::parse_xsd(value).map(Value::#{c.name}),"
		else
			puts "\t\t\tSelf::#{c.name}(t) => t.parse(value).map(Into::into),"
		end
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	classes.each do |c|
		if !c.subclasses.empty? then
			c.generate_datatype_enum
		end
	end
end

def generate_value_enum(classes)
	puts "/// Any XSD value."
	# `Time`/`Date`/`GMonthDay`/`GDay`/`Duration`/`DateTime` only have a
	# partial order, so `Value`/`ValueRef` can never derive `Ord`. `Float`/
	# `Double` don't even have `Eq`/`Hash` (`NaN` is incomparable with itself),
	# so `Value`/`ValueRef` can't derive those either.
	puts "#[derive(Debug, Clone, PartialEq, PartialOrd)]"
	puts "pub enum Value {"
	classes.each do |c|
		c.generate_value_variants
	end
	puts "}"

	puts "impl Value {"
	puts "\tpub fn datatype(&self) -> Datatype {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_datatype_cases(lambda {|t| variant("Datatype::#{c.name}", t)})
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	puts "impl XsdValue for Value {"
	puts "\tfn datatype(&self) -> Datatype {"
	puts "\t\tself.datatype()"
	puts "\t}"
	puts "}"

	puts "impl fmt::Display for Value {"
	puts "\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_fmt
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	puts "/// Any XSD value reference."
	puts "#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]"
	puts "pub enum ValueRef<'a> {"
	classes.each do |c|
		c.generate_value_ref_variants
	end
	puts "}"

	puts "impl<'a> fmt::Display for ValueRef<'a> {"
	puts "\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_fmt
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	puts "impl Value {"
	puts "\tpub fn as_ref(&self) -> ValueRef<'_> {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_value_variants_as_ref("ValueRef")
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	puts "impl<'a> ValueRef<'a> {"
	puts "\tpub fn datatype(&self) -> Datatype {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_datatype_cases(lambda {|t| variant("Datatype::#{c.name}", t)})
	end
	puts "\t\t}"
	puts "\t}"
	puts "\tpub fn into_owned(self) -> Value {"
	puts "\t\tmatch self {"
	classes.each do |c|
		c.generate_value_ref_variants_to_owned("Value")
	end
	puts "\t\t}"
	puts "\t}"
	puts "}"

	puts "impl<'a> XsdValue for ValueRef<'a> {"
	puts "\tfn datatype(&self) -> Datatype {"
	puts "\t\tself.datatype()"
	puts "\t}"
	puts "}"

	classes.each do |c|
		if !c.subclasses.empty? then
			c.generate_into_datatype("Datatype", lambda { |value| "Self::#{c.name}(#{value})" })
			c.generate_try_from_datatype("Datatype", lambda { |v, p| "Datatype::#{v}(#{p})" })
			c.generate_into_value("Value")
			c.generate_try_from_value("Value")
			c.generate_into_value_ref("ValueRef")
			c.generate_try_from_value_ref("ValueRef")
			c.generate_value_enum
		end
	end
end

puts "// @generated by `ruby utils/generate.rb`. Do not edit directly;"
puts "// edit the generator instead, then run:"
puts "//     ruby utils/generate.rb > src/types.rs && cargo fmt"
puts "use iref::Iri;"
puts "use std::fmt;"
puts "use crate::{"
puts "XsdValue,"
puts "ParseXsd,"
puts "ParseXsdError,"
datatypes.each do |t|
	t.each_subtype do |t|
		puts "#{t.iri},"
	end
end
puts "};"
puts "use super::{"
datatypes.each do |t|
	t.each_subtype do |t|
		if t.owned_name != "String" then
			puts "#{t.owned_name},"
			puts "#{t.ref_name}," if t.ref_name != t.owned_name
		end
	end
end
puts "};"

generate_datatype_enum(datatypes)
generate_value_enum(datatypes)
