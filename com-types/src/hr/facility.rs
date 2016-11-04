/*!
Overdone facility.

No idea why I did this, was fun scraping though.
*/

use ::std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Facility(pub u32);
impl fmt::Debug for Facility {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if cfg!(features = "strings") {
			if let Some(name) = name(*self) {
				name.fmt(f)
			}
			else {
				write!(f, "Facility({})", self.0)
			}
		}
		else {
			write!(f, "Facility({})", self.0)
		}
	}
}

pub const FACILITY_NULL: Facility = Facility(0);
pub const FACILITY_RPC: Facility = Facility(1);
pub const FACILITY_DISPATCH: Facility = Facility(2);
pub const FACILITY_STORAGE: Facility = Facility(3);
pub const FACILITY_ITF: Facility = Facility(4);
pub const FACILITY_WIN32: Facility = Facility(7);
pub const FACILITY_WINDOWS: Facility = Facility(8);
pub const FACILITY_SECURITY: Facility = Facility(9);
pub const FACILITY_SSPI: Facility = Facility(9);
pub const FACILITY_CONTROL: Facility = Facility(10);
pub const FACILITY_CERT: Facility = Facility(11);
pub const FACILITY_INTERNET: Facility = Facility(12);
pub const FACILITY_MEDIASERVER: Facility = Facility(13);
pub const FACILITY_MSMQ: Facility = Facility(14);
pub const FACILITY_SETUPAPI: Facility = Facility(15);
pub const FACILITY_SCARD: Facility = Facility(16);
pub const FACILITY_COMPLUS: Facility = Facility(17);
pub const FACILITY_AAF: Facility = Facility(18);
pub const FACILITY_URT: Facility = Facility(19);
pub const FACILITY_ACS: Facility = Facility(20);
pub const FACILITY_DPLAY: Facility = Facility(21);
pub const FACILITY_UMI: Facility = Facility(22);
pub const FACILITY_SXS: Facility = Facility(23);
pub const FACILITY_WINDOWS_CE: Facility = Facility(24);
pub const FACILITY_HTTP: Facility = Facility(25);
pub const FACILITY_USERMODE_COMMONLOG: Facility = Facility(26);
pub const FACILITY_USERMODE_FILTER_MANAGER: Facility = Facility(31);
pub const FACILITY_BACKGROUNDCOPY: Facility = Facility(32);
pub const FACILITY_CONFIGURATION: Facility = Facility(33);
pub const FACILITY_STATE_MANAGEMENT: Facility = Facility(34);
pub const FACILITY_METADIRECTORY: Facility = Facility(35);
pub const FACILITY_WINDOWSUPDATE: Facility = Facility(36);
pub const FACILITY_DIRECTORYSERVICE: Facility = Facility(37);
pub const FACILITY_GRAPHICS: Facility = Facility(38);
pub const FACILITY_SHELL: Facility = Facility(39);
pub const FACILITY_TPM_SERVICES: Facility = Facility(40);
pub const FACILITY_TPM_SOFTWARE: Facility = Facility(41);
pub const FACILITY_PLA: Facility = Facility(48);
pub const FACILITY_FVE: Facility = Facility(49);
pub const FACILITY_FWP: Facility = Facility(50);
pub const FACILITY_WINRM: Facility = Facility(51);
pub const FACILITY_NDIS: Facility = Facility(52);
pub const FACILITY_USERMODE_HYPERVISOR: Facility = Facility(53);
pub const FACILITY_CMI: Facility = Facility(54);
pub const FACILITY_USERMODE_VIRTUALIZATION: Facility = Facility(55);
pub const FACILITY_USERMODE_VOLMGR: Facility = Facility(56);
pub const FACILITY_BCD: Facility = Facility(57);
pub const FACILITY_USERMODE_VHD: Facility = Facility(58);
pub const FACILITY_SDIAG: Facility = Facility(60);
pub const FACILITY_WEBSERVICES: Facility = Facility(61);
pub const FACILITY_WINDOWS_DEFENDER: Facility = Facility(80);
pub const FACILITY_OPC: Facility = Facility(81);

