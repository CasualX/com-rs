Scraped from https://msdn.microsoft.com/en-us/library/cc231198.aspx with Scratchpad:

```js
var tr = document.querySelectorAll('table[summary=table] tr');
var list = [];
for (var i = 3; i<tr.length; ++i) {
  let row = tr[i];
  let name = row.querySelector("td>p:nth-child(1)");
  let id = row.querySelector("td>p:nth-child(2)");
  let desc = row.querySelector("td:nth-child(2)>p");
  list.push({
    name: name.innerText,
    id: id.innerText,
    desc: desc.innerText,
  });
}

var s = "";
for (var i = 0; i<list.length; ++i) {
  let f = list[i];
  s += "{id:" + f.id + ", name:" + f.name + ", desc:\"" + f.desc + "\"}\r\n";
}
s
```

```js
[{id:0, name:FACILITY_NULL, desc:"The default facility code."}
{id:1, name:FACILITY_RPC, desc:"The source of the error code is an RPC subsystem."}
{id:2, name:FACILITY_DISPATCH, desc:"The source of the error code is a COM Dispatch."}
{id:3, name:FACILITY_STORAGE, desc:"The source of the error code is OLE Storage."}
{id:4, name:FACILITY_ITF, desc:"The source of the error code is COM/OLE Interface management."}
{id:7, name:FACILITY_WIN32, desc:"This region is reserved to map undecorated error codes into HRESULTs."}
{id:8, name:FACILITY_WINDOWS, desc:"The source of the error code is the Windows subsystem."}
{id:9, name:FACILITY_SECURITY, desc:"The source of the error code is the Security API layer."}
{id:9, name:FACILITY_SSPI, desc:"The source of the error code is the Security API layer."}
{id:10, name:FACILITY_CONTROL, desc:"The source of the error code is the control mechanism."}
{id:11, name:FACILITY_CERT, desc:"The source of the error code is a certificate client or server?�"}
{id:12, name:FACILITY_INTERNET, desc:"The source of the error code is Wininet related."}
{id:13, name:FACILITY_MEDIASERVER, desc:"The source of the error code is the Windows Media Server."}
{id:14, name:FACILITY_MSMQ, desc:"The source of the error code is the Microsoft Message Queue."}
{id:15, name:FACILITY_SETUPAPI, desc:"The source of the error code is the Setup API."}
{id:16, name:FACILITY_SCARD, desc:"The source of the error code is the Smart-card subsystem."}
{id:17, name:FACILITY_COMPLUS, desc:"The source of the error code is COM+."}
{id:18, name:FACILITY_AAF, desc:"The source of the error code is the Microsoft agent."}
{id:19, name:FACILITY_URT, desc:"The source of the error code is .NET CLR."}
{id:20, name:FACILITY_ACS, desc:"The source of the error code is the audit collection service."}
{id:21, name:FACILITY_DPLAY, desc:"The source of the error code is Direct Play."}
{id:22, name:FACILITY_UMI, desc:"The source of the error code is the ubiquitous memoryintrospection service."}
{id:23, name:FACILITY_SXS, desc:"The source of the error code is Side-by-side servicing."}
{id:24, name:FACILITY_WINDOWS_CE, desc:"The error code is specific to Windows CE."}
{id:25, name:FACILITY_HTTP, desc:"The source of the error code is HTTP support."}
{id:26, name:FACILITY_USERMODE_COMMONLOG, desc:"The source of the error code is common Logging support."}
{id:31, name:FACILITY_USERMODE_FILTER_MANAGER, desc:"The source of the error code is the user mode filter manager."}
{id:32, name:FACILITY_BACKGROUNDCOPY, desc:"The source of the error code is background copy control"}
{id:33, name:FACILITY_CONFIGURATION, desc:"The source of the error code is configuration services."}
{id:34, name:FACILITY_STATE_MANAGEMENT, desc:"The source of the error code is state management services."}
{id:35, name:FACILITY_METADIRECTORY, desc:"The source of the error code is the Microsoft Identity Server."}
{id:36, name:FACILITY_WINDOWSUPDATE, desc:"The source of the error code is a Windows update."}
{id:37, name:FACILITY_DIRECTORYSERVICE, desc:"The source of the error code is Active Directory."}
{id:38, name:FACILITY_GRAPHICS, desc:"The source of the error code is the graphics drivers."}
{id:39, name:FACILITY_SHELL, desc:"The source of the error code is the user Shell."}
{id:40, name:FACILITY_TPM_SERVICES, desc:"The source of the error code is the Trusted Platform Module services."}
{id:41, name:FACILITY_TPM_SOFTWARE, desc:"The source of the error code is the Trusted Platform Module applications."}
{id:48, name:FACILITY_PLA, desc:"The source of the error code is Performance Logs and Alerts"}
{id:49, name:FACILITY_FVE, desc:"The source of the error code is Full volume encryption."}
{id:50, name:FACILITY_FWP, desc:"he source of the error code is the Firewall Platform."}
{id:51, name:FACILITY_WINRM, desc:"The source of the error code is the Windows Resource Manager."}
{id:52, name:FACILITY_NDIS, desc:"The source of the error code is the Network Driver Interface."}
{id:53, name:FACILITY_USERMODE_HYPERVISOR, desc:"The source of the error code is the Usermode Hypervisor components."}
{id:54, name:FACILITY_CMI, desc:"The source of the error code is the Configuration Management Infrastructure."}
{id:55, name:FACILITY_USERMODE_VIRTUALIZATION, desc:"The source of the error code is the user mode virtualization subsystem."}
{id:56, name:FACILITY_USERMODE_VOLMGR, desc:"The source of the error code is� the user mode volume manager"}
{id:57, name:FACILITY_BCD, desc:"The source of the error code is the Boot Configuration Database."}
{id:58, name:FACILITY_USERMODE_VHD, desc:"The source of the error code is user mode virtual hard disk support."}
{id:60, name:FACILITY_SDIAG, desc:"The source of the error code is System Diagnostics."}
{id:61, name:FACILITY_WEBSERVICES, desc:"The source of the error code is the Web Services."}
{id:80, name:FACILITY_WINDOWS_DEFENDER, desc:"The source of the error code is a Windows Defender component."}
{id:81, name:FACILITY_OPC, desc:"The source of the error code is the open connectivity service."}]
```
