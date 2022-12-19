package day07;

/**
 * Holds the name and size of a "File". A File can be a directory or an actual file.
 */
public class FileInfo {

    String name;
    long size;
    boolean isDir;

    public FileInfo newFile(String name, long size) {
        FileInfo fn = new FileInfo();
        fn.name = name;
        fn.size = size;
        fn.isDir = false;
        return fn;
    }

    public FileInfo newDir(String name, long size) {
        FileInfo fn = new FileInfo();
        fn.name = name;
        fn.size = size;
        fn.isDir = true;
        return fn;
    }

    @Override
    public String toString() {
        if (isDir) {
            return "DIR  " + name + " (" + size + ")";
        } else {
            return "FILE " + name + " (" + size + ")";
        }
    }
}
