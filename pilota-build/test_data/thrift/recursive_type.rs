pub mod recursive_type {
    #![allow(warnings, clippy::all)]

    pub mod recursive_type {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub a: ::std::option::Option<::std::boxed::Box<A>>,

            pub a_b: ::std::option::Option<::std::boxed::Box<B>>,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                }
                if let Some(value) = self.a_b.as_ref() {
                    protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;
                let mut a_b = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                a = Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                ));
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                a_b = Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                ));
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { a, a_b };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;
                    let mut a_b = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let ::std::result::Result::Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    a = Some(::std::boxed::Box::new(
                                        <A as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    ));
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    a_b = Some(::std::boxed::Box::new(
                                        <B as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    ));
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { a, a_b };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.struct_field_len(Some(1), value))
                    + self
                        .a_b
                        .as_ref()
                        .map_or(0, |value| protocol.struct_field_len(Some(2), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct C {
            pub c: ::std::option::Option<::pilota::AHashSet<::pilota::FastStr>>,
        }
        impl ::pilota::thrift::Message for C {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "C" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.c.as_ref() {
                    protocol.write_set_field(
                        1,
                        ::pilota::thrift::TType::Binary,
                        &value,
                        |protocol, val| {
                            protocol.write_faststr((val).clone())?;
                            ::std::result::Result::Ok(())
                        },
                    )?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut c = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::Set => {
                                c = Some({
                                    let list_ident = protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(protocol.read_faststr()?);
                                    }
                                    protocol.read_set_end()?;
                                    val
                                });
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `C` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { c };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut c = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let ::std::result::Result::Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::Set =>
                                {
                                    c = Some({
                                        let list_ident = protocol.read_set_begin().await?;
                                        let mut val =
                                            ::pilota::AHashSet::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.insert(protocol.read_faststr().await?);
                                        }
                                        protocol.read_set_end().await?;
                                        val
                                    });
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `C` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { c };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "C" })
                    + self.c.as_ref().map_or(0, |value| {
                        protocol.set_field_len(
                            Some(1),
                            ::pilota::thrift::TType::Binary,
                            value,
                            |protocol, el| protocol.faststr_len(el),
                        )
                    })
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub b_a: ::std::option::Option<::std::boxed::Box<A>>,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "B" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.b_a.as_ref() {
                    protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                ::std::result::Result::Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut b_a = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let ::std::result::Result::Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                b_a = Some(::std::boxed::Box::new(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                ));
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `B` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return ::std::result::Result::Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { b_a };
                ::std::result::Result::Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut b_a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let ::std::result::Result::Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    b_a = Some(::std::boxed::Box::new(
                                        <A as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    ));
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        ::std::result::Result::Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `B` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return ::std::result::Result::Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self { b_a };
                    ::std::result::Result::Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "B" })
                    + self
                        .b_a
                        .as_ref()
                        .map_or(0, |value| protocol.struct_field_len(Some(1), value))
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
