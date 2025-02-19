pub fn get_windows_excludes() -> Vec<String> {
    vec![
        "scoop".to_string(),
        "AppData\\Local\\AMD".to_string(),
        "AppData\\Local\\Microsoft".to_string(),
        "AppData\\Local\\Mozilla\\Firefox".to_string(),
        "AppData\\Roaming\\Mozilla\\Firefox".to_string(),
        "AppData\\Local\\Steam\\htmlcache".to_string(),
        "AppData\\Local\\Packages".to_string(),
        "AppData\\Local\\Programs\\cursor".to_string(),
        "AppData\\Roaming\\Cursor".to_string(),
        "AppData\\Local\\Temp".to_string(),
        "AppData\\Roaming\\asus_framework".to_string(),
        "NTUSER.DAT".to_string(),
        "ntuser.dat.LOG*".to_string(),
        "AppData\\Local\\Application Data".to_string(),
        "AppData\\Local\\History".to_string(),
        "AppData\\Local\\ElevatedDiagnostics".to_string(),
        "AppData\\Local\\Temporary Internet Files".to_string(),
        "Application Data".to_string(),
        "Cookies".to_string(),
        "Local Settings".to_string(),
        "My Documents".to_string(),
        "NetHood".to_string(),
        "PrintHood".to_string(),
        "Recent".to_string(),
        "SendTo".to_string(),
        "Start Menu".to_string(),
        "Templates".to_string(),
        "Documents\\My Music".to_string(),
        "Documents\\My Pictures".to_string(),
        "Documents\\My Videos".to_string(),
    ]
}
