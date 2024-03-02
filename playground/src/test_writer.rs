use std::cell::RefCell;
use std::io::{Cursor, Read, Result as IoResult, Write};
use std::rc::Rc;

pub struct TestWriterEnvelope {
	state: Rc<RefCell<State>>,
}

impl TestWriterEnvelope {
	pub fn new() -> Self {
		TestWriterEnvelope {
			state: Rc::new(RefCell::new(State {
				cursor: Cursor::<Vec<u8>>::default(),
				is_writer_dropped: false,
			}))
		}
	}
	
	pub fn publish(&self) -> Option<TestWriter> {
		if !self.is_dropped() && Rc::strong_count(&self.state) == 1 {
			Some(TestWriter::new(self.state.clone()))
		} else {
			None
		}
	}
	
	pub fn is_dropped(&self) -> bool {
		self.state.borrow().is_writer_dropped
	}
	
	pub fn read_string(&self, buffer: &mut String) -> IoResult<usize> {
		let recent = self.state.borrow().cursor.position();
		
		let mut reference = self.state.borrow_mut();
		reference.cursor.set_position(0);
		
		let result = reference.cursor.read_to_string(buffer);
		
		reference.cursor.set_position(recent);
		
		result
	}
}

struct State {
	cursor: Cursor<Vec<u8>>,
	is_writer_dropped: bool,
}


pub struct TestWriter {
	state: Rc<RefCell<State>>,
}

impl TestWriter {
	fn new(state: Rc<RefCell<State>>) -> Self {
		assert_ne!(Rc::strong_count(&state), 0, "Already rented");
		
		TestWriter {
			state
		}
	}
}

impl Drop for TestWriter {
	fn drop(&mut self) {
		assert!(!self.state.borrow().is_writer_dropped, "Already dropped.");
		self.state.borrow_mut().is_writer_dropped = true
	}
}

impl Write for TestWriter {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.state.borrow_mut().cursor.write(buf)
	}
	
	fn flush(&mut self) -> std::io::Result<()> {
		self.state.borrow_mut().cursor.flush()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn write_test() {
		let fixture = TestWriterEnvelope::new();
		let mut writer = fixture.publish().unwrap();
		
		writer.write(b"hello world").unwrap();
		
		let mut buff = String::new();
		
		fixture.read_string(&mut buff).unwrap();
		
		assert_eq!(&buff, "hello world");
		
		
		writer.write(b" hoge moge").unwrap();
		buff.clear();
		fixture.read_string(&mut buff).unwrap();
		
		assert_eq!(&buff, "hello world hoge moge");
	}
	
	#[test]
	fn is_dropped_test() {
		let fixture = TestWriterEnvelope::new();
		assert!(!fixture.is_dropped());
		
		{
			_ = fixture.publish().unwrap();
		}
		
		assert!(fixture.is_dropped());
	}
	
	#[test]
	fn init_test() {
		let fixture = TestWriterEnvelope::new();
		assert!(!fixture.is_dropped());
		assert_eq!(Rc::strong_count(&fixture.state), 1);
		
		let mut buff = String::new();
		fixture.read_string(&mut buff).unwrap();
		assert_eq!(&buff, "");
	}
	
	#[test]
	fn publish_test() {
		let fixgure = TestWriterEnvelope::new();
		
		{
			_ = fixgure.publish().unwrap();
			assert!(fixgure.publish().is_none());
		}
		
		assert!(fixgure.publish().is_none());
	}
}