package main

// #cgo CFLAGS: -I${SRCDIR}/../target
// #cgo LDFLAGS: -L${SRCDIR}/../target/debug -l:libazure_data_cosmos_shared.a
// #include <azure_data_cosmos_shared.h>
import "C"

func main() {
	input := "{ \"name\": \"Kevin\" }"

	encoded := C.binary_encode(C.CString(input))
	defer C.free_byte_buffer(encoded)

	orig := C.binary_decode(&encoded)
	defer C.free_string(orig)

	origGo := C.GoString(orig)

	println("Roundtripped: '", input, "' to get '", origGo, "'.")
	println("It was ", encoded.len, "bytes in encoded form")
}
