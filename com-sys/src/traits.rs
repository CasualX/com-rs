use winapi::{CLSID, IID};

/// COM Vtbl structs implement `ComVtbl` which is little more than a marker.
///
/// The virtual function tables are structs containing function pointers whose first argument is a pointer to the instance being acted upon.
///
/// Therein lies a footgun as nothing is preventing you from calling a virtual function with a different `This` pointer than what was used to access its vtbl.
///
/// Consider the type of the `This` pointer, it depends on which type this vtbl is part of.
/// But if you naively reuse the vtbl of a base class, they use the `This` pointer type of the inherited interface without fuss while the correct `This` pointer requires an ugly cast.
///
/// The approach I see used in C is to repeat the base classes virtual functions with the only change being their `This` pointer type.
/// Clearly this is not ideal as it is very error prone and hard on code refactoring.
///
/// Rust can do better:
///
/// ```
/// # use ::com_core::ComInterface;
/// struct IInterfaceVtbl<IInterface: ComInterface> {
/// 	pub Foo: unsafe extern "stdcall" fn(
/// 		This: *mut IInterface,
/// 	),
/// }
/// ```
///
/// By making the vtbl generic over its `This` pointer we can achieve some semblance of type safety against accidental errors and achieve complete reusability.
/// Note the name of the generic parameter, looks very natural while being maximum flexible. The trait bound further reduces accidents.
///
/// It is still unsafe to call, but the type system is holding our hand.
///
/// Implementing `Deref` to access the base vtbl (with correct `This` pointer!) is icing on the cake.
///
/// See [`com_vtbl!`](macros/index.html#com-ffi-macros) for how to use the provided macros to automate their implementation.
pub trait ComVtbl: Sized {}

/// COM Interface structs implement `ComInterface`.
///
/// Interfaces have an associated vtbl type and, in COM land, are identified with an IID (**I**&#8203;nterface **ID**&#8203;entifier).
///
/// All they contain is a static reference to their vtbl.
///
/// Note that it is not legal to 'cast' (transmute, really) the interface to its base interface as this would expose its vtbl with the incorrect type.
///
/// Inheritance is declared by [`ComInherit`](trait.ComInherit.html) with an associated type indicating its base interface.
///
/// See [`com_interface!`](macros/index.html#com-ffi-macros) for how to use the provided macros to automate their implementation.
pub trait ComInterface: Sized {
	type Vtbl: ComVtbl;
	/// Returns the vtbl.
	fn vtbl(&self) -> &Self::Vtbl;
	/// Returns the interface id.
	fn iid() -> &'static IID;
}

/// Declares a com interface inherits another interface.
///
/// Implemented as part of [`com_interface!`](macros/index.html#com-ffi-macros).
pub trait ComInherit: ComInterface {
	type Super: ComInterface;
}

/// COM Class structs implement `ComClass`.
///
/// Classes are identified with a CLSID (**CL**&#8203;a&#8203;**S**&#8203;s **ID**&#8203;entifier).
///
/// When classes are defined externally, which is usually the case, they may be represented by an empty enum.
///
/// See [`com_class!`](macros/index.html#com-ffi-macros) for how to use the provided macros to automate their implementation.
pub trait ComClass {
	/// Returns its class id.
	fn clsid() -> &'static CLSID;
}

/// Declares the `ComInterface`s which are implemented by a `ComClass`.
///
/// Implemented as part of [`com_class!`](macros/index.html#com-ffi-macros).
pub trait ComClassInterface<I: ComInterface>: ComClass {}
