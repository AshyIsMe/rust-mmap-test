# Simple quick mmap perf comparisons


M3 Max Macbook Pro:

```
Rust: 

% cargo run --release
*snip*
Memory-mapped vs Vec<i64>:
Generation time - MMap: 1.15s, Vec: 1.07s (Vec is 1.08x faster)
Summation time - MMap: 82.66ms, Vec: 45.20ms (Vec is 1.83x faster)
Total time - MMap: 1.23s, Vec: 1.11s (Vec is 1.11x faster)

j:
% ijconsole j.j
*snip*
Generation time:
0.932229
Summation time:
0.101916
Total time:
1.03415


k: 
% ~/k/k k.k
172
13

(k measurements in ms: 0.172 + 0.013 = 0.185s)
```
