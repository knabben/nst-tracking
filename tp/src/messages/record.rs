// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `record.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct Record {
    // message fields
    pub record_id: ::std::string::String,
    pub owners: ::protobuf::RepeatedField<Record_Owner>,
    pub locations: ::protobuf::RepeatedField<Record_Location>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Record {
    fn default() -> &'a Record {
        <Record as ::protobuf::Message>::default_instance()
    }
}

impl Record {
    pub fn new() -> Record {
        ::std::default::Default::default()
    }

    // string record_id = 1;


    pub fn get_record_id(&self) -> &str {
        &self.record_id
    }
    pub fn clear_record_id(&mut self) {
        self.record_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_record_id(&mut self, v: ::std::string::String) {
        self.record_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_record_id(&mut self) -> &mut ::std::string::String {
        &mut self.record_id
    }

    // Take field
    pub fn take_record_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.record_id, ::std::string::String::new())
    }

    // repeated .Record.Owner owners = 2;


    pub fn get_owners(&self) -> &[Record_Owner] {
        &self.owners
    }
    pub fn clear_owners(&mut self) {
        self.owners.clear();
    }

    // Param is passed by value, moved
    pub fn set_owners(&mut self, v: ::protobuf::RepeatedField<Record_Owner>) {
        self.owners = v;
    }

    // Mutable pointer to the field.
    pub fn mut_owners(&mut self) -> &mut ::protobuf::RepeatedField<Record_Owner> {
        &mut self.owners
    }

    // Take field
    pub fn take_owners(&mut self) -> ::protobuf::RepeatedField<Record_Owner> {
        ::std::mem::replace(&mut self.owners, ::protobuf::RepeatedField::new())
    }

    // repeated .Record.Location locations = 3;


