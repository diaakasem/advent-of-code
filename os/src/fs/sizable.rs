pub trait Sizable {
    fn get_size(&self) -> u64;
    fn get_readable_size(&self) -> String {
        let size = self.get_size();
        let mut fsize = size as f64;
        let mut unit = "B";
        if fsize > 1024.0 {
            fsize /= 1024.0;
            unit = "KB";
        }
        if fsize > 1024.0 {
            fsize /= 1024.0;
            unit = "MB";
        }
        if fsize > 1024.0 {
            fsize /= 1024.0;
            unit = "GB";
        }
        if fsize > 1024.0 {
            fsize /= 1024.0;
            unit = "TB";
        }
        format!("{:.2} {}", fsize, unit)
    }
}
