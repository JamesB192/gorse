#! /usr/bin/env python
import ctypes
import ctypes.util
import os
import sys
import unittest


def importado():
    ffi_paths = [
        os.path.join(os.path.abspath(x), "libgorse.so")
        for x in [
            os.path.dirname(os.path.realpath(__file__)),
            os.path.realpath("/usr/local/lib"),
        ]
    ]

    ffi_path = ctypes.util.find_library('gorse')
    if ffi_path:
        ffi_paths.append(ffi_path)

    for ffi_path in ffi_paths:
        try:
            sys.stderr.write('try_gorse_lib: %s\n' % ffi_path)
            lib = ctypes.cdll.LoadLibrary(ffi_path)
            return lib
        except OSError:
            pass

    raise OSError("Can't find gorse library")


_gorse = importado()
_gorse.chextolfp.restype = ctypes.c_bool
_gorse.chextolfp.argtypes = [ctypes.c_char_p, ctypes.c_void_p]


def hex2lfp(hex):
    mid_str = ctypes.c_char_p(bytes(hex, encoding="latin-1"))
    lfp = ctypes.c_ulonglong(0)
    ret = _gorse.chextolfp(mid_str, ctypes.byref(lfp))
    if ret == False:
        raise OSError(os.strerror(ctypes.get_errno()))
    return lfp.value


class TestShot(unittest.TestCase):
    def test_good(self):
        cases =	[
            [" \t0123456787654321", 0x0123456787654321],
            ["1234567876543210 \t", 0x1234567876543210],
            ["01234567.87654321", 0x0123456787654321],
            ["0x1234567876543210", 0x1234567876543210],
            ["0123456787654321", 0x0123456787654321],
        ]
        for case in cases:
            with self.subTest(case=case):
                 self.assertEqual(first=case[1], second=hex2lfp(case[0]))

    def test_bad(self):
        cases = [
            "123456787654321",
            "12345678.7654321",
            "1234567.87654321",
            "12345678:76543210",
            "y1234567876543210",
            "1234567876543210y",
        ]
        for case in cases:
            with self.subTest(case=case):
                self.assertRaises(OSError, hex2lfp, case)

if "__main__" == __name__:
    unittest.main()
