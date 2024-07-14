package stack

import (
	"lockfree/backoff"
	"sync/atomic"
)

type Node[V any] struct {
	value V
	next  atomic.Pointer[Node[V]]
}

func NewNode[V any](v V) *Node[V] {
	return &Node[V]{value: v}
}

type Stack[V any] struct {
	head atomic.Pointer[Node[V]]
}

func NewStack[V any]() *Stack[V] {
	return &Stack[V]{}
}

func (s *Stack[V]) tryPush(node *Node[V]) bool {

	currTop := s.head.Load()

	node.next.Store(currTop)

	return s.head.CompareAndSwap(currTop, node)
}

func (s *Stack[V]) Push(value V) {

	busy := backoff.New(2)
	node := NewNode(value)

	for {
		if s.tryPush(node) {
			return
		}

		busy.Spin()
	}
}

func (s *Stack[V]) tryPop() (*V, bool) {

	currTop := s.head.Load()

	if currTop == nil {
		return nil, true
	}

	if s.head.CompareAndSwap(currTop, currTop.next.Load()) {
		return &currTop.value, true
	}
	return nil, false
}

func (s *Stack[V]) Pop() *V {

	busy := backoff.New(2)
	for {
		if value, ok := s.tryPop(); ok {
			return value
		}

		busy.Spin()
	}
}
