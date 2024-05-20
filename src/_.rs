extern crate protobuf_upb as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct MyMessage {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for MyMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(std::any::type_name::<Self>())
      .field("raw_msg", &self.raw_msg())
      .finish()
  }
}

impl std::default::Default for MyMessage {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `MyMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `MyMessageMut`.
unsafe impl Sync for MyMessage {}

// SAFETY:
// - `MyMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for MyMessage {}

impl ::__pb::Proxied for MyMessage {
  type View<'msg> = MyMessageView<'msg>;
  type Mut<'msg> = MyMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MyMessageView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for MyMessageView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(std::any::type_name::<Self>())
      .field("raw_msg", &self.raw_msg())
      .finish()
  }
}

#[allow(dead_code)]
impl<'msg> MyMessageView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    let arena = ::__pb::__runtime::Arena::new();
    // SAFETY: MyMessage_msg_init is a static of a const object.
    let mini_table = unsafe { ::__std::ptr::addr_of!(MyMessage_msg_init) };
    let options = 0;
    let mut buf: *mut u8 = std::ptr::null_mut();
    let mut len = 0;

    // SAFETY: `mini_table` is the corresponding one that was used to
    // construct `self.raw_msg()`.
    let status = unsafe {
      ::__pb::__runtime::upb_Encode(self.raw_msg(), mini_table, options, arena.raw(),
          &mut buf, &mut len)
    };

    assert!(status == ::__pb::__runtime::EncodeStatus::Ok);
    let data = if len == 0 {
      std::ptr::NonNull::dangling()
    } else {
      std::ptr::NonNull::new(buf).unwrap()
    };

    // SAFETY:
    // - `arena` allocated `data`.
    // - `data` is valid for reads up to `len` and will not be mutated.
    unsafe {
      ::__pb::__runtime::SerializedData::from_raw_parts(arena, data, len)
    }
  }

  // id: optional int32
  pub fn id(self) -> i32 {
    unsafe { MyMessage_id(self.raw_msg()) }
  }

  // name: optional string
  pub fn name(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { MyMessage_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `MyMessageView` is `Sync` because it does not support mutation.
unsafe impl Sync for MyMessageView<'_> {}

// SAFETY:
// - `MyMessageView` is `Send` because while its alive a `MyMessageMut` cannot.
// - `MyMessageView` does not use thread-local data.
unsafe impl Send for MyMessageView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for MyMessageView<'msg> {
  type Proxied = MyMessage;

  fn as_view(&self) -> ::__pb::View<'msg, MyMessage> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, MyMessage> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for MyMessage {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    MyMessageView::new(::__pb::__internal::Private, msg.unwrap_or_else(||::__pb::__runtime::ScratchSpace::zeroed_block(::__pb::__internal::Private)))
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg(), inner.msg_ref().arena(::__pb::__internal::Private).raw())
    };
    MyMessageMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for MyMessage {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for MyMessage {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, MyMessage>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, MyMessage>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<MyMessage> for MyMessageView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, MyMessage>)
    where MyMessage: 'dst {
    unsafe { ::__pb::__runtime::upb_Message_DeepCopy(
      mutator.inner.msg(),
      self.msg,
      ::__std::ptr::addr_of!(MyMessage_msg_init),
      mutator.inner.arena(::__pb::__internal::Private).raw(),
    ) };
  }
}

