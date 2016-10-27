/*!
Automate COM plumbing.

These macros automate the required plumbing to make everything work nicely.

Some work could be done to improve their ergonomics: allow trailing commas, attributes without keywords, ...

GUID macros
-----------

Define GUIDs with almost native GUID syntax.

Every number group is required to be prefixed by `0x` to indicate they're hexadecimal.

* [`guid!`](../macro.guid!.html)`{0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE}`

  Creates a `GUID` instance.

* [`static_guid!`](../macro.static_guid!.html)`(STATIC_GUID, {0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE});`

  Expands to a static definition so you avoid having to name its type.

COM FFI macros
--------------

These help with plumbing the required FFI structs and associated traits.

* [`com_vtbl!`](../macro.com_vtbl!.html)

  Defines and implements a vtbl struct.

  ```
  # #[macro_use] extern crate com_core; fn main() {
  use ::com_core::unknown::{IUnknownVtbl};

  com_vtbl! {
        IInterfaceVtbl: IUnknownVtbl,
        IInterface,
        pub Foo: unsafe extern "stdcall" fn(
            This: *mut IInterface,
        ),
  }
  # }
  ```

* [`com_interface!`](../macro.com_interface!.html)

  Defines and implements an interface struct and its vtbl struct.

  ```
  # #[macro_use] extern crate com_core; fn main() {
  use ::com_core::unknown::{IUnknown, IUnknownVtbl};

  com_interface! {
        interface IInterface(IInterfaceVtbl): IUnknown(IUnknownVtbl);
        {0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE}
        pub Foo: unsafe extern "stdcall" fn(
            This: *mut IInterface,
        ),
  }
  # }
  ```

* [`com_class!`](../macro.com_class!.html)`(_extern Class, {0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE}, IInterface);`

  Defines a placeholder class (an empty enum), its class id and the interfaces it implements.
*/

/// Constructs a GUID.
///
/// See [`static_guid!`](macro.static_guid!.html) to define a named static GUID.
/// This avoids having to specify the `GUID` type, it just works(tm).
///
/// Note that you need to prefix the numbers with `0x` to indicate they're hexadecimal.
/// Limitations in `macro_rules!` prevent this being done automatically.
///
/// GUIDs can appear as part of other macros in this crate, they are all expected to have their numbers prefixed with `0x`.
#[macro_export]
macro_rules! guid {
	($a:tt-$b:tt-$c:tt-$d:tt-$e:tt) => {
		$crate::GUID {
			Data1: $a,
			Data2: $b,
			Data3: $c,
			Data4: [
				($d as u16 >> 8) as u8,
				($d as u16 >> 0) as u8,
				($e as u64 >> 40) as u8,
				($e as u64 >> 32) as u8,
				($e as u64 >> 24) as u8,
				($e as u64 >> 16) as u8,
				($e as u64 >> 8) as u8,
				($e as u64 >> 0) as u8,
			],
		}
	};
}

/// Defines a named static GUID.
///
/// Note that you need to prefix the numbers with `0x` to indicate they're hexadecimal.
/// Limitations in `macro_rules!` prevent this being done automatically.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate com_core; fn main() {
/// static_guid!(STATIC_GUID, {0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE});
/// 
/// // This would be equal if you could compare `GUID`s...
/// // assert_eq!(STATIC_GUID, guid!{0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE});
/// # }
/// ```
#[macro_export]
macro_rules! static_guid {
	($name:ident, $guid:tt) => {
		static $name: $crate::GUID = guid!$guid;
	};
}

