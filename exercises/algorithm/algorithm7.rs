/*
	stack
	This question requires you to use a stack to achieve a bracket match
	利用堆栈实现括号匹配
*/

// 定义堆栈
#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
// 实现堆栈
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// 如果堆栈为空，返回 None
		if 0 == self.size {
			return None;
		}
		// 如果堆栈不为空，返回堆栈顶部的元素
		self.size -= 1;
		self.data.pop()
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	// 创建一个堆栈
	let mut stack = Stack::new();
	// 遍历括号字符串
	for c in bracket.chars() {
		// 如果当前字符是左括号，将其压入堆栈
		if c == '(' || c == '[' || c == '{' {
			stack.push(c);
		} else if c == ')' || c == ']' || c == '}' {
			// 如果当前字符是右括号，检查堆栈是否为空
			if stack.is_empty() {
				return false;
			}
			// 如果堆栈不为空，弹出堆栈顶部的元素
			let top = stack.pop().unwrap();
			// 如果弹出的元素与当前字符不匹配，返回 false
			if (c == ')' && top != '(') ||
			   (c == ']' && top != '[') || 
			   (c == '}' && top != '{') {
				return false;
			}
		}
	}
	// 如果堆栈为空，返回 true，否则返回 false
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}