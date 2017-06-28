// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CodeGeneratorRequest {
    // message fields
    file_to_generate: ::protobuf::RepeatedField<::std::string::String>,
    parameter: ::protobuf::SingularField<::std::string::String>,
    proto_file: ::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CodeGeneratorRequest {}

impl CodeGeneratorRequest {
    pub fn new() -> CodeGeneratorRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CodeGeneratorRequest {
        static mut instance: ::protobuf::lazy::Lazy<CodeGeneratorRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CodeGeneratorRequest,
        };
        unsafe {
            instance.get(CodeGeneratorRequest::new)
        }
    }

    // repeated string file_to_generate = 1;

    pub fn clear_file_to_generate(&mut self) {
        self.file_to_generate.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_to_generate(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.file_to_generate = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file_to_generate(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.file_to_generate
    }

    // Take field
    pub fn take_file_to_generate(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.file_to_generate, ::protobuf::RepeatedField::new())
    }

    pub fn get_file_to_generate(&self) -> &[::std::string::String] {
        &self.file_to_generate
    }

    fn get_file_to_generate_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.file_to_generate
    }

    fn mut_file_to_generate_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.file_to_generate
    }

    // optional string parameter = 2;

    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    pub fn has_parameter(&self) -> bool {
        self.parameter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parameter(&mut self, v: ::std::string::String) {
        self.parameter = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parameter(&mut self) -> &mut ::std::string::String {
        if self.parameter.is_none() {
            self.parameter.set_default();
        }
        self.parameter.as_mut().unwrap()
    }

    // Take field
    pub fn take_parameter(&mut self) -> ::std::string::String {
        self.parameter.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_parameter(&self) -> &str {
        match self.parameter.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_parameter_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.parameter
    }

    fn mut_parameter_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.parameter
    }

    // repeated .google.protobuf.FileDescriptorProto proto_file = 15;

    pub fn clear_proto_file(&mut self) {
        self.proto_file.clear();
    }

    // Param is passed by value, moved
    pub fn set_proto_file(&mut self, v: ::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto>) {
        self.proto_file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_proto_file(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto> {
        &mut self.proto_file
    }

    // Take field
    pub fn take_proto_file(&mut self) -> ::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto> {
        ::std::mem::replace(&mut self.proto_file, ::protobuf::RepeatedField::new())
    }

    pub fn get_proto_file(&self) -> &[::protobuf::descriptor::FileDescriptorProto] {
        &self.proto_file
    }

    fn get_proto_file_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto> {
        &self.proto_file
    }

    fn mut_proto_file_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::descriptor::FileDescriptorProto> {
        &mut self.proto_file
    }
}

impl ::protobuf::Message for CodeGeneratorRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.proto_file {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.file_to_generate)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parameter)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.proto_file)?;
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
        for value in &self.file_to_generate {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.parameter.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.proto_file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.file_to_generate {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.parameter.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.proto_file {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CodeGeneratorRequest {
    fn new() -> CodeGeneratorRequest {
        CodeGeneratorRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CodeGeneratorRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_to_generate",
                    CodeGeneratorRequest::get_file_to_generate_for_reflect,
                    CodeGeneratorRequest::mut_file_to_generate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parameter",
                    CodeGeneratorRequest::get_parameter_for_reflect,
                    CodeGeneratorRequest::mut_parameter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::descriptor::FileDescriptorProto>>(
                    "proto_file",
                    CodeGeneratorRequest::get_proto_file_for_reflect,
                    CodeGeneratorRequest::mut_proto_file_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CodeGeneratorRequest>(
                    "CodeGeneratorRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CodeGeneratorRequest {
    fn clear(&mut self) {
        self.clear_file_to_generate();
        self.clear_parameter();
        self.clear_proto_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CodeGeneratorRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeGeneratorRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CodeGeneratorResponse {
    // message fields
    error: ::protobuf::SingularField<::std::string::String>,
    file: ::protobuf::RepeatedField<CodeGeneratorResponse_File>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CodeGeneratorResponse {}

impl CodeGeneratorResponse {
    pub fn new() -> CodeGeneratorResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CodeGeneratorResponse {
        static mut instance: ::protobuf::lazy::Lazy<CodeGeneratorResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CodeGeneratorResponse,
        };
        unsafe {
            instance.get(CodeGeneratorResponse::new)
        }
    }

    // optional string error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }

    // repeated .google.protobuf.compiler.CodeGeneratorResponse.File file = 15;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::protobuf::RepeatedField<CodeGeneratorResponse_File>) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&mut self) -> &mut ::protobuf::RepeatedField<CodeGeneratorResponse_File> {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::protobuf::RepeatedField<CodeGeneratorResponse_File> {
        ::std::mem::replace(&mut self.file, ::protobuf::RepeatedField::new())
    }

    pub fn get_file(&self) -> &[CodeGeneratorResponse_File] {
        &self.file
    }

    fn get_file_for_reflect(&self) -> &::protobuf::RepeatedField<CodeGeneratorResponse_File> {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CodeGeneratorResponse_File> {
        &mut self.file
    }
}

impl ::protobuf::Message for CodeGeneratorResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.file {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.file)?;
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
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.file {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CodeGeneratorResponse {
    fn new() -> CodeGeneratorResponse {
        CodeGeneratorResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CodeGeneratorResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    CodeGeneratorResponse::get_error_for_reflect,
                    CodeGeneratorResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CodeGeneratorResponse_File>>(
                    "file",
                    CodeGeneratorResponse::get_file_for_reflect,
                    CodeGeneratorResponse::mut_file_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CodeGeneratorResponse>(
                    "CodeGeneratorResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CodeGeneratorResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CodeGeneratorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeGeneratorResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CodeGeneratorResponse_File {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    insertion_point: ::protobuf::SingularField<::std::string::String>,
    content: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CodeGeneratorResponse_File {}

impl CodeGeneratorResponse_File {
    pub fn new() -> CodeGeneratorResponse_File {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CodeGeneratorResponse_File {
        static mut instance: ::protobuf::lazy::Lazy<CodeGeneratorResponse_File> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CodeGeneratorResponse_File,
        };
        unsafe {
            instance.get(CodeGeneratorResponse_File::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string insertion_point = 2;

    pub fn clear_insertion_point(&mut self) {
        self.insertion_point.clear();
    }

    pub fn has_insertion_point(&self) -> bool {
        self.insertion_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_insertion_point(&mut self, v: ::std::string::String) {
        self.insertion_point = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_insertion_point(&mut self) -> &mut ::std::string::String {
        if self.insertion_point.is_none() {
            self.insertion_point.set_default();
        }
        self.insertion_point.as_mut().unwrap()
    }

    // Take field
    pub fn take_insertion_point(&mut self) -> ::std::string::String {
        self.insertion_point.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_insertion_point(&self) -> &str {
        match self.insertion_point.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_insertion_point_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.insertion_point
    }

    fn mut_insertion_point_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.insertion_point
    }

    // optional string content = 15;

    pub fn clear_content(&mut self) {
        self.content.clear();
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content(&mut self, v: ::std::string::String) {
        self.content = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content(&mut self) -> &mut ::std::string::String {
        if self.content.is_none() {
            self.content.set_default();
        }
        self.content.as_mut().unwrap()
    }

    // Take field
    pub fn take_content(&mut self) -> ::std::string::String {
        self.content.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content(&self) -> &str {
        match self.content.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content
    }

    fn mut_content_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content
    }
}

impl ::protobuf::Message for CodeGeneratorResponse_File {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.insertion_point)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.insertion_point.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.content.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.insertion_point.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.content.as_ref() {
            os.write_string(15, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CodeGeneratorResponse_File {
    fn new() -> CodeGeneratorResponse_File {
        CodeGeneratorResponse_File::new()
    }

    fn descriptor_static(_: ::std::option::Option<CodeGeneratorResponse_File>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CodeGeneratorResponse_File::get_name_for_reflect,
                    CodeGeneratorResponse_File::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "insertion_point",
                    CodeGeneratorResponse_File::get_insertion_point_for_reflect,
                    CodeGeneratorResponse_File::mut_insertion_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content",
                    CodeGeneratorResponse_File::get_content_for_reflect,
                    CodeGeneratorResponse_File::mut_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CodeGeneratorResponse_File>(
                    "CodeGeneratorResponse_File",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CodeGeneratorResponse_File {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_insertion_point();
        self.clear_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CodeGeneratorResponse_File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeGeneratorResponse_File {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%google/protobuf/compiler/plugin.proto\x12\x18google.protobuf.compiler\
    \x1a\x20google/protobuf/descriptor.proto\"\xa3\x01\n\x14CodeGeneratorReq\
    uest\x12(\n\x10file_to_generate\x18\x01\x20\x03(\tR\x0efileToGenerate\
    \x12\x1c\n\tparameter\x18\x02\x20\x01(\tR\tparameter\x12C\n\nproto_file\
    \x18\x0f\x20\x03(\x0b2$.google.protobuf.FileDescriptorProtoR\tprotoFile\
    \"\xd6\x01\n\x15CodeGeneratorResponse\x12\x14\n\x05error\x18\x01\x20\x01\
    (\tR\x05error\x12H\n\x04file\x18\x0f\x20\x03(\x0b24.google.protobuf.comp\
    iler.CodeGeneratorResponse.FileR\x04file\x1a]\n\x04File\x12\x12\n\x04nam\
    e\x18\x01\x20\x01(\tR\x04name\x12'\n\x0finsertion_point\x18\x02\x20\x01(\
    \tR\x0einsertionPoint\x12\x18\n\x07content\x18\x0f\x20\x01(\tR\x07conten\
    tB7\n\x1ccom.google.protobuf.compilerB\x0cPluginProtosZ\tplugin_goJ\x89;\
    \n\x07\x12\x05.\0\x95\x01\x01\n\xca\x11\n\x01\x0c\x12\x03.\0\x122\xc1\
    \x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\x20interchange\x20f\
    ormat\n\x20Copyright\x202008\x20Google\x20Inc.\x20\x20All\x20rights\x20r\
    eserved.\n\x20https://developers.google.com/protocol-buffers/\n\n\x20Red\
    istribution\x20and\x20use\x20in\x20source\x20and\x20binary\x20forms,\x20\
    with\x20or\x20without\n\x20modification,\x20are\x20permitted\x20provided\
    \x20that\x20the\x20following\x20conditions\x20are\n\x20met:\n\n\x20\x20\
    \x20\x20\x20*\x20Redistributions\x20of\x20source\x20code\x20must\x20reta\
    in\x20the\x20above\x20copyright\n\x20notice,\x20this\x20list\x20of\x20co\
    nditions\x20and\x20the\x20following\x20disclaimer.\n\x20\x20\x20\x20\x20\
    *\x20Redistributions\x20in\x20binary\x20form\x20must\x20reproduce\x20the\
    \x20above\n\x20copyright\x20notice,\x20this\x20list\x20of\x20conditions\
    \x20and\x20the\x20following\x20disclaimer\n\x20in\x20the\x20documentatio\
    n\x20and/or\x20other\x20materials\x20provided\x20with\x20the\n\x20distri\
    bution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20name\x20of\x20Google\
    \x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\x20contributors\x20may\
    \x20be\x20used\x20to\x20endorse\x20or\x20promote\x20products\x20derived\
    \x20from\n\x20this\x20software\x20without\x20specific\x20prior\x20writte\
    n\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\
    \x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\
    \x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\
    \x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MER\
    CHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\
    \x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\
    \n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIREC\
    T,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONS\
    EQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\
    \x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\
    \x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINESS\x20INTERRU\
    PTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIA\
    BILITY,\x20WHETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILITY,\x20OR\x20T\
    ORT\n\x20(INCLUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\
    \x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\
    \x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20SUCH\x20D\
    AMAGE.\n2\xfb\x04\x20Author:\x20kenton@google.com\x20(Kenton\x20Varda)\n\
    \n\x20WARNING:\x20\x20The\x20plugin\x20interface\x20is\x20currently\x20E\
    XPERIMENTAL\x20and\x20is\x20subject\x20to\n\x20\x20\x20change.\n\n\x20pr\
    otoc\x20(aka\x20the\x20Protocol\x20Compiler)\x20can\x20be\x20extended\
    \x20via\x20plugins.\x20\x20A\x20plugin\x20is\n\x20just\x20a\x20program\
    \x20that\x20reads\x20a\x20CodeGeneratorRequest\x20from\x20stdin\x20and\
    \x20writes\x20a\n\x20CodeGeneratorResponse\x20to\x20stdout.\n\n\x20Plugi\
    ns\x20written\x20using\x20C++\x20can\x20use\x20google/protobuf/compiler/\
    plugin.h\x20instead\n\x20of\x20dealing\x20with\x20the\x20raw\x20protocol\
    \x20defined\x20here.\n\n\x20A\x20plugin\x20executable\x20needs\x20only\
    \x20to\x20be\x20placed\x20somewhere\x20in\x20the\x20path.\x20\x20The\n\
    \x20plugin\x20should\x20be\x20named\x20\"protoc-gen-$NAME\",\x20and\x20w\
    ill\x20then\x20be\x20used\x20when\x20the\n\x20flag\x20\"--${NAME}_out\"\
    \x20is\x20passed\x20to\x20protoc.\n\n\x08\n\x01\x02\x12\x03/\x08\x20\n\
    \x08\n\x01\x08\x12\x030\05\n\x0b\n\x04\x08\xe7\x07\0\x12\x030\05\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x030\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x030\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x030\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x030\x164\n\x08\n\x01\x08\x12\x031\0-\n\
    \x0b\n\x04\x08\xe7\x07\x01\x12\x031\0-\n\x0c\n\x05\x08\xe7\x07\x01\x02\
    \x12\x031\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x031\x07\x1b\n\x0e\
    \n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x031\x07\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x01\x07\x12\x031\x1e,\n\x08\n\x01\x08\x12\x033\0\x20\n\x0b\n\x04\
    \x08\xe7\x07\x02\x12\x033\0\x20\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x033\
    \x07\x11\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x033\x07\x11\n\x0e\n\x07\
    \x08\xe7\x07\x02\x02\0\x01\x12\x033\x07\x11\n\x0c\n\x05\x08\xe7\x07\x02\
    \x07\x12\x033\x14\x1f\n\t\n\x02\x03\0\x12\x035\x07)\nO\n\x02\x04\0\x12\
    \x048\0M\x01\x1aC\x20An\x20encoded\x20CodeGeneratorRequest\x20is\x20writ\
    ten\x20to\x20the\x20plugin's\x20stdin.\n\n\n\n\x03\x04\0\x01\x12\x038\
    \x08\x1c\n\xd1\x01\n\x04\x04\0\x02\0\x12\x03<\x02'\x1a\xc3\x01\x20The\
    \x20.proto\x20files\x20that\x20were\x20explicitly\x20listed\x20on\x20the\
    \x20command-line.\x20\x20The\n\x20code\x20generator\x20should\x20generat\
    e\x20code\x20only\x20for\x20these\x20files.\x20\x20Each\x20file's\n\x20d\
    escriptor\x20will\x20be\x20included\x20in\x20proto_file,\x20below.\n\n\
    \x0c\n\x05\x04\0\x02\0\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03<\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03<\x12\"\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03<%&\nB\n\x04\x04\0\x02\x01\x12\x03?\x02\x20\x1a5\
    \x20The\x20generator\x20parameter\x20passed\x20on\x20the\x20command-line\
    .\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03?\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03?\x12\x1b\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03?\x1e\x1f\n\xa9\x05\n\x04\x04\0\
    \x02\x02\x12\x03L\x02/\x1a\x9b\x05\x20FileDescriptorProtos\x20for\x20all\
    \x20files\x20in\x20files_to_generate\x20and\x20everything\n\x20they\x20i\
    mport.\x20\x20The\x20files\x20will\x20appear\x20in\x20topological\x20ord\
    er,\x20so\x20each\x20file\n\x20appears\x20before\x20any\x20file\x20that\
    \x20imports\x20it.\n\n\x20protoc\x20guarantees\x20that\x20all\x20proto_f\
    iles\x20will\x20be\x20written\x20after\n\x20the\x20fields\x20above,\x20e\
    ven\x20though\x20this\x20is\x20not\x20technically\x20guaranteed\x20by\
    \x20the\n\x20protobuf\x20wire\x20format.\x20\x20This\x20theoretically\
    \x20could\x20allow\x20a\x20plugin\x20to\x20stream\n\x20in\x20the\x20File\
    DescriptorProtos\x20and\x20handle\x20them\x20one\x20by\x20one\x20rather\
    \x20than\x20read\n\x20the\x20entire\x20set\x20into\x20memory\x20at\x20on\
    ce.\x20\x20However,\x20as\x20of\x20this\x20writing,\x20this\n\x20is\x20n\
    ot\x20similarly\x20optimized\x20on\x20protoc's\x20end\x20--\x20it\x20wil\
    l\x20store\x20all\x20fields\x20in\n\x20memory\x20at\x20once\x20before\
    \x20sending\x20them\x20to\x20the\x20plugin.\n\n\x0c\n\x05\x04\0\x02\x02\
    \x04\x12\x03L\x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03L\x0b\x1e\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03L\x1f)\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03L,.\nL\n\x02\x04\x01\x12\x05P\0\x95\x01\x01\x1a?\x20The\x20plugin\
    \x20writes\x20an\x20encoded\x20CodeGeneratorResponse\x20to\x20stdout.\n\
    \n\n\n\x03\x04\x01\x01\x12\x03P\x08\x1d\n\xed\x03\n\x04\x04\x01\x02\0\
    \x12\x03Y\x02\x1c\x1a\xdf\x03\x20Error\x20message.\x20\x20If\x20non-empt\
    y,\x20code\x20generation\x20failed.\x20\x20The\x20plugin\x20process\n\
    \x20should\x20exit\x20with\x20status\x20code\x20zero\x20even\x20if\x20it\
    \x20reports\x20an\x20error\x20in\x20this\x20way.\n\n\x20This\x20should\
    \x20be\x20used\x20to\x20indicate\x20errors\x20in\x20.proto\x20files\x20w\
    hich\x20prevent\x20the\n\x20code\x20generator\x20from\x20generating\x20c\
    orrect\x20code.\x20\x20Errors\x20which\x20indicate\x20a\n\x20problem\x20\
    in\x20protoc\x20itself\x20--\x20such\x20as\x20the\x20input\x20CodeGenera\
    torRequest\x20being\n\x20unparseable\x20--\x20should\x20be\x20reported\
    \x20by\x20writing\x20a\x20message\x20to\x20stderr\x20and\n\x20exiting\
    \x20with\x20a\x20non-zero\x20status\x20code.\n\n\x0c\n\x05\x04\x01\x02\0\
    \x04\x12\x03Y\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03Y\x0b\x11\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03Y\x12\x17\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03Y\x1a\x1b\n4\n\x04\x04\x01\x03\0\x12\x05\\\x02\x93\x01\x03\x1a%\
    \x20Represents\x20a\x20single\x20generated\x20file.\n\n\x0c\n\x05\x04\
    \x01\x03\0\x01\x12\x03\\\n\x0e\n\xad\x05\n\x06\x04\x01\x03\0\x02\0\x12\
    \x03h\x04\x1d\x1a\x9d\x05\x20The\x20file\x20name,\x20relative\x20to\x20t\
    he\x20output\x20directory.\x20\x20The\x20name\x20must\x20not\n\x20contai\
    n\x20\".\"\x20or\x20\"..\"\x20components\x20and\x20must\x20be\x20relativ\
    e,\x20not\x20be\x20absolute\x20(so,\n\x20the\x20file\x20cannot\x20lie\
    \x20outside\x20the\x20output\x20directory).\x20\x20\"/\"\x20must\x20be\
    \x20used\x20as\n\x20the\x20path\x20separator,\x20not\x20\"\\\".\n\n\x20I\
    f\x20the\x20name\x20is\x20omitted,\x20the\x20content\x20will\x20be\x20ap\
    pended\x20to\x20the\x20previous\n\x20file.\x20\x20This\x20allows\x20the\
    \x20generator\x20to\x20break\x20large\x20files\x20into\x20small\x20chunk\
    s,\n\x20and\x20allows\x20the\x20generated\x20text\x20to\x20be\x20streame\
    d\x20back\x20to\x20protoc\x20so\x20that\x20large\n\x20files\x20need\x20n\
    ot\x20reside\x20completely\x20in\x20memory\x20at\x20one\x20time.\x20\x20\
    Note\x20that\x20as\x20of\n\x20this\x20writing\x20protoc\x20does\x20not\
    \x20optimize\x20for\x20this\x20--\x20it\x20will\x20read\x20the\x20entire\
    \n\x20CodeGeneratorResponse\x20before\x20writing\x20files\x20to\x20disk.\
    \n\n\x0e\n\x07\x04\x01\x03\0\x02\0\x04\x12\x03h\x04\x0c\n\x0e\n\x07\x04\
    \x01\x03\0\x02\0\x05\x12\x03h\r\x13\n\x0e\n\x07\x04\x01\x03\0\x02\0\x01\
    \x12\x03h\x14\x18\n\x0e\n\x07\x04\x01\x03\0\x02\0\x03\x12\x03h\x1b\x1c\n\
    \xae\x10\n\x06\x04\x01\x03\0\x02\x01\x12\x04\x8f\x01\x04(\x1a\x9d\x10\
    \x20If\x20non-empty,\x20indicates\x20that\x20the\x20named\x20file\x20sho\
    uld\x20already\x20exist,\x20and\x20the\n\x20content\x20here\x20is\x20to\
    \x20be\x20inserted\x20into\x20that\x20file\x20at\x20a\x20defined\x20inse\
    rtion\n\x20point.\x20\x20This\x20feature\x20allows\x20a\x20code\x20gener\
    ator\x20to\x20extend\x20the\x20output\n\x20produced\x20by\x20another\x20\
    code\x20generator.\x20\x20The\x20original\x20generator\x20may\x20provide\
    \n\x20insertion\x20points\x20by\x20placing\x20special\x20annotations\x20\
    in\x20the\x20file\x20that\x20look\n\x20like:\n\x20\x20\x20@@protoc_inser\
    tion_point(NAME)\n\x20The\x20annotation\x20can\x20have\x20arbitrary\x20t\
    ext\x20before\x20and\x20after\x20it\x20on\x20the\x20line,\n\x20which\x20\
    allows\x20it\x20to\x20be\x20placed\x20in\x20a\x20comment.\x20\x20NAME\
    \x20should\x20be\x20replaced\x20with\n\x20an\x20identifier\x20naming\x20\
    the\x20point\x20--\x20this\x20is\x20what\x20other\x20generators\x20will\
    \x20use\n\x20as\x20the\x20insertion_point.\x20\x20Code\x20inserted\x20at\
    \x20this\x20point\x20will\x20be\x20placed\n\x20immediately\x20above\x20t\
    he\x20line\x20containing\x20the\x20insertion\x20point\x20(thus\x20multip\
    le\n\x20insertions\x20to\x20the\x20same\x20point\x20will\x20come\x20out\
    \x20in\x20the\x20order\x20they\x20were\x20added).\n\x20The\x20double-@\
    \x20is\x20intended\x20to\x20make\x20it\x20unlikely\x20that\x20the\x20gen\
    erated\x20code\n\x20could\x20contain\x20things\x20that\x20look\x20like\
    \x20insertion\x20points\x20by\x20accident.\n\n\x20For\x20example,\x20the\
    \x20C++\x20code\x20generator\x20places\x20the\x20following\x20line\x20in\
    \x20the\n\x20.pb.h\x20files\x20that\x20it\x20generates:\n\x20\x20\x20//\
    \x20@@protoc_insertion_point(namespace_scope)\n\x20This\x20line\x20appea\
    rs\x20within\x20the\x20scope\x20of\x20the\x20file's\x20package\x20namesp\
    ace,\x20but\n\x20outside\x20of\x20any\x20particular\x20class.\x20\x20Ano\
    ther\x20plugin\x20can\x20then\x20specify\x20the\n\x20insertion_point\x20\
    \"namespace_scope\"\x20to\x20generate\x20additional\x20classes\x20or\n\
    \x20other\x20declarations\x20that\x20should\x20be\x20placed\x20in\x20thi\
    s\x20scope.\n\n\x20Note\x20that\x20if\x20the\x20line\x20containing\x20th\
    e\x20insertion\x20point\x20begins\x20with\n\x20whitespace,\x20the\x20sam\
    e\x20whitespace\x20will\x20be\x20added\x20to\x20every\x20line\x20of\x20t\
    he\n\x20inserted\x20text.\x20\x20This\x20is\x20useful\x20for\x20language\
    s\x20like\x20Python,\x20where\n\x20indentation\x20matters.\x20\x20In\x20\
    these\x20languages,\x20the\x20insertion\x20point\x20comment\n\x20should\
    \x20be\x20indented\x20the\x20same\x20amount\x20as\x20any\x20inserted\x20\
    code\x20will\x20need\x20to\x20be\n\x20in\x20order\x20to\x20work\x20corre\
    ctly\x20in\x20that\x20context.\n\n\x20The\x20code\x20generator\x20that\
    \x20generates\x20the\x20initial\x20file\x20and\x20the\x20one\x20which\n\
    \x20inserts\x20into\x20it\x20must\x20both\x20run\x20as\x20part\x20of\x20\
    a\x20single\x20invocation\x20of\x20protoc.\n\x20Code\x20generators\x20ar\
    e\x20executed\x20in\x20the\x20order\x20in\x20which\x20they\x20appear\x20\
    on\x20the\n\x20command\x20line.\n\n\x20If\x20|insertion_point|\x20is\x20\
    present,\x20|name|\x20must\x20also\x20be\x20present.\n\n\x0f\n\x07\x04\
    \x01\x03\0\x02\x01\x04\x12\x04\x8f\x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\0\
    \x02\x01\x05\x12\x04\x8f\x01\r\x13\n\x0f\n\x07\x04\x01\x03\0\x02\x01\x01\
    \x12\x04\x8f\x01\x14#\n\x0f\n\x07\x04\x01\x03\0\x02\x01\x03\x12\x04\x8f\
    \x01&'\n$\n\x06\x04\x01\x03\0\x02\x02\x12\x04\x92\x01\x04!\x1a\x14\x20Th\
    e\x20file\x20contents.\n\n\x0f\n\x07\x04\x01\x03\0\x02\x02\x04\x12\x04\
    \x92\x01\x04\x0c\n\x0f\n\x07\x04\x01\x03\0\x02\x02\x05\x12\x04\x92\x01\r\
    \x13\n\x0f\n\x07\x04\x01\x03\0\x02\x02\x01\x12\x04\x92\x01\x14\x1b\n\x0f\
    \n\x07\x04\x01\x03\0\x02\x02\x03\x12\x04\x92\x01\x1e\x20\n\x0c\n\x04\x04\
    \x01\x02\x01\x12\x04\x94\x01\x02\x1a\n\r\n\x05\x04\x01\x02\x01\x04\x12\
    \x04\x94\x01\x02\n\n\r\n\x05\x04\x01\x02\x01\x06\x12\x04\x94\x01\x0b\x0f\
    \n\r\n\x05\x04\x01\x02\x01\x01\x12\x04\x94\x01\x10\x14\n\r\n\x05\x04\x01\
    \x02\x01\x03\x12\x04\x94\x01\x17\x19\
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
