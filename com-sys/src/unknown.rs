use ::winapi::{HRESULT, LPVOID, ULONG, REFIID};

com_interface! {
	interface IUnknown(IUnknownVtbl);
	{0x00000000-0x0000-0x0000-0xC000-0x000000000046}
	pub QueryInterface: unsafe extern "stdcall" fn(
		This: *mut IUnknown,
		riid: REFIID,
		ppvObject: *mut LPVOID,
	) -> HRESULT,
	pub AddRef: unsafe extern "stdcall" fn(
		This: *mut IUnknown,
	) -> ULONG,
	pub Release: unsafe extern "stdcall" fn(
		This: *mut IUnknown,
	) -> ULONG,
}
