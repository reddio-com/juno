package vm

//#include <stdint.h>
//#include <stdlib.h>
import "C"

import (
	"errors"
	"unsafe"

	"github.com/NethermindEth/juno/core/felt"
	"github.com/NethermindEth/juno/db"
)

//export JunoFree
func JunoFree(ptr unsafe.Pointer) {
	C.free(ptr)
}

//export JunoStateGetStorageAt
func JunoStateGetStorageAt(readerHandle C.uintptr_t, contractAddress, storageLocation unsafe.Pointer) unsafe.Pointer {
	context := unwrapContext(readerHandle)

	contractAddressFelt := makeFeltFromPtr(contractAddress)
	storageLocationFelt := makeFeltFromPtr(storageLocation)
	val, err := context.state.ContractStorage(contractAddressFelt, storageLocationFelt)
	if err != nil {
		if !errors.Is(err, db.ErrKeyNotFound) {
			context.log.Errorw("JunoStateGetStorageAt failed to read contract storage", "err", err)
			return nil
		}
		val = &felt.Zero
	}

	return makePtrFromFelt(val)
}

//export JunoStateGetNonceAt
func JunoStateGetNonceAt(readerHandle C.uintptr_t, contractAddress unsafe.Pointer) unsafe.Pointer {
	context := unwrapContext(readerHandle)

	contractAddressFelt := makeFeltFromPtr(contractAddress)
	val, err := context.state.ContractNonce(contractAddressFelt)
	if err != nil {
		if !errors.Is(err, db.ErrKeyNotFound) {
			context.log.Errorw("JunoStateGetNonceAt failed to read contract nonce", "err", err)
			return nil
		}
		val = &felt.Zero
	}

	return makePtrFromFelt(val)
}

//export JunoStateGetClassHashAt
func JunoStateGetClassHashAt(readerHandle C.uintptr_t, contractAddress unsafe.Pointer) unsafe.Pointer {
	context := unwrapContext(readerHandle)

	contractAddressFelt := makeFeltFromPtr(contractAddress)
	val, err := context.state.ContractClassHash(contractAddressFelt)
	if err != nil {
		if !errors.Is(err, db.ErrKeyNotFound) {
			context.log.Errorw("JunoStateGetClassHashAt failed to read contract class", "err", err)
			return nil
		}
		val = &felt.Zero
	}

	return makePtrFromFelt(val)
}

//export JunoStateGetCompiledClass
func JunoStateGetCompiledClass(readerHandle C.uintptr_t, classHash unsafe.Pointer) unsafe.Pointer {
	context := unwrapContext(readerHandle)

	classHashFelt := makeFeltFromPtr(classHash)
	val, err := context.state.Class(classHashFelt)
	if err != nil {
		if !errors.Is(err, db.ErrKeyNotFound) {
			context.log.Errorw("JunoStateGetCompiledClass failed to read class", "err", err)
		}
		return nil
	}

	compiledClass, err := marshalCompiledClass(val.Class)
	if err != nil {
		context.log.Errorw("JunoStateGetCompiledClass failed to marshal compiled class", "err", err)
		return nil
	}

	return unsafe.Pointer(cstring(compiledClass))
}

//export JunoStateSetContractClass
func JunoStateSetContractClass(handle C.uintptr_t, classHash, class unsafe.Pointer) {
	ctx := unwrapContext(handle)
	classHashFelt := makeFeltFromPtr(classHash)
}

//export JunoStateSetStorage
func JunoStateSetStorage(handle C.uintptr_t, addr, key, value unsafe.Pointer) {
	ctx := unwrapContext(handle)
	addrFelt := makeFeltFromPtr(addr)
	keyFelt := makeFeltFromPtr(key)
	valueFelt := makeFeltFromPtr(value)
	err := ctx.state.SetStorage(*addrFelt, *keyFelt, valueFelt, 0)
	if err != nil {
		ctx.log.Errorw("JunoStateSetStorage failed to set storage", "err", err)
	}
}

//export JunoStateSetNonce
func JunoStateSetNonce(handle C.uintptr_t, addr, nonce unsafe.Pointer) {
	ctx := unwrapContext(handle)
	addrFelt := makeFeltFromPtr(addr)
	nonceAddr := makeFeltFromPtr(nonce)

}

//export JunoStateSetClassHashAt
func JunoStateSetClassHashAt(handle C.uintptr_t, addr, classHash unsafe.Pointer) {
	ctx := unwrapContext(handle)
	addrFelt := makeFeltFromPtr(addr)
	classHashFelt := makeFeltFromPtr(classHash)
	err := ctx.state.SetClassHashAt(addrFelt, classHashFelt, 0)
	if err != nil {
		ctx.log.Errorw("JunoStateSetClassHashAt failed to set class hash at contract addr", "err", err)
	}
}

//export JunoStateSetCompiledClassHash
func JunoStateSetCompiledClassHash(handle C.uintptr_t, classHash, compiledClassHash unsafe.Pointer) {
	ctx := unwrapContext(handle)
	classHashFelt := makeFeltFromPtr(classHash)
	compClassHash := makeFeltFromPtr(compiledClassHash)
	err := ctx.state.SetCompiledClassHash(classHashFelt, compClassHash, 0)
	if err != nil {
		ctx.log.Errorw("JunoStateSetCompiledClassHash failed to set compiled class hash", "err", err)
	}
}
