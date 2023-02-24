package main.java.github.kasuminova.nativelib;

import java.io.File;
import java.io.IOException;

public class Main {
    static {
        String currentDir = System.getProperty("user.dir");
        System.load(currentDir + File.separator + "pangu_native.dll");
    }

    public static void main(String[] args) throws IOException {
        setClipboardContent("这是一个剪贴板信息。");
    }

    public static native void setClipboardContent(String content) throws IOException;
}