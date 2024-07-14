package backoff

import (
	"fmt"
	"runtime"
)

type Exponential struct {
	tryCount uint8
	maxTry   uint8
}

func New(max uint8) *Exponential {
	return &Exponential{tryCount: 1, maxTry: max}
}

func (e *Exponential) Spin() {

	if e.tryCount >= e.maxTry {
		runtime.Gosched()
		fmt.Println("Goshed!")
		return
	}

	for range 1 << e.tryCount {
		// busy wait...
	}

	e.tryCount += 1
}
