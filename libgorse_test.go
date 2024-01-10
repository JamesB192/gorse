package main

import (
	"testing"
)

func TestShot(t *testing.T) {
	tests := map[string]struct {
		input   string
		success bool
		lfp     uint64
	}{
		"y1234567876543210":     {success: false, lfp: 0},
		"0123456787654321y":     {success: false, lfp: 0},
		"12345678y76543210":     {success: false, lfp: 0},
		"12345678.7654321":      {success: false, lfp: 0},
		"1234567.876543210":     {success: false, lfp: 0},
		"123456787654321":       {success: false, lfp: 0},
		"\t0123456787654321":    {success: true, lfp: 0x123456787654321},
		" 12345678.76543210":    {success: true, lfp: 0x1234567876543210},
		"0x01234567.87654321\t": {success: true, lfp: 0x123456787654321},
		"0x1234567876543210 ":   {success: true, lfp: 0x1234567876543210},
		"01234567.87654321":     {success: true, lfp: 0x123456787654321},
	}

	fails := 0
	for name, tc := range tests {
		success, lfp := HexToLFP(name)
		if tc.success != success {
			fails++
			t.Errorf("success expected: %v, got: %v", tc.success, success)
			continue
		}
		if tc.lfp != lfp {
			fails++
			t.Errorf("lfp expected: %016x, got: %016x", tc.lfp, lfp)
		}
	}
	/*
		if 0 != fails {
			t.Errorf("%d tests failed.")
		}
	*/
}