#[cfg(features = "strings")]
pub fn name(facility: Facility) -> Option<&'static str> {
	match facility {
		FACILITY_NULL => Some("FACILITY_NULL"),
		FACILITY_RPC => Some("FACILITY_RPC"),
		FACILITY_DISPATCH => Some("FACILITY_DISPATCH"),
		FACILITY_STORAGE => Some("FACILITY_STORAGE"),
		FACILITY_ITF => Some("FACILITY_ITF"),
		FACILITY_WIN32 => Some("FACILITY_WIN32"),
		FACILITY_WINDOWS => Some("FACILITY_WINDOWS"),
		FACILITY_SECURITY => Some("FACILITY_SECURITY"),
		FACILITY_SSPI => Some("FACILITY_SSPI"),
		FACILITY_CONTROL => Some("FACILITY_CONTROL"),
		FACILITY_CERT => Some("FACILITY_CERT"),
		FACILITY_INTERNET => Some("FACILITY_INTERNET"),
		FACILITY_MEDIASERVER => Some("FACILITY_MEDIASERVER"),
		FACILITY_MSMQ => Some("FACILITY_MSMQ"),
		FACILITY_SETUPAPI => Some("FACILITY_SETUPAPI"),
		FACILITY_SCARD => Some("FACILITY_SCARD"),
		FACILITY_COMPLUS => Some("FACILITY_COMPLUS"),
		FACILITY_AAF => Some("FACILITY_AAF"),
		FACILITY_URT => Some("FACILITY_URT"),
		FACILITY_ACS => Some("FACILITY_ACS"),
		FACILITY_DPLAY => Some("FACILITY_DPLAY"),
		FACILITY_UMI => Some("FACILITY_UMI"),
		FACILITY_SXS => Some("FACILITY_SXS"),
		FACILITY_WINDOWS_CE => Some("FACILITY_WINDOWS_CE"),
		FACILITY_HTTP => Some("FACILITY_HTTP"),
		FACILITY_USERMODE_COMMONLOG => Some("FACILITY_USERMODE_COMMONLOG"),
		FACILITY_USERMODE_FILTER_MANAGER => Some("FACILITY_USERMODE_FILTER_MANAGER"),
		FACILITY_BACKGROUNDCOPY => Some("FACILITY_BACKGROUNDCOPY"),
		FACILITY_CONFIGURATION => Some("FACILITY_CONFIGURATION"),
		FACILITY_STATE_MANAGEMENT => Some("FACILITY_STATE_MANAGEMENT"),
		FACILITY_METADIRECTORY => Some("FACILITY_METADIRECTORY"),
		FACILITY_WINDOWSUPDATE => Some("FACILITY_WINDOWSUPDATE"),
		FACILITY_DIRECTORYSERVICE => Some("FACILITY_DIRECTORYSERVICE"),
		FACILITY_GRAPHICS => Some("FACILITY_GRAPHICS"),
		FACILITY_SHELL => Some("FACILITY_SHELL"),
		FACILITY_TPM_SERVICES => Some("FACILITY_TPM_SERVICES"),
		FACILITY_TPM_SOFTWARE => Some("FACILITY_TPM_SOFTWARE"),
		FACILITY_PLA => Some("FACILITY_PLA"),
		FACILITY_FVE => Some("FACILITY_FVE"),
		FACILITY_FWP => Some("FACILITY_FWP"),
		FACILITY_WINRM => Some("FACILITY_WINRM"),
		FACILITY_NDIS => Some("FACILITY_NDIS"),
		FACILITY_USERMODE_HYPERVISOR => Some("FACILITY_USERMODE_HYPERVISOR"),
		FACILITY_CMI => Some("FACILITY_CMI"),
		FACILITY_USERMODE_VIRTUALIZATION => Some("FACILITY_USERMODE_VIRTUALIZATION"),
		FACILITY_USERMODE_VOLMGR => Some("FACILITY_USERMODE_VOLMGR"),
		FACILITY_BCD => Some("FACILITY_BCD"),
		FACILITY_USERMODE_VHD => Some("FACILITY_USERMODE_VHD"),
		FACILITY_SDIAG => Some("FACILITY_SDIAG"),
		FACILITY_WEBSERVICES => Some("FACILITY_WEBSERVICES"),
		FACILITY_WINDOWS_DEFENDER => Some("FACILITY_WINDOWS_DEFENDER"),
		FACILITY_OPC => Some("FACILITY_OPC"),
		_ => None,
	}
}
#[cfg(not(features = "strings"))]
fn name(_: Facility) -> Option<&'static str> {
	None
}

