
pub struct WindowConfig {

    pub width: u32,
    pub height: u32,
    pub title: &'static str,

}


impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "𝕯𝖎𝖘𝖈𝖎𝖚𝖒 𝕲𝖗𝖆𝖕𝖍𝖎𝖈𝖘",
        }
    }
}