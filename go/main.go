package main

// #cgo CFLAGS: -I${SRCDIR}/../target
// #cgo LDFLAGS: -L${SRCDIR}/../target/debug -l:libazure_data_cosmos_shared.a
// #include <azure_data_cosmos_shared.h>
import "C"

func main() {
    println(C.add(2, 2))
}