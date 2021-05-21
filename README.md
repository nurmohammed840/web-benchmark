## Tools

Envirenment: `Ubantu-20.04 (WSL)`
Processor: [Intel Celeron N4000](https://ark.intel.com/content/www/us/en/ark/products/128988/intel-celeron-processor-n4000-4m-cache-up-to-2-60-ghz.html)
Benchmark tool ([wrk](https://github.com/wg/wrk)): `wrk --latency -t2 -c10 -d10 <url>`

```txt
Running 10 seconds, 2 threads and 10 connections, (Fri, May 21, 2021)
```

### Hyper (https://github.com/hyperium/hyper) v0.14

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   437.49us  632.37us  22.56ms   98.62%
  Req/Sec    12.56k   726.58    13.59k    92.04%

Latency Distribution
   50%  407.00us
   75%  484.00us
   90%  564.00us
   99%    1.83ms

251253 requests in 10.10s, 21.33MB read

Requests/sec:  24875.88
Transfer/sec:  2.11MB
```

### [Actix-Web](https://github.com/actix/actix-web) v3.3.2

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   456.09us  538.80us  13.30ms   97.49%
  Req/Sec    12.42k   587.38    13.31k    86.50%

Latency Distribution
   50%  386.00us
   75%  400.00us
   90%  420.00us
   99%    3.03ms

247092 requests in 10.00s, 30.40MB read

Requests/sec:  24699.91
Transfer/sec:  3.04MB
```

### [Warp](https://github.com/seanmonstar/warp) v0.3.1

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   436.03us  383.02us  15.10ms   98.55%
  Req/Sec    11.98k   445.32    14.48k    88.06%

Latency Distribution
   50%  430.00us
   75%  511.00us
   90%  592.00us
   99%    1.18ms
  
239521 requests in 10.10s, 29.70MB read

Requests/sec:  23715.24
Transfer/sec:  2.94MB
```

### [Gotham](https://github.com/gotham-rs/gotham) v0.6.0

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   514.18us  469.73us  12.51ms   98.42%
  Req/Sec    10.24k   625.39    11.04k    90.00%

Latency Distribution
   50%  497.00us
   75%  600.00us
   90%  692.00us
   99%    1.63ms
  
203797 requests in 10.00s, 32.26MB read

Requests/sec:  20376.83
Transfer/sec:  3.23MB
```

### [Tiny-Http]() v0.6

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   489.92us    0.99ms  21.06ms   96.61%
  Req/Sec     9.44k     4.21k   16.60k    55.00%

Latency Distribution
   50%  287.00us
   75%  463.00us
   90%  718.00us
   99%    5.26ms

187855 requests in 10.00s, 27.59MB read

Requests/sec:  18776.92
Transfer/sec:  2.76MB
```

### [Nickel]() v0.11.0

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   119.77us  145.01us  10.78ms   99.64%
  Req/Sec    13.70k   545.48    15.67k    80.20%

Latency Distribution
   50%  117.00us
   75%  140.00us
   90%  150.00us
   99%  189.00us

137745 requests in 10.10s, 19.18MB read

Requests/sec:  13637.91
Transfer/sec:  1.90MB
```

### [Tide]() v0.16.0

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency    10.72ms   15.71ms 100.42ms   81.45%
  Req/Sec     5.05k     1.30k    7.18k    68.50%

Latency Distribution
   50%  286.00us
   75%   18.56ms
   90%   38.00ms
   99%   54.18ms

100642 requests in 10.03s, 12.38MB read

Requests/sec:  10032.87
Transfer/sec:  1.23MB
```

### [Rocket](https://github.com/SergioBenitez/Rocket) v0.4.9

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency   462.94us  814.43us  19.04ms   96.73%
  Req/Sec     4.23k     0.92k    6.94k    79.50%

Latency Distribution
   50%  306.00us
   75%  479.00us
   90%  664.00us
   99%    4.09ms

 84397 requests in 10.02s, 11.75MB read
Socket errors: connect 0, read 84396, write 0, timeout 0

Requests/sec:   8422.24
Transfer/sec:   1.17MB
```

# Java-Script
Javascript is single Threaded mean, Only single core being used.
Those test run on a single procress. (not dual)

## [Deno](https://deno.land/) v1.10.2
```
deno 1.10.2 (release, x86_64-unknown-linux-gnu)
v8 9.1.269.27
typescript 4.2.2
```

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency     1.19ms  480.42us  11.51ms   96.01%
  Req/Sec     4.27k   444.91     4.98k    56.50%
  
Latency Distribution
   50%    1.08ms
   75%    1.27ms
   90%    1.38ms
   99%    3.59ms
  
84986 requests in 10.00s, 4.05MB read

Requests/sec:   8498.00
Transfer/sec:   414.94KB
```

### Oak.js (Deno) v7.5.0

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency     1.70ms  413.07us  11.74ms   95.80%
  Req/Sec     2.93k   171.60     3.28k    72.50%
  
Latency Distribution
   50%    1.62ms
   75%    1.73ms
   90%    1.89ms
   99%    3.40ms

58302 requests in 10.01s, 5.12MB read
  
Requests/sec:   5822.67
Transfer/sec:   523.13KB
```

## [Node](https://nodejs.org/) v10.19.0

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency     1.15ms  437.51us  10.59ms   94.10%
  Req/Sec     4.43k   275.69     5.13k    69.00%

Latency Distribution
   50%    1.09ms
   75%    1.14ms
   90%    1.21ms
   99%    2.28ms
  
88268 requests in 10.02s, 9.34MB read

Requests/sec:   8812.16
Transfer/sec:   0.93MB
```

### Koa.js (Node) v2.13.1

```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency     1.50ms  481.28us  14.09ms   94.19%
  Req/Sec     3.38k   217.87     4.24k    76.50%
  
Latency Distribution
   50%    1.42ms
   75%    1.50ms
   90%    1.64ms
   99%    2.94ms

67219 requests in 10.02s, 9.55MB read

Requests/sec:   6706.19
Transfer/sec:   0.95MB
```

### Express.js (Node) v4.17.1
```txt
Thread Stats   Avg      Stdev     Max   +/- Stdev
  Latency     2.19ms  682.71us  12.89ms   90.07%
  Req/Sec     2.31k   197.64     2.71k    67.50%

Latency Distribution
   50%    2.04ms
   75%    2.18ms
   90%    2.56ms
   99%    4.60ms

45985 requests in 10.02s, 9.47MB read

Requests/sec:   4590.44
Transfer/sec:   0.95MB
```