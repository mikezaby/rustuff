struct Node {
  value: int,
  next: Option<@mut Node>
}

struct Queue {
  head: Option<@mut Node>,
  tail: Option<@mut Node>,
  length: int
}

impl Queue {
  pub fn new() -> Queue {
    Queue { head: None, tail: None, length: 0 }
  }

  pub fn enqueue(&mut self, value: int) {
    let node = @mut Node { value: value, next: None};
    if self.length == 0 {
      self.head = Some(node);
    }
    else {
      self.tail.unwrap().next = Some(node);
    }
    self.tail = Some(node);
    self.length += 1;
  }

  pub fn dequeue(&mut self) -> int {
    if self.length == 1 { self.tail = None; }
    if self.length > 0 {
      let node = self.head.unwrap();
      self.head = node.next;
      self.length -= 1;
      node.value
    }
    else {
      fail!("ALT+QQ")
    }
  }
}

fn main() {
  let mut q = Queue::new();
  q.enqueue(66);
  q.enqueue(3);

  println!("value: {:d}", q.dequeue());
  println!("value: {:d}", q.dequeue());
}
