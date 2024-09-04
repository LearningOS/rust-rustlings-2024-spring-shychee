/*
	queue
	This question requires you to use queues to implement the functionality of the stack
    队列实现堆栈
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    // 入队
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    // 出队
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    // 查看队列的第一个元素
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    // 查看队列的大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    // 查看队列是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	// 两个队列
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			// 初始化两个队列
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 将元素入队到 q1
		self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
		// 如果 q1 为空，返回错误
		if self.q1.is_empty() {
			Err("Stack is empty")
		} else {
			// 将 q1 中的元素出队到 q2，直到 q1 中只剩下一个元素
			while self.q1.size() > 1 {
				self.q2.enqueue(self.q1.dequeue().unwrap());
			}
			// 保存 q1 中的最后一个元素
            let last_element = self.q1.dequeue().unwrap();
            // 交换 q1 和 q2
            std::mem::swap(&mut self.q1, &mut self.q2);
            Ok(last_element)
		}
    }
    pub fn is_empty(&self) -> bool {
		// 如果 q1 和 q2 都为空，返回 true
		self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}