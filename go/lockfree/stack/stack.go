package stack

import "sync/atomic"

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

	node := NewNode(value)

	for {
		if s.tryPush(node) {
			return
		}
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
	for {
		value, ok := s.tryPop()
		if !ok {
			continue
		}

		return value
	}
}
