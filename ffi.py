from ctypes import cdll
lib = cdll.LoadLibrary('./target/debug/libstringtools.dylib')

if __name__ == '__main__':
    print lib.length("foo", "bar")
