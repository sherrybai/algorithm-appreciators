use std::io;
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// from https://users.rust-lang.org/t/reading-and-parsing-a-line-from-stdin-containing-3-integers/7265/2
macro_rules! parse_line {
    ($($t: ty),+) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        let mut a_iter = a_str.split_whitespace();
        (
            $(
            a_iter.next().unwrap().parse::<$t>().expect("parse error"),
            )+
        )
    })
}
macro_rules! parse_line_as_vec {
    ($t: ty) => ({
        let mut a_str = String::new();
        io::stdin().read_line(&mut a_str).expect("read error");
        a_str.split_whitespace()
            .map(|x| x.parse::<$t>().expect("parse error"))
            .collect::<Vec<$t>>()
    })
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PqItem {
    MinItem(Reverse<usize>),
    MaxItem(usize)
}

impl PqItem {
    fn unwrap(&self) -> usize {
        match self {
            Self::MinItem(Reverse(val)) => *val,
            Self::MaxItem(val) => *val
        }
    }
}

struct PriorityQueueWrapper {
    heap: BinaryHeap<PqItem>,
    min_heap: bool
}

impl PriorityQueueWrapper {
    fn new(min_heap: bool) -> Self {
        Self {
            heap: BinaryHeap::new(),
            min_heap,
        }
    }
}

struct Solution {
    res: Vec<usize>,
    visited: HashMap<usize, VecDeque<usize>>,
    pq_smaller: PriorityQueueWrapper,
    pq_larger: PriorityQueueWrapper,
    sum_smaller: usize,
    sum_larger: usize,
    current_index: usize,
}

impl Solution {
    fn new() -> Self {
        let mut res : Vec<usize> = Vec::new();
        let mut visited: HashMap<usize, VecDeque<usize>> = HashMap::new();
    
        let mut pq_smaller: PriorityQueueWrapper = PriorityQueueWrapper::new(false);
        let mut pq_larger: PriorityQueueWrapper = PriorityQueueWrapper::new(true);
        let mut sum_smaller: usize = 0;
        let mut sum_larger: usize = 0;
    
        let current_index: usize = 0;
        Self {
            res,
            visited,
            pq_smaller,
            pq_larger,
            sum_smaller,
            sum_larger,
            current_index,
        }
    }

    fn clear_old_values(&mut self, pq: &mut PriorityQueueWrapper) {
        let mut heap = &mut pq.heap;
        while !heap.is_empty() {
            let val = heap.peek().unwrap().unwrap();
            let visited_list: Option<&VecDeque<usize>> = self.visited.get(&val);
            if visited_list == None {
                break
            }
            let visited_list_front: Option<&usize> = visited_list.unwrap().front();
            if *visited_list_front.unwrap() >= self.current_index {
                break
            }
            self.visited.get_mut(&val).unwrap().pop_front();
            heap.pop();
        }
    }

    fn heappop(&mut self, pq: &mut PriorityQueueWrapper) -> (usize, usize) {
        self.clear_old_values(pq);
        let val = pq.heap.pop().unwrap().unwrap();
        let index = self.visited.get_mut(&val).unwrap().pop_front().unwrap();
        (val, index)
    }

    fn access_head(&mut self, pq: &mut PriorityQueueWrapper) -> (usize, usize) {
        self.clear_old_values(pq);
        let val = pq.heap.peek().unwrap().unwrap();
        let index = self.visited.get_mut(&val).unwrap().front().unwrap();
        (val, *index)
    }

    fn heappush(&mut self, pq: &mut PriorityQueueWrapper, val: usize, index: usize) {
        self.visited.get_mut(&val).unwrap().push_back(index);
        if pq.min_heap {
            pq.heap.push(PqItem::MinItem(Reverse(val)));
        } else {
            pq.heap.push(PqItem::MaxItem(val));
        }
    }
}


fn main() {
    let (n, k) = parse_line!(usize, usize);
    let x = parse_line_as_vec!(usize);
    // println!("{},{},{:?}", n, k, x);

    let solution = Solution::new();
    
    // initial window values
    let mut initial_window = &mut x[..k].to_vec();
    initial_window.sort();
}