impl ::__pb::SettableValue<MyMessage> for MyMessage {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, MyMessage>)
    where MyMessage: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for MyMessage {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `upb_Array*`.
    unsafe { ::__pb::__runtime::upb_Array_Size(f.as_raw(::__pb::__internal::Private)) }
  }
  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    // - `i < len(f)` is promised by the caller.
    let dest_msg = unsafe {
      ::__pb::__runtime::upb_Array_GetMutable(f.as_raw(::__pb::__internal::Private), i).msg
    }.expect("upb_Array* element should not be NULL");

    // SAFETY:
    // - `dest_msg` is a valid `upb_Message*`.
    // - `v.raw_msg()` and `dest_msg` both have message minitable `MyMessage_msg_init`.
    unsafe {
      ::__pb::__runtime::upb_Message_DeepCopy(
        dest_msg,
        v.raw_msg(),
        ::__std::ptr::addr_of!(MyMessage_msg_init),
        f.raw_arena(::__pb::__internal::Private),
      )
    };
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const upb_Array*`.
    // - `i < len(f)` is promised by the caller.
    let msg_ptr = unsafe { ::__pb::__runtime::upb_Array_Get(f.as_raw(::__pb::__internal::Private), i).msg_val }
      .expect("upb_Array* element should not be NULL.");
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg_ptr)
  }

  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    unsafe {
      ::__pb::__runtime::upb_Array_Resize(f.as_raw(::__pb::__internal::Private), 0, f.raw_arena(::__pb::__internal::Private))
    };
  }
  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `v.raw_msg()` is a valid `const upb_Message*` with minitable `MyMessage_msg_init`.
    let msg_ptr = unsafe {
      ::__pb::__runtime::upb_Message_DeepClone(
        v.raw_msg(),
        std::ptr::addr_of!(MyMessage_msg_init),
        f.raw_arena(::__pb::__internal::Private),
      )
    }.expect("upb_Message_DeepClone failed.");

    // Append new default message to array.
    // SAFETY:
    // - `f.as_raw()` is a valid `upb_Array*`.
    // - `msg_ptr` is a valid `upb_Message*`.
    unsafe {
      ::__pb::__runtime::upb_Array_Append(
        f.as_raw(::__pb::__internal::Private),
        ::__pb::__runtime::upb_MessageValue{msg_val: Some(msg_ptr)},
        f.raw_arena(::__pb::__internal::Private),
      );
    };
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
      // SAFETY:
      // - Elements of `src` and `dest` have message minitable `MyMessage_msg_init`.
      unsafe {
        ::__pb::__runtime::repeated_message_copy_from(src, dest, ::__std::ptr::addr_of!(MyMessage_msg_init));
      }
  }
}
impl ::__pb::__runtime::UpbTypeConversions for MyMessage {
    fn upb_type() -> ::__pb::__runtime::CType {
        ::__pb::__runtime::CType::Message
    }

    fn to_message_value(
        val: ::__pb::View<'_, Self>) -> ::__pb::__runtime::upb_MessageValue {
        ::__pb::__runtime::upb_MessageValue { msg_val: Some(val.raw_msg()) }
    }

    unsafe fn to_message_value_copy_if_required(
      arena: ::__pb::__runtime::RawArena,
      val: ::__pb::View<'_, Self>) -> ::__pb::__runtime::upb_MessageValue {
      // Self::to_message_value(val)
      // SAFETY: The arena memory is not freed due to `ManuallyDrop`.
      let cloned_msg = ::__pb::__runtime::upb_Message_DeepClone(
          val.raw_msg(), ::__std::ptr::addr_of!(MyMessage_msg_init), arena)
          .expect("upb_Message_DeepClone failed.");
      Self::to_message_value(
          MyMessageView::new(::__pb::__internal::Private, cloned_msg))
      }

    unsafe fn from_message_value<'msg>(msg: ::__pb::__runtime::upb_MessageValue)
        -> ::__pb::View<'msg, Self> {
        MyMessageView::new(
            ::__pb::__internal::Private,
            unsafe { msg.msg_val }
                .expect("expected present message value in map"))
    }
}
impl ::__pb::ProxiedInMapValue<i32> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <i32 as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<i32, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <i32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <i32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <i32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, i32, Self>
    ) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <i32 as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}
impl ::__pb::ProxiedInMapValue<u32> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <u32 as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<u32, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <u32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <u32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <u32 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, u32, Self>
    ) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <u32 as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}
impl ::__pb::ProxiedInMapValue<i64> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <i64 as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<i64, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <i64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <i64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <i64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, i64, Self>
    ) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <i64 as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}
impl ::__pb::ProxiedInMapValue<u64> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <u64 as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<u64, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <u64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <u64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <u64 as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, u64, Self>
    ) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <u64 as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}
impl ::__pb::ProxiedInMapValue<bool> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <bool as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<bool, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <bool as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <bool as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <bool as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, bool, Self>
    ) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <bool as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for MyMessage {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        let arena = ::__pb::__runtime::Arena::new();
        let raw = unsafe {
          ::__pb::__runtime::upb_Map_New(
            arena.raw(),
            <::__pb::ProtoStr as ::__pb::__runtime::UpbTypeConversions>::upb_type(),
            <Self as ::__pb::__runtime::UpbTypeConversions>::upb_type())
        };

        ::__pb::Map::from_inner(
            ::__pb::__internal::Private,
            ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, raw, arena))
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, _map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        // No-op: the memory will be dropped by the arena.
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe {
            ::__pb::__runtime::upb_Map_Clear(map.as_raw(::__pb::__internal::Private));
        }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe {
            ::__pb::__runtime::upb_Map_Size(map.as_raw(::__pb::__internal::Private))
        }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        let arena = map.inner(::__pb::__internal::Private).raw_arena(::__pb::__internal::Private);
        unsafe {
            ::__pb::__runtime::upb_Map_InsertAndReturnIfInserted(
                map.as_raw(::__pb::__internal::Private),
                <::__pb::ProtoStr as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                <Self as ::__pb::__runtime::UpbTypeConversions>::to_message_value_copy_if_required(arena, value),
                arena
            )
        }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let mut val = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe {
            ::__pb::__runtime::upb_Map_Get(
                map.as_raw(::__pb::__internal::Private),
                <::__pb::ProtoStr as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                val.as_mut_ptr())
        };
        if !found {
            return None;
        }
        Some(unsafe { <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(val.assume_init()) })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        unsafe {
            ::__pb::__runtime::upb_Map_Delete(
                map.as_raw(::__pb::__internal::Private),
                <::__pb::ProtoStr as ::__pb::__runtime::UpbTypeConversions>::to_message_value(key),
                ::__std::ptr::null_mut())
        }
    }
    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY: View<Map<'_,..>> guarantees its RawMap outlives '_.
        unsafe {
            ::__pb::MapIter::from_raw(::__pb::__internal::Private, ::__pb::__runtime::RawMapIter::new(::__pb::__internal::Private, map.as_raw(::__pb::__internal::Private)))
        }
    }

    fn map_iter_next<'a>(
        iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>
    ) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY: MapIter<'a, ..> guarantees its RawMapIter outlives 'a.
        unsafe { iter.as_raw_mut(::__pb::__internal::Private).next_unchecked(::__pb::__internal::Private) }
            // SAFETY: MapIter<K, V> returns key and values message values
            //         with the variants for K and V active.
            .map(|(k, v)| unsafe {(
                <::__pb::ProtoStr as ::__pb::__runtime::UpbTypeConversions>::from_message_value(k),
                <Self as ::__pb::__runtime::UpbTypeConversions>::from_message_value(v),
            )})
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MyMessageMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for MyMessageMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(std::any::type_name::<Self>())
      .field("raw_msg", &self.raw_msg())
      .finish()
  }
}

#[allow(dead_code)]
impl<'msg> MyMessageMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    self.inner.arena(::__pb::__internal::Private)
  }

  // id: optional int32
  pub fn id(&self) -> i32 {
    unsafe { MyMessage_id(self.raw_msg()) }
  }
  pub fn set_id(&mut self, val: i32) {
    unsafe { MyMessage_set_id(self.raw_msg(), val) }
  }

  // name: optional string
  pub fn name(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { MyMessage_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_name(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.name_mut().set(val);
  }
  fn name_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          MyMessage::__NAME_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `MyMessageMut` does not perform any shared mutation.
// - `MyMessageMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for MyMessageMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for MyMessageMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, MyMessage> {
    MyMessageMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, MyMessage> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for MyMessageMut<'msg> {
  type Proxied = MyMessage;
  fn as_view(&self) -> ::__pb::View<'_, MyMessage> {
    MyMessageView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, MyMessage> where 'msg: 'shorter {
    MyMessageView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl MyMessage {
  pub fn new() -> Self {
    let arena = ::__pb::__runtime::Arena::new();
    Self {
      inner: ::__pb::__runtime::MessageInner {
        msg: unsafe { MyMessage_new(arena.raw()) },
        arena,
      }
    }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }

  fn arena(&self) -> &::__pb::__runtime::Arena {
    &self.inner.arena
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let mut msg = Self::new();
    // SAFETY: MyMessage_msg_init is a static of a const object.
    let mini_table = unsafe { ::__std::ptr::addr_of!(MyMessage_msg_init) };
    let ext_reg = std::ptr::null();
    let options = 0;

    // SAFETY:
    // - `data.as_ptr()` is valid to read for `data.len()`
    // - `mini_table` is the one used to construct `msg.raw_msg()`
    // - `msg.arena().raw()` is held for the same lifetime as `msg`.
    let status = unsafe {
      ::__pb::__runtime::upb_Decode(
          data.as_ptr(), data.len(), msg.raw_msg(),
          mini_table, ext_reg, options, msg.arena().raw())
    };
    match status {
      ::__pb::__runtime::DecodeStatus::Ok => {
        std::mem::swap(self, &mut msg);
        Ok(())
      }
      _ => Err(::__pb::ParseError)
    }
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> MyMessageView {
    MyMessageView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> MyMessageMut {
    MyMessageMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // id: optional int32
  pub fn id(&self) -> i32 {
    unsafe { MyMessage_id(self.raw_msg()) }
  }
  pub fn set_id(&mut self, val: i32) {
    unsafe { MyMessage_set_id(self.raw_msg(), val) }
  }

  // name: optional string
  pub fn name(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { MyMessage_name(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_name(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.name_mut().set(val);
  }
  const __NAME_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      MyMessage_name,
      MyMessage_set_name,
    );
  fn name_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          MyMessage::__NAME_VTABLE,
        )
      )
    }
  }

}  // impl MyMessage

impl ::__std::ops::Drop for MyMessage {
  fn drop(&mut self) {
  }
}

extern "C" {
  fn MyMessage_new(arena: ::__pb::__runtime::RawArena) -> ::__pb::__runtime::RawMessage;
  /// Opaque wrapper for this message's MiniTable. The only valid way to
  /// reference this static is with `std::ptr::addr_of!(..)`.
  static MyMessage_msg_init: ::__pb::__runtime::upb_MiniTable;

  fn MyMessage_id(raw_msg: ::__pb::__runtime::RawMessage) -> i32;
  fn MyMessage_set_id(raw_msg: ::__pb::__runtime::RawMessage, val: i32);

  fn MyMessage_name(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn MyMessage_set_name(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for MyMessage


