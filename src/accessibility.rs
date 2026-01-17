use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::string::CFString;
use std::ptr;

// External function declaration for AXIsProcessTrustedWithOptions
#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: *const core_foundation::dictionary::__CFDictionary) -> bool;
}

// Key for the options dictionary
const K_AX_TRUSTED_CHECK_OPTION_PROMPT: &str = "AXTrustedCheckOptionPrompt";

/// Check if the app has Accessibility permissions
pub fn is_trusted() -> bool {
    unsafe { AXIsProcessTrustedWithOptions(ptr::null()) }
}

/// Check if the app has Accessibility permissions, optionally prompting the user
pub fn check_accessibility(prompt: bool) -> bool {
    if prompt {
        let key = CFString::new(K_AX_TRUSTED_CHECK_OPTION_PROMPT);
        let value = if prompt { CFBoolean::true_value() } else { CFBoolean::false_value() };
        
        let options = core_foundation::dictionary::CFDictionary::from_CFType_pairs(&[
            (key.as_CFType(), value.as_CFType())
        ]);
        
        unsafe { AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef()) }
    } else {
        is_trusted()
    }
}

/// Print instructions for enabling Accessibility permissions
pub fn print_permission_instructions() {
    eprintln!();
    eprintln!("╔════════════════════════════════════════════════════════════════╗");
    eprintln!("║           Accessibility Permission Required                     ║");
    eprintln!("╠════════════════════════════════════════════════════════════════╣");
    eprintln!("║ This app needs Accessibility permissions to detect trackpad    ║");
    eprintln!("║ gestures globally.                                             ║");
    eprintln!("║                                                                ║");
    eprintln!("║ To grant permission:                                           ║");
    eprintln!("║ 1. Open System Settings                                        ║");
    eprintln!("║ 2. Go to Privacy & Security → Accessibility                    ║");
    eprintln!("║ 3. Add and enable this application                             ║");
    eprintln!("║                                                                ║");
    eprintln!("║ After granting permission, restart this application.           ║");
    eprintln!("╚════════════════════════════════════════════════════════════════╝");
    eprintln!();
}