/// Defines a vtbl struct.
///
/// A peculiar feature of vtbl structs is that they are generic over their `This` parameter type.
/// This allows the vtbl to be embedded in other vtbls where they can assume the correct type.
///
/// When a virtual function is called the type system will check the pointer type without needing ugly casts.
///
/// A `Deref` impl is provided to access the base vtbl members with correct `This` parameter type.
///
/// This macro is expanded as part of [`com_interface!`](macro.com_interface!.html) which you should use.
#[macro_export]
macro_rules! com_vtbl {
	(
		$iface_vtbl:ident,
		$iface:ident,
		$(pub $ident:ident: $vfn:ty,)*
	) => {
		#[repr(C)]
		#[allow(non_snake_case)]
		pub struct $iface_vtbl<$iface: $crate::ComInterface> { $(pub $ident: $vfn,)* }
		impl<$iface: $crate::ComInterface> $crate::ComVtbl for $iface_vtbl<$iface> {}
	};
	(
		$iface_vtbl:ident: $ibase_vtbl:ident,
		$iface:ident,
		$(pub $ident:ident: $vfn:ty,)*
	) => {
		com_vtbl! {
			$iface_vtbl,
			$iface,
			pub base: $ibase_vtbl<$iface>,
			$(pub $ident: $vfn,)*
		}
		impl<$iface: $crate::ComInterface> ::core::ops::Deref for $iface_vtbl<$iface> {
			type Target = $ibase_vtbl<$iface>;
			#[inline(always)] fn deref(&self) -> &$ibase_vtbl<$iface> { &self.base }
		}
	};
}

