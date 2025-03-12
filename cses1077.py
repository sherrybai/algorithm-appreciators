# per window:
# find median
# cost = sum(> median) - sum(< median)
# keep 2 priority queues per window

import heapq
from collections import defaultdict, deque

# parse input
n, k = (int(i) for i in input().split())
x = [int(xi) for xi in input().split()]

# final output
res = []

pq_smaller = []
sum_smaller = 0
pq_larger = []
sum_larger = 0

visited = defaultdict(deque)
current_index = 0

def clear_old_values(heap):
    while heap:
        val = abs(heap[0])
        if not visited[val] or visited[val][0] >= current_index:
            break
        visited[val].popleft()
        heapq.heappop(heap)

def heappop(heap):
    clear_old_values(heap)
    val = abs(heapq.heappop(heap))
    index = visited[val].popleft()
    return val, index

def heappush(heap, val, index, invert=False):
    visited[val].append(index)
    if not invert:
        heapq.heappush(heap, val)
    else:
        heapq.heappush(heap, -val)

def access_head(heap):
    clear_old_values(heap)
    val = abs(heap[0])
    index = visited[val][0]
    return val, index

# initial window values
initial_window = [(v, i) for i, v in enumerate(x[:k])]
initial_window.sort()
med = (k-1) // 2  # median index
# add values before median
for i in range(med):
    val, vi = initial_window[i]
    heappush(pq_smaller, val, vi, invert=True)
    sum_smaller += initial_window[i][0]
# add median
median, mi = initial_window[med]
heappush(pq_smaller, median, mi, invert=True)
# values after median
for i in range(med+1, k):
    val, vi = initial_window[i]
    heappush(pq_larger, val, vi, invert=False)
    sum_larger += initial_window[i][0]

# calculate the cost
cost = sum_larger - sum_smaller
if k % 2 == 0:
    cost -= initial_window[med][0]
res.append(cost)

# print(x[:k], pq_smaller, sum_smaller, pq_larger, sum_larger, visited)

# iterate through each subsequent window
for lo in range(1, n-k+1):
    median, med_index = access_head(pq_smaller)
    # print(median)
    # kick out lo-1 (lazily)
    current_index = lo
    if x[lo-1] < median:
        # remove lo-1
        sum_smaller -= x[lo-1]
        if x[lo+k-1] <= median:  # median stays the same
            sum_smaller += x[lo+k-1]
            heappush(pq_smaller, x[lo+k-1], lo+k-1, invert=True)
        else:  # median gets larger
            # add the new element
            sum_larger += x[lo+k-1]
            heappush(pq_larger, x[lo+k-1], lo+k-1, invert=False)
            # update the median
            sum_smaller += median
            new_median, new_index = heappop(pq_larger)
            heappush(pq_smaller, new_median, new_index, invert=True)
            sum_larger -= new_median
    elif x[lo-1] > median:
        sum_larger -= x[lo-1]
        if x[lo+k-1] >= median:  # median stays the same
            sum_larger += x[lo+k-1]
            heappush(pq_larger, x[lo+k-1], lo+k-1, invert=False)
        else:  # median gets smaller
            # add the new element
            sum_smaller += x[lo+k-1]
            heappush(pq_smaller, x[lo+k-1], lo+k-1, invert=True)
            # update the median
            median, med = heappop(pq_smaller)
            heappush(pq_larger, median, med, invert=False)
            sum_larger += median
            new_median, new_med = access_head(pq_smaller)
            sum_smaller -= new_median
    else:  # is median
        clear_old_values(pq_smaller)  # kick out the median
        clear_old_values(pq_larger)
        # print(pq_smaller, pq_larger)
        if pq_larger and x[lo+k-1] <= access_head(pq_larger)[0]:  # heap sizes are unchanged
            pq_larger_head, pq_larger_index = access_head(pq_larger)
            sum_smaller += x[lo+k-1]
            if pq_larger and x[lo+k-1] == pq_larger_head:
                heappush(pq_smaller, pq_larger_head, pq_larger_index, invert=True)
                heappop(pq_larger)
                heappush(pq_larger, x[lo+k-1], lo+k-1, invert=False)
            else:
                heappush(pq_smaller, x[lo+k-1], lo+k-1, invert=True)
            new_median, _ = access_head(pq_smaller)
            sum_smaller -= new_median
        else:  # head of pq_larger becomes new median
            sum_larger += x[lo+k-1]
            heappush(pq_larger, x[lo+k-1], lo+k-1)
            new_median, new_med = access_head(pq_larger)
            sum_larger -= new_median
            heappop(pq_larger)
            heappush(pq_smaller, new_median, new_med, invert=True)


    # print(x[lo:lo+k], pq_smaller, sum_smaller, pq_larger, sum_larger, visited)

    # calculate the new cost
    cost = sum_larger - sum_smaller
    median = access_head(pq_smaller)[0]
    if k % 2 == 0:
        cost -= median
    res.append(cost)

# print result
print(" ".join(str(i) for i in res))