#[cfg(features = "strings")]
pub fn desc(facility: Facility) -> Option<&'static str> {
	match facility {
		FACILITY_NULL => Some("The default facility code."),
		FACILITY_RPC => Some("The source of the error code is an RPC subsystem."),
		FACILITY_DISPATCH => Some("The source of the error code is a COM Dispatch."),
		FACILITY_STORAGE => Some("The source of the error code is OLE Storage."),
		FACILITY_ITF => Some("The source of the error code is COM/OLE Interface management."),
		FACILITY_WIN32 => Some("This region is reserved to map undecorated error codes into HRESULTs."),
		FACILITY_WINDOWS => Some("The source of the error code is the Windows subsystem."),
		FACILITY_SECURITY => Some("The source of the error code is the Security API layer."),
		FACILITY_SSPI => Some("The source of the error code is the Security API layer."),
		FACILITY_CONTROL => Some("The source of the error code is the control mechanism."),
		FACILITY_CERT => Some("The source of the error code is a certificate client or server? "),
		FACILITY_INTERNET => Some("The source of the error code is Wininet related."),
		FACILITY_MEDIASERVER => Some("The source of the error code is the Windows Media Server."),
		FACILITY_MSMQ => Some("The source of the error code is the Microsoft Message Queue."),
		FACILITY_SETUPAPI => Some("The source of the error code is the Setup API."),
		FACILITY_SCARD => Some("The source of the error code is the Smart-card subsystem."),
		FACILITY_COMPLUS => Some("The source of the error code is COM+."),
		FACILITY_AAF => Some("The source of the error code is the Microsoft agent."),
		FACILITY_URT => Some("The source of the error code is .NET CLR."),
		FACILITY_ACS => Some("The source of the error code is the audit collection service."),
		FACILITY_DPLAY => Some("The source of the error code is Direct Play."),
		FACILITY_UMI => Some("The source of the error code is the ubiquitous memoryintrospection service."),
		FACILITY_SXS => Some("The source of the error code is Side-by-side servicing."),
		FACILITY_WINDOWS_CE => Some("The error code is specific to Windows CE."),
		FACILITY_HTTP => Some("The source of the error code is HTTP support."),
		FACILITY_USERMODE_COMMONLOG => Some("The source of the error code is common Logging support."),
		FACILITY_USERMODE_FILTER_MANAGER => Some("The source of the error code is the user mode filter manager."),
		FACILITY_BACKGROUNDCOPY => Some("The source of the error code is background copy control"),
		FACILITY_CONFIGURATION => Some("The source of the error code is configuration services."),
		FACILITY_STATE_MANAGEMENT => Some("The source of the error code is state management services."),
		FACILITY_METADIRECTORY => Some("The source of the error code is the Microsoft Identity Server."),
		FACILITY_WINDOWSUPDATE => Some("The source of the error code is a Windows update."),
		FACILITY_DIRECTORYSERVICE => Some("The source of the error code is Active Directory."),
		FACILITY_GRAPHICS => Some("The source of the error code is the graphics drivers."),
		FACILITY_SHELL => Some("The source of the error code is the user Shell."),
		FACILITY_TPM_SERVICES => Some("The source of the error code is the Trusted Platform Module services."),
		FACILITY_TPM_SOFTWARE => Some("The source of the error code is the Trusted Platform Module applications."),
		FACILITY_PLA => Some("The source of the error code is Performance Logs and Alerts"),
		FACILITY_FVE => Some("The source of the error code is Full volume encryption."),
		FACILITY_FWP => Some("he source of the error code is the Firewall Platform."),
		FACILITY_WINRM => Some("The source of the error code is the Windows Resource Manager."),
		FACILITY_NDIS => Some("The source of the error code is the Network Driver Interface."),
		FACILITY_USERMODE_HYPERVISOR => Some("The source of the error code is the Usermode Hypervisor components."),
		FACILITY_CMI => Some("The source of the error code is the Configuration Management Infrastructure."),
		FACILITY_USERMODE_VIRTUALIZATION => Some("The source of the error code is the user mode virtualization subsystem."),
		FACILITY_USERMODE_VOLMGR => Some("The source of the error code is the user mode volume manager"),
		FACILITY_BCD => Some("The source of the error code is the Boot Configuration Database."),
		FACILITY_USERMODE_VHD => Some("The source of the error code is user mode virtual hard disk support."),
		FACILITY_SDIAG => Some("The source of the error code is System Diagnostics."),
		FACILITY_WEBSERVICES => Some("The source of the error code is the Web Services."),
		FACILITY_WINDOWS_DEFENDER => Some("The source of the error code is a Windows Defender component."),
		FACILITY_OPC => Some("The source of the error code is the open connectivity service."),
		_ => None,
	}
}

// #[cfg(not(features = "strings"))]
// fn desc(_: Facility) -> Option<&'static str> {
// 	None
// }