/// Defines an interface struct and its vtbl struct.
///
/// Due to Rust's macro limitations you must provide names for the vtbl types.
/// By convention they should be named after their interface name with `Vtbl` added at the end.
///
/// Creates a new vtbl struct inheriting from the base vtbl.
/// Implements [`ComVtbl`](trait.ComVtbl.html) and `::core::ops::Deref` to the base through its `base` member.
///
/// Creates a new interface struct containing just a `&'static` reference to its vtbl.
/// Implements [`ComInterface`](trait.ComInterface.html) which can access its vtbl and iid.
/// Implements [`ComCast<$ibase>`](trait.ComCast.html) allowing an exlicit casting to its base interface.
///
/// # Examples
///
/// Note that you **must** use the `$iface` interface name as type for the `This` pointer.
///
/// It is required to import the base interface and its vtbl as the macro only accepts idents.
///
/// See [`guid!`](macro.guid!.html) for help with the IID format.
///
/// ```
/// # #[macro_use] extern crate com_core; fn main() {
/// use ::com_core::unknown::{IUnknown, IUnknownVtbl};
/// 
/// com_interface! {
/// 	interface IInterface(IInterfaceVtbl): IUnknown(IUnknownVtbl);
/// 	{0xe4059080-0xdacb-0x46dd-0xbb59-0x560cb47c9578}
/// 	pub Foo: unsafe extern "stdcall" fn(
/// 		This: *mut IInterface,
/// 	),
/// }
/// # }
/// ```
///
/// Expands to, comments added.
///
/// ```
/// # #[macro_use] extern crate com_core; fn main() {
/// use ::com_core::unknown::{IUnknown, IUnknownVtbl};
/// 
/// // Import these to reduce visual clutter, they use `$crate::*` internally.
/// use ::com_core::{ComInterface, ComVtbl, GUID};
/// 
/// // Represents a boring vtbl with fn pointers to its virtual functions.
/// // Note that it is generic over its `This` type allowing easier reuse and type safety.
/// // Here `base`'s type is `IUnknownVtbl<IInterface>` and uses `IInterface` as its `This` type.
/// #[repr(C)]
/// pub struct IInterfaceVtbl<IInterface: ComInterface> {
/// 	pub base: IUnknownVtbl<IInterface>,
/// 	pub Foo: unsafe extern "stdcall" fn(
/// 		This: *mut IInterface,
/// 	),
/// }
/// // Marks the vtbl struct as `ComVtbl`.
/// impl<IInterface: ComInterface> ComVtbl for IInterfaceVtbl<IInterface> {}
/// // Faux inheritance through `Deref` for convenience.
/// impl<IInterface: ComInterface> ::core::ops::Deref for IInterfaceVtbl<IInterface> {
/// 	type Target = IUnknownVtbl<IInterface>;
/// 	fn deref(&self) -> &IUnknownVtbl<IInterface> { &self.base }
/// }
/// 
/// // The FFI interface struct with internal vtbl pointer.
/// #[repr(C)]
/// pub struct IInterface {
/// 	vtbl: &'static IInterfaceVtbl<IInterface>,
/// }
/// // Implements `ComInterface` giving access to its vtbl and iid.
/// impl ComInterface for IInterface {
///		type Vtbl = IInterfaceVtbl<IInterface>;
/// 	fn vtbl(&self) -> &Self::Vtbl { unsafe { &*self.vtbl } }
/// 	fn iid() -> &'static GUID {
/// 		static_guid!(IID, {0xe4059080-0xdacb-0x46dd-0xbb59-0x560cb47c9578});
/// 		&IID
/// 	}
/// }
/// // Note the lack of `Deref` abuse for inheritance, this is by design.
/// // It would allow you to access the parent's vtbl with incorrect `This` type.
/// // Instead you can access the parent's virtual functions directly through its vtbl.
/// # }
/// ```
#[macro_export]
macro_rules! com_interface {
	(
		$(#[$attr:meta])*
		interface $iface:ident($iface_vtbl:ident);
		$iid:tt
		$(pub $ident:ident: $vfn:ty,)*
	) => {
		com_vtbl!($iface_vtbl, $iface, $(pub $ident: $vfn,)*);
		$(#[$attr])*
		#[repr(C)]
		pub struct $iface { vtbl: *const $iface_vtbl<$iface> }
		impl $crate::ComInterface for $iface {
			type Vtbl = $iface_vtbl<$iface>;
			#[inline(always)] fn vtbl(&self) -> &Self::Vtbl { unsafe { &*self.vtbl } }
			#[inline(always)]
			fn iid() -> &'static $crate::GUID {
				static_guid!(IID, $iid);
				&IID
			}
		}
	};
	(
		$(#[$attr:meta])*
		interface $iface:ident($iface_vtbl:ident): $ibase:ident($ibase_vtbl:ident);
		$iid:tt
		$(pub $ident:ident: $vfn:ty,)*
	) => {
		com_vtbl!($iface_vtbl: $ibase_vtbl, $iface, $(pub $ident: $vfn,)*);
		$(#[$attr])*
		#[repr(C)]
		pub struct $iface { vtbl: *const $iface_vtbl<$iface> }
		impl $crate::ComInterface for $iface {
			type Vtbl = $iface_vtbl<$iface>;
			#[inline(always)] fn vtbl(&self) -> &Self::Vtbl { unsafe { &*self.vtbl } }
			#[inline(always)]
			fn iid() -> &'static $crate::GUID {
				static_guid!(IID, $iid);
				&IID
			}
		}
		impl $crate::ComInherit for $iface {
			type Super = $ibase;
		}
	};
}

/// Implements `ComClass` given a class, the interface it implements and its class id.
#[macro_export]
macro_rules! com_class {
	($class:ty, $clsid:tt $(,$iface:ty)*) => {
		impl $crate::ComClass for $class {
			#[inline(always)]
			fn clsid() -> &'static $crate::GUID {
				static_guid!(CLSID, $clsid);
				&CLSID
			}
		}
		$(impl $crate::ComClassInterface<$iface> for $class {})*
	};
	(_extern $class:ident, $clsid:tt $(,$iface:ty)*) => {
		pub enum $class {}
		com_class!($class, $clsid $(,$iface)*);
	}
}

/// Shorthand defining a public const HRESULT.
#[macro_export]
macro_rules! com_const_hresult {
	($ident:ident = $hr:expr) => {
		pub const $ident: $crate::HRESULT = ($hr as u32) as $crate::HRESULT;
	}
}
