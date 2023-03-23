// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Windows Internet Services API procedure declarations, types and constants.
// Currently, this only contains `INTERNET_FLAG_BGUPDATE`, which is needed to correctly define
// `wininet::INTERNET_FLAGS_MASK`.
use shared::minwindef::DWORD;
pub const INTERNET_FLAG_BGUPDATE: DWORD = 0x00000008;
// Functions from wininet.dll that *should* be in this header.
// pub fn AppCacheCheckManifest();
// pub fn AppCacheCloseHandle();
// pub fn AppCacheCreateAndCommitFile();
// pub fn AppCacheDeleteGroup();
// pub fn AppCacheDeleteIEGroup();
// pub fn AppCacheDuplicateHandle();
// pub fn AppCacheFinalize();
// pub fn AppCacheFreeDownloadList();
// pub fn AppCacheFreeGroupList();
// pub fn AppCacheFreeIESpace();
// pub fn AppCacheFreeSpace();
// pub fn AppCacheGetDownloadList();
// pub fn AppCacheGetFallbackUrl();
// pub fn AppCacheGetGroupList();
// pub fn AppCacheGetIEGroupList();
// pub fn AppCacheGetInfo();
// pub fn AppCacheGetManifestUrl();
// pub fn AppCacheLookup();
// pub fn CommitUrlCacheEntryBinaryBlob();
// pub fn CreateCacheServerRpcBinding();
// pub fn CreateUrlCacheContainerA();
// pub fn CreateUrlCacheContainerW();
// pub fn CreateUrlCacheEntryExW();
// pub fn DeleteIE3Cache();
// pub fn DeleteUrlCacheContainerA();
// pub fn DeleteUrlCacheContainerW();
// pub fn DoConnectoidsExist();
// pub fn ExportCookieFileA();
// pub fn ExportCookieFileW();
// pub fn FindFirstUrlCacheContainerA();
// pub fn FindFirstUrlCacheContainerW();
// pub fn FindNextUrlCacheContainerA();
// pub fn FindNextUrlCacheContainerW();
// pub fn FindP3PPolicySymbol();
// pub fn ForceNexusLookupExW();
// pub fn FreeP3PObject();
// pub fn FreeUrlCacheSpaceA();
// pub fn FreeUrlCacheSpaceW();
// pub fn GetCacheServerConnection();
// pub fn GetDiskInfoA();
// pub fn GetP3PPolicy();
// pub fn GetP3PRequestStatus();
// pub fn GetUrlCacheConfigInfoA();
// pub fn GetUrlCacheConfigInfoW();
// pub fn GetUrlCacheEntryBinaryBlob();
// pub fn GetUrlCacheHeaderData();
// pub fn HttpCheckDavComplianceA();
// pub fn HttpCheckDavComplianceW();
// pub fn HttpCloseDependencyHandle();
// pub fn HttpDuplicateDependencyHandle();
// pub fn HttpGetServerCredentials();
// pub fn HttpGetTunnelSocket();
// pub fn HttpIsHostHstsEnabled();
// pub fn HttpOpenDependencyHandle();
// pub fn HttpPushClose();
// pub fn HttpPushEnable();
// pub fn HttpPushWait();
// pub fn HttpWebSocketClose();
// pub fn HttpWebSocketCompleteUpgrade();
// pub fn HttpWebSocketQueryCloseStatus();
// pub fn HttpWebSocketReceive();
// pub fn HttpWebSocketSend();
// pub fn HttpWebSocketShutdown();
// pub fn ImportCookieFileA();
// pub fn ImportCookieFileW();
// pub fn IncrementUrlCacheHeaderData();
// pub fn InternalInternetGetCookie();
// pub fn InternetAlgIdToStringA();
// pub fn InternetAlgIdToStringW();
// pub fn InternetAutodialCallback();
// pub fn InternetAutoProxyGetProxyForUrl();
// pub fn InternetAutoProxyOnSendRequestComplete();
// pub fn InternetFortezzaCommand();
// pub fn InternetFreeProxyInfoList();
// pub fn InternetGetCertByURLA();
// pub fn InternetGetProxyForUrl();
// pub fn InternetGetSecurityInfoByURLA();
// pub fn InternetGetSecurityInfoByURLW();
// pub fn InternetQueryFortezzaStatus();
// pub fn InternetSecurityProtocolToStringA();
// pub fn InternetSecurityProtocolToStringW();
// pub fn InternetShowSecurityInfoByURLA();
// pub fn InternetShowSecurityInfoByURLW();
// pub fn InternetWriteFileExA();
// pub fn InternetWriteFileExW();
// pub fn IsDialUpConnection();
// pub fn IsDomainLegalCookieDomainA();
// pub fn IsDomainLegalCookieDomainW();
// pub fn IsHostInProxyBypassList();
// pub fn IsLanConnection();
// pub fn IsProfilesEnabled();
// pub fn IsUrlCacheEntryExpiredA();
// pub fn IsUrlCacheEntryExpiredW();
// pub fn LoadUrlCacheContent();
// pub fn MapResourceToPolicy();
// pub fn ParseX509EncodedCertificateForListBoxEntry();
// pub fn PerformOperationOverUrlCacheA();
// pub fn ReadGuidsForConnectedNetworks();
// pub fn RegisterForNetworkChangeNotification();
// pub fn RegisterUrlCacheNotification();
// pub fn RunOnceUrlCache();
// pub fn SetGlobalJetParameters();
// pub fn SetUrlCacheConfigInfoA();
// pub fn SetUrlCacheConfigInfoW();
// pub fn SetUrlCacheHeaderData();
// pub fn ShowCertificate();
// pub fn ShowClientAuthCerts();
// pub fn ShowSecurityInfo();
// pub fn ShowX509EncodedCertificate();
// pub fn UnRegisterNetworkChangeNotification();
// pub fn UpdateUrlCacheContentPath();
// pub fn UrlCacheCheckEntriesExist();
// pub fn UrlCacheCloseEntryHandle();
// pub fn UrlCacheContainerSetEntryMaximumAge();
// pub fn UrlCacheCreateContainer();
// pub fn UrlCacheFindFirstEntry();
// pub fn UrlCacheFindNextEntry();
// pub fn UrlCacheFreeEntryInfo();
// pub fn UrlCacheFreeGlobalSpace();
// pub fn UrlCacheGetContentPaths();
// pub fn UrlCacheGetEntryInfo();
// pub fn UrlCacheGetGlobalCacheSize();
// pub fn UrlCacheGetGlobalLimit();
// pub fn UrlCacheReadEntryStream();
// pub fn UrlCacheReloadSettings();
// pub fn UrlCacheRetrieveEntryFile();
// pub fn UrlCacheRetrieveEntryStream();
// pub fn UrlCacheServer();
// pub fn UrlCacheSetGlobalLimit();
// pub fn UrlCacheUpdateEntryExtraData();
// pub fn UrlZonesDetach();
