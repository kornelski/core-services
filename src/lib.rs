#![allow(non_upper_case_globals)]
#![allow(bad_style)]

/// The functions you want are probably missing - please make a [pull request](https://github.com/kornelski/core-services) adding whatever you need!
///
/// [The original Apple docs are here](https://developer.apple.com/documentation/coreservices?language=objc)

use std::os::raw::c_void;
use core_foundation::url::CFURLRef;
use core_foundation::base::OSStatus;
use core_foundation::error::CFErrorRef;
pub use core_foundation::base::TCFType;
pub use core_foundation::string::CFStringRef;
pub use core_foundation::string::CFString;
pub use core_foundation::array::CFArrayRef;
pub use core_foundation::array::CFArray;
use std::os::raw::c_uint;

/// Specify the desired role or roles for an application to claim with respect to an item or family of items.
pub type LSRolesMask = c_uint;
pub type LSAcceptanceFlags = c_uint;

/// Requests the role None (the application cannot open the item, but provides an icon and a kind string for it).
pub const kLSRolesNone: LSRolesMask = 0x00000001;

/// Requests the role Viewer (the application can read and present the item, but cannot manipulate or save it).
pub const kLSRolesViewer: LSRolesMask = 0x00000002;

/// Requests the role Editor (the application can read, present, manipulate, and save the item).
pub const kLSRolesEditor: LSRolesMask = 0x00000004;

/// Requests the role Shell (the application can execute the item).
pub const kLSRolesShell: LSRolesMask = 0x00000008;

/// Accepts any role with respect to the item.
pub const kLSRolesAll: LSRolesMask = 0xFFFFFFFF;

/// Requests the default behavior (as opposed to the behavior specified by the kLSAcceptAllowLoginUI flag).
pub const kLSAcceptDefault: LSAcceptanceFlags = 0x00000001;

/// Requests that the user interface to log in be presented if necessary. If LSCanRefAcceptItem or
/// LSCanURLAcceptURL is called during a drag-and-drop operation, showing a server login dialog would
/// be an inappropriate user experience. If the target designated in the function call is an alias to
/// an application, Launch Services needs to resolve the alias to ascertain what file types the
/// application can open; however, if the application is on a server that needs to be authenticated,
/// Launch Services will by default fail to resolve the alias, to avoid having to present the login interface.
/// To override this default behavior by allowing the server login interface, set the kLSAcceptAllowLoginUI flag.
pub const kLSAcceptAllowLoginUI: LSAcceptanceFlags = 0x00000002;

pub type FourCharCode = u32;
pub type ResType = FourCharCode;

pub type DescType = ResType;
#[repr(C)]
pub struct AEDataStorageType {
    _opaque: [u8; 0]
}
pub type AEDataStorage = *mut AEDataStorageType;

#[repr(C)]
pub struct AEDesc {
  pub descriptorType: DescType,
  pub dataHandle: AEDataStorage ,
}

#[repr(C)]
pub struct LSLaunchURLSpec {
    /// A Core Foundation URL reference designating the application to launch.
    /// The URL must have scheme file and contain a valid path to an application file or application bundle.
    /// Set this field to NULL to request that each item in the itemURLs array be opened in its own preferred application.
    pub appURL: CFURLRef,
    /// items to open/print
    pub itemURLs: CFArrayRef,
    /// passed untouched to application as optional parameter
    pub passThruParams: *const AEDesc,
    pub launchFlags: LSLaunchFlags,
    /// used if you register for app birth/death notification
    pub asyncRefCon:  *mut c_void,
}

pub type LSLaunchFlags = u32;

/// Defaults = open, async
pub const kLSLaunchDefaults: LSLaunchFlags             = 0x00000001;
/// Print items instead of open them
pub const kLSLaunchAndPrint: LSLaunchFlags             = 0x00000002;
/// Report launch/open failures in the UI
pub const kLSLaunchAndDisplayErrors: LSLaunchFlags     = 0x00000040;
/// Do not add app or documents to recents menus.
pub const kLSLaunchDontAddToRecents: LSLaunchFlags     = 0x00000100;
/// Do not bring new app to the foreground.
pub const kLSLaunchDontSwitch: LSLaunchFlags           = 0x00000200;
/// Asynchronous launch; return as soon as the app starts launching.
pub const kLSLaunchAsync: LSLaunchFlags                = 0x00010000;
/// Instantiate app even if it is already running.
pub const kLSLaunchNewInstance: LSLaunchFlags          = 0x00080000;
/// Send child a "hide" request as soon as it checks in.
pub const kLSLaunchAndHide: LSLaunchFlags              = 0x00100000;
/// Hide all other apps when the app checks in.
pub const kLSLaunchAndHideOthers: LSLaunchFlags        = 0x00200000;


#[link(name="CoreServices", kind="framework")]
extern "C" {
    /// Indicates that the tag is a filename extension.
    pub static kUTTagClassFilenameExtension: CFStringRef;
    /// Indicates that the tag is a MIME type.
    pub static kUTTagClassMIMEType: CFStringRef;
    /// Indicates that the tag is an NSPasteBoard type.
    pub static kUTTagClassNSPboardType: CFStringRef;
    /// Indicates that the tag is an OSType.
    pub static kUTTagClassOSType: CFStringRef;

    pub fn UTTypeCreatePreferredIdentifierForTag(inTagClass: CFStringRef, inTag: CFStringRef, inConformingToUTI: CFStringRef) -> CFStringRef;

    pub fn LSCopyDefaultRoleHandlerForContentType(inContentType: CFStringRef, inRole: LSRolesMask) -> CFStringRef;
    /// Locates an array of application bundle identifiers for applications capable of handling a specified content type with the specified roles.
    pub fn LSCopyAllRoleHandlersForContentType(inContentType: CFStringRef, inRole: LSRolesMask) -> CFArrayRef;

    /// Locates all URLs for applications that correspond to the specified bundle identifier.
    pub fn LSCopyApplicationURLsForBundleIdentifier(inBundleIdentifier: CFStringRef, outError: *mut CFErrorRef) -> CFArrayRef;

    /// Tests whether an application can accept (open) an item designated by URL.
    pub fn LSCanURLAcceptURL(inItemURL: CFURLRef, inTargetURL: CFURLRef, inRoleMask: LSRolesMask, inFlags: LSAcceptanceFlags, outAcceptsItem: *mut bool) -> OSStatus;

    /// Opens one or more items designated by URL, in either their preferred applications or a designated application.
    pub fn LSOpenFromURLSpec(inLaunchSpec: *const LSLaunchURLSpec, outLaunchedURL: *mut CFURLRef) -> OSStatus;
}
