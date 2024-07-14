package queue

import (
	"sync/atomic"
)

type Node[V any] struct {
	value V
	next  atomic.Pointer[Node[V]]
}

func NewNode[V any](v V) *Node[V] {
	return &Node[V]{value: v}
}

type Queue[V any] struct {
	head atomic.Pointer[Node[V]]
	tail atomic.Pointer[Node[V]]
}

func NewQueue[V any]() *Queue[V] {

	sentinel := NewNode(*new(V))
	queue := &Queue[V]{}

	queue.head.Store(sentinel)
	queue.tail.Store(sentinel)

	return queue
}

func (q *Queue[V]) Enqueue(value V) {

	node := NewNode(value)

	for {

		last := q.tail.Load()
		next := last.next.Load()

		// actual last node
		if next == nil {
			if ok := last.next.CompareAndSwap(next, node); ok {
				q.tail.CompareAndSwap(last, node)
				return
			} else {
				continue
			}
		}

		// last not actual last node
		// fix tail pointer
		q.tail.CompareAndSwap(last, last.next.Load())
	}
}

func (q *Queue[V]) Dequeue() *V {

	for {

		sentinel := q.head.Load()
		next := sentinel.next.Load()

		// queue is empty
		if next == nil {
			return nil
		}

		// check if tail pointer needs fixing
		last := q.tail.Load()
		if nextOfLast := last.next.Load(); nextOfLast != nil {
			q.tail.CompareAndSwap(last, nextOfLast.next.Load())
			continue
		}

		if ok := q.head.CompareAndSwap(sentinel, next); ok {
			return &next.value
		}

	}

}
