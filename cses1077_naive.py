# per window:
# find median
# cost = sum(> median) - sum(< median)
# keep 2 priority queues per window

# parse input
n, k = (int(i) for i in input().split())
x = [int(xi) for xi in input().split()]

# final output
res = []

for i in range(n-k+1):
    sorted_window = sorted(x[i:i+k])
    med = (k-1)//2
    sum_smaller = sum(sorted_window[:med])
    sum_larger = sum(sorted_window[med+1:])
    cost = sum_larger - sum_smaller
    if k % 2 == 0:
        cost -= sorted_window[med]
    res.append(cost)

print(" ".join(str(i) for i in res))