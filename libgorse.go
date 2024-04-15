package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L. -l gorse -v
// -Wl,-rpath=.
#include "gorse.h"
*/
import "C"
import "unsafe"

// bool chextolfp(const char *c_buf, uint64_t *lfp);

func HexToLFP(in_string string) (bool, uint64) {
	mid_string := C.CString(in_string)
	defer C.free(unsafe.Pointer(mid_string))
	mid_fixed_point := C.uint64_t(0)
	success := C.chextolfp(mid_string, &mid_fixed_point)
	return bool(success), uint64(mid_fixed_point)
}
