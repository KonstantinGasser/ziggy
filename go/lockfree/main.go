package main

import (
	"fmt"
	"lockfree/queue"
	"lockfree/stack"
	"sync"
)

func testQueue() {
	q := queue.NewQueue[int]()

	for i := range 100 {
		q.Enqueue(i)
	}

	wg := sync.WaitGroup{}
	wg.Add(2)

	go func(q *queue.Queue[int], w *sync.WaitGroup) {
		defer w.Done()
		for {
			value := q.Dequeue()
			if value == nil {
				fmt.Println("[1] Queue empty - leaving...")
				return
			}

			fmt.Println("[1]: ", *value)
		}

	}(q, &wg)

	go func(q *queue.Queue[int], w *sync.WaitGroup) {
		defer w.Done()
		for {
			value := q.Dequeue()
			if value == nil {
				fmt.Println("[2] Queue empty - leaving...")
				return
			}

			fmt.Println("[2]: ", *value)
		}

	}(q, &wg)

	wg.Wait()

}

func testStack() {
	s := stack.NewStack[int]()

	for i := range 100 {
		s.Push(i)
	}

	wg := sync.WaitGroup{}
	wg.Add(2)

	go func(s *stack.Stack[int], w *sync.WaitGroup) {
		defer w.Done()

		for {
			value := s.Pop()
			if value == nil {
				fmt.Println("[1]: Stack is empty")
				return

			} else {
				fmt.Println("[1]: ", *value)
			}
		}
	}(s, &wg)

	go func(s *stack.Stack[int], w *sync.WaitGroup) {
		defer w.Done()

		for {
			value := s.Pop()
			if value == nil {
				fmt.Println("[2]: Stack is empty")
				return

			} else {
				fmt.Println("[2]: ", *value)
			}
		}
	}(s, &wg)

	wg.Wait()

}

func main() {

	testQueue()
	testStack()

}
