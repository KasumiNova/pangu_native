use jni::JNIEnv;
use jni::objects::{JClass, JString};
use cli_clipboard::set_contents;

#[no_mangle]
pub unsafe extern "C" fn Java_github_kasuminova_nativelib_Main_setClipboardContent(
    env: JNIEnv,
    _class: JClass,
    j_content: JString)
{
    let binding = env.get_string(j_content).unwrap();
    let content = binding.to_str().unwrap();
    match set_contents(content.parse().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            env.throw_new("java/io/IOException", err.to_string()).unwrap()
        }
    }
}