    pub fn get_locations(&self) -> &[Record_Location] {
        &self.locations
    }
    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }

    // Param is passed by value, moved
    pub fn set_locations(&mut self, v: ::protobuf::RepeatedField<Record_Location>) {
        self.locations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locations(&mut self) -> &mut ::protobuf::RepeatedField<Record_Location> {
        &mut self.locations
    }

    // Take field
    pub fn take_locations(&mut self) -> ::protobuf::RepeatedField<Record_Location> {
        ::std::mem::replace(&mut self.locations, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Record {
    fn is_initialized(&self) -> bool {
        for v in &self.owners {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.record_id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.owners)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locations)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.record_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.record_id);
        }
        for value in &self.owners {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.locations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.record_id.is_empty() {
            os.write_string(1, &self.record_id)?;
        }
        for v in &self.owners {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.locations {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Record {
        Record::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "record_id",
                    |m: &Record| { &m.record_id },
                    |m: &mut Record| { &mut m.record_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Record_Owner>>(
                    "owners",
                    |m: &Record| { &m.owners },
                    |m: &mut Record| { &mut m.owners },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Record_Location>>(
                    "locations",
                    |m: &Record| { &m.locations },
                    |m: &mut Record| { &mut m.locations },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record>(
                    "Record",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Record {
        static mut instance: ::protobuf::lazy::Lazy<Record> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record,
        };
        unsafe {
            instance.get(Record::new)
        }
    }
}

impl ::protobuf::Clear for Record {
    fn clear(&mut self) {
        self.record_id.clear();
        self.owners.clear();
        self.locations.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Record_Owner {
    // message fields
    pub agent_id: ::std::string::String,
    pub timestamp: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Record_Owner {
    fn default() -> &'a Record_Owner {
        <Record_Owner as ::protobuf::Message>::default_instance()
    }
}

impl Record_Owner {
    pub fn new() -> Record_Owner {
        ::std::default::Default::default()
    }

    // string agent_id = 1;


    pub fn get_agent_id(&self) -> &str {
        &self.agent_id
    }
    pub fn clear_agent_id(&mut self) {
        self.agent_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_agent_id(&mut self, v: ::std::string::String) {
        self.agent_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_agent_id(&mut self) -> &mut ::std::string::String {
        &mut self.agent_id
    }

    // Take field
    pub fn take_agent_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.agent_id, ::std::string::String::new())
    }

    // uint64 timestamp = 2;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }
}

impl ::protobuf::Message for Record_Owner {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.agent_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.agent_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.agent_id);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.agent_id.is_empty() {
            os.write_string(1, &self.agent_id)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(2, self.timestamp)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Record_Owner {
        Record_Owner::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "agent_id",
                    |m: &Record_Owner| { &m.agent_id },
                    |m: &mut Record_Owner| { &mut m.agent_id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    |m: &Record_Owner| { &m.timestamp },
                    |m: &mut Record_Owner| { &mut m.timestamp },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record_Owner>(
                    "Record_Owner",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Record_Owner {
        static mut instance: ::protobuf::lazy::Lazy<Record_Owner> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record_Owner,
        };
        unsafe {
            instance.get(Record_Owner::new)
        }
    }
}

impl ::protobuf::Clear for Record_Owner {
    fn clear(&mut self) {
        self.agent_id.clear();
        self.timestamp = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record_Owner {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record_Owner {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Record_Location {
    // message fields
    pub latitude: i64,
    pub longitude: i64,
    pub timestamp: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Record_Location {
    fn default() -> &'a Record_Location {
        <Record_Location as ::protobuf::Message>::default_instance()
    }
}

impl Record_Location {
    pub fn new() -> Record_Location {
        ::std::default::Default::default()
    }

    // sint64 latitude = 1;


    pub fn get_latitude(&self) -> i64 {
        self.latitude
    }
    pub fn clear_latitude(&mut self) {
        self.latitude = 0;
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: i64) {
        self.latitude = v;
    }

    // sint64 longitude = 2;


    pub fn get_longitude(&self) -> i64 {
        self.longitude
    }
    pub fn clear_longitude(&mut self) {
        self.longitude = 0;
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: i64) {
        self.longitude = v;
    }

    // uint64 timestamp = 3;


    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }
}

impl ::protobuf::Message for Record_Location {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.latitude = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.longitude = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.latitude != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.latitude);
        }
        if self.longitude != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.longitude);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(3, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.latitude != 0 {
            os.write_sint64(1, self.latitude)?;
        }
        if self.longitude != 0 {
            os.write_sint64(2, self.longitude)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(3, self.timestamp)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Record_Location {
        Record_Location::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "latitude",
                    |m: &Record_Location| { &m.latitude },
                    |m: &mut Record_Location| { &mut m.latitude },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "longitude",
                    |m: &Record_Location| { &m.longitude },
                    |m: &mut Record_Location| { &mut m.longitude },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    |m: &Record_Location| { &m.timestamp },
                    |m: &mut Record_Location| { &mut m.timestamp },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record_Location>(
                    "Record_Location",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Record_Location {
        static mut instance: ::protobuf::lazy::Lazy<Record_Location> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record_Location,
        };
        unsafe {
            instance.get(Record_Location::new)
        }
    }
}

impl ::protobuf::Clear for Record_Location {
    fn clear(&mut self) {
        self.latitude = 0;
        self.longitude = 0;
        self.timestamp = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Record_Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Record_Location {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RecordContainer {
    // message fields
    pub entries: ::protobuf::RepeatedField<Record>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RecordContainer {
    fn default() -> &'a RecordContainer {
        <RecordContainer as ::protobuf::Message>::default_instance()
    }
}

impl RecordContainer {
    pub fn new() -> RecordContainer {
        ::std::default::Default::default()
    }

    // repeated .Record entries = 1;


    pub fn get_entries(&self) -> &[Record] {
        &self.entries
    }
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<Record>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<Record> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<Record> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for RecordContainer {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RecordContainer {
        RecordContainer::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Record>>(
                    "entries",
                    |m: &RecordContainer| { &m.entries },
                    |m: &mut RecordContainer| { &mut m.entries },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecordContainer>(
                    "RecordContainer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static RecordContainer {
        static mut instance: ::protobuf::lazy::Lazy<RecordContainer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecordContainer,
        };
        unsafe {
            instance.get(RecordContainer::new)
        }
    }
}

impl ::protobuf::Clear for RecordContainer {
    fn clear(&mut self) {
        self.entries.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RecordContainer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RecordContainer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0crecord.proto\"\xa2\x02\n\x06Record\x12\x1b\n\trecord_id\x18\x01\
    \x20\x01(\tR\x08recordId\x12%\n\x06owners\x18\x02\x20\x03(\x0b2\r.Record\
    .OwnerR\x06owners\x12.\n\tlocations\x18\x03\x20\x03(\x0b2\x10.Record.Loc\
    ationR\tlocations\x1a@\n\x05Owner\x12\x19\n\x08agent_id\x18\x01\x20\x01(\
    \tR\x07agentId\x12\x1c\n\ttimestamp\x18\x02\x20\x01(\x04R\ttimestamp\x1a\
    b\n\x08Location\x12\x1a\n\x08latitude\x18\x01\x20\x01(\x12R\x08latitude\
    \x12\x1c\n\tlongitude\x18\x02\x20\x01(\x12R\tlongitude\x12\x1c\n\ttimest\
    amp\x18\x03\x20\x01(\x04R\ttimestamp\"4\n\x0fRecordContainer\x12!\n\x07e\
    ntries\x18\x01\x20\x03(\x0b2\x07.RecordR\x07entriesb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
