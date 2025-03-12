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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Debug)]
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
    res: Vec<String>,
    visited: HashMap<usize, VecDeque<usize>>,
    pq_smaller: PriorityQueueWrapper,
    pq_larger: PriorityQueueWrapper,
    sum_smaller: usize,
    sum_larger: usize,
    current_index: usize,
}

impl Solution {
    fn new() -> Self {
        let res : Vec<String> = Vec::new();
        let visited: HashMap<usize, VecDeque<usize>> = HashMap::new();
    
        let pq_smaller: PriorityQueueWrapper = PriorityQueueWrapper::new(false);
        let pq_larger: PriorityQueueWrapper = PriorityQueueWrapper::new(true);
        let sum_smaller: usize = 0;
        let sum_larger: usize = 0;
    
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
}

fn clear_old_values(visited: &mut HashMap<usize, VecDeque<usize>>, pq: &mut PriorityQueueWrapper, current_index: usize) {
    let heap = &mut pq.heap;
    while !heap.is_empty() {
        let val = heap.peek().unwrap().unwrap();
        let visited_list: Option<&VecDeque<usize>> = visited.get(&val);
        if visited_list == None {
            break
        }
        let visited_list_front: Option<&usize> = visited_list.unwrap().front();
        if *visited_list_front.unwrap() >= current_index {
            break
        }
        visited.get_mut(&val).unwrap().pop_front();
        heap.pop();
    }
}

fn heappop(visited: &mut HashMap<usize, VecDeque<usize>>, pq: &mut PriorityQueueWrapper, current_index: usize) -> (usize, usize) {
    clear_old_values(visited, pq, current_index);
    let val = pq.heap.pop().unwrap().unwrap();
    let index = visited.get_mut(&val).unwrap().pop_front().unwrap();
    (val, index)
}

fn access_head(visited: &mut HashMap<usize, VecDeque<usize>>, pq: &mut PriorityQueueWrapper, current_index: usize) -> (usize, usize) {
    clear_old_values(visited, pq, current_index);
    let val = pq.heap.peek().unwrap().unwrap();
    let index = visited.get_mut(&val).unwrap().front().unwrap();
    (val, *index)
}

fn heappush(visited: &mut HashMap<usize, VecDeque<usize>>, pq: &mut PriorityQueueWrapper, val: usize, index: usize) {
    if visited.get_mut(&val) == None {
        visited.insert(val, VecDeque::new());
    }
    visited.get_mut(&val).unwrap().push_back(index);
    if pq.min_heap {
        pq.heap.push(PqItem::MinItem(Reverse(val)));
    } else {
        pq.heap.push(PqItem::MaxItem(val));
    }
}


fn main() {
    let (n, k) = parse_line!(usize, usize);
    let x = parse_line_as_vec!(usize);
    // println!("{},{},{:?}", n, k, x);

    let mut solution = Solution::new();
    
    // initial window values
    let initial_window = &mut x[..k].to_vec();
    initial_window.sort();

    let med = (k-1) / 2;
    for i in 0..med {
        solution.pq_smaller.heap.push(PqItem::MaxItem(initial_window[i]));
        solution.sum_smaller += initial_window[i];
    }
    solution.pq_smaller.heap.push(PqItem::MaxItem(initial_window[med]));
    for i in med+1..k {
        solution.pq_larger.heap.push(PqItem::MinItem(Reverse(initial_window[i])));
        solution.sum_larger += initial_window[i];
    }
    for i in 0..k {
        let val = x.get(i).unwrap();
        if solution.visited.get_mut(val) == None {
            solution.visited.insert(*val, VecDeque::new());
        }
        solution.visited.get_mut(val).unwrap().push_back(i);    
    }

    // calculate cost
    let mut cost = solution.sum_larger - solution.sum_smaller;
    if k % 2 == 0 {
        cost -= initial_window[med];
    }
    solution.res.push(cost.to_string());


    // iterate through each subsequent window
    for lo in 1..n-k+1 {
        let (median, _) = access_head(&mut solution.visited, &mut solution.pq_smaller, solution.current_index);

        // kick out lo-1
        let kicked_out = *x.get(lo-1).unwrap();
        let added_in = *x.get(lo+k-1).unwrap();
        solution.current_index = lo;
        if kicked_out < median { 
            solution.sum_smaller -= kicked_out;
            if added_in <= median { // median stays the same
                solution.sum_smaller += added_in;
                heappush(&mut solution.visited, &mut solution.pq_smaller, added_in, lo+k-1);
            } else { // median gets larger
                // add the new element
                solution.sum_larger += added_in;
                heappush(&mut solution.visited, &mut solution.pq_larger, added_in, lo+k-1);
                // update the median
                solution.sum_smaller += median;
                let (new_median, new_index) = heappop(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
                heappush(&mut solution.visited, &mut solution.pq_smaller, new_median, new_index);
                solution.sum_larger -= new_median;
            }
        } else if kicked_out > median {
            solution.sum_larger -= kicked_out;
            if added_in >= median { // median stays the same
                solution.sum_larger += added_in;
                heappush(&mut solution.visited, &mut solution.pq_larger, added_in, lo+k-1);
            } else { // median gets smaller
                // add the new element
                solution.sum_smaller += added_in;
                heappush(&mut solution.visited, &mut solution.pq_smaller, added_in, lo+k-1);
                // update the median
                let (median, med_index) = heappop(&mut solution.visited, &mut solution.pq_smaller, solution.current_index);
                heappush(&mut solution.visited, &mut solution.pq_larger, median, med_index);
                solution.sum_larger += median;
                let (new_median, _) = access_head(&mut solution.visited, &mut solution.pq_smaller, solution.current_index);
                solution.sum_smaller -= new_median;
            }
        } else {  // equal to median
            // kick out the median
            clear_old_values(&mut solution.visited, &mut solution.pq_smaller, solution.current_index);
            clear_old_values(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
            if !solution.pq_larger.heap.is_empty() && added_in <= access_head(&mut solution.visited, &mut solution.pq_larger, solution.current_index).0 {
                let (pq_larger_head, pq_larger_index) = access_head(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
                solution.sum_smaller += added_in;
                if !solution.pq_larger.heap.is_empty() && added_in == pq_larger_head { // heap sizes are unchanged
                    heappush(&mut solution.visited, &mut solution.pq_smaller, pq_larger_head, pq_larger_index);
                    heappop(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
                    heappush(&mut solution.visited, &mut solution.pq_larger, added_in, lo+k-1);
                } else {
                    heappush(&mut solution.visited, &mut solution.pq_smaller, added_in, lo+k-1);
                }
                let (new_median, _) = access_head(&mut solution.visited, &mut solution.pq_smaller, solution.current_index);
                solution.sum_smaller -= new_median
            } else { // head of pq_larger becomes new median
                solution.sum_larger += added_in;
                heappush(&mut solution.visited, &mut solution.pq_larger, added_in, lo+k-1);
                let (new_median, new_index) = access_head(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
                solution.sum_larger -= new_median;
                heappop(&mut solution.visited, &mut solution.pq_larger, solution.current_index);
                heappush(&mut solution.visited, &mut solution.pq_smaller, new_median, new_index);
            }
        }

        // calculate the new cost
        let mut cost = solution.sum_larger - solution.sum_smaller;
        let median = access_head(&mut solution.visited, &mut solution.pq_smaller, solution.current_index).0;
        // dbg!(cost);
        if k % 2 == 0 {
            cost -= median;
        }
        solution.res.push(cost.to_string());
    }

    println!("{}", solution.res.join(" "))
}
