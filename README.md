# rs-web-light
A continuation of [web-light](https://github.com/rushton/web-light), but in rust. The rust program communicates LED colors serially to an arduino via USB.

# Wire protocol
```
<led_address:8-byte-int><red:8-byte-int><green:8-byte-int><blue:8-byte-int><intensity:8-byte-int>
```
