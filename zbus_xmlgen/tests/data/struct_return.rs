#[proxy(interface = "test.StructReturn", assume_defaults = true)]
pub trait StructReturn {
    /// ReturnsNestedStruct method
    fn returns_nested_struct(&self) -> zbus::Result<(((String, String), i32),)>;

    /// ReturnsOneString method
    fn returns_one_string(&self) -> zbus::Result<String>;

    /// ReturnsStruct method
    fn returns_struct(&self) -> zbus::Result<((String, String),)>;

    /// ReturnsTwoStrings method
    fn returns_two_strings(&self) -> zbus::Result<(String, String)>;
}
