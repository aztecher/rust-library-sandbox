Tokio
===

References
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rustの非同期プログラミングをマスターする](https://tech-blog.optim.co.jp/entry/2019/11/08/163000)

## When not to use Tokio

This is very important for deciding to use tokio framework when we select the asyncronous library in Rust.  

From tokio tutorial (「When not to use Tokio」 in Overview section)
```
Although Tokio is useful for many projects that need to do a lot of things simultaneously, there are also some use-cases where Tokio is not a good fit.

* Speeding up CPU-bound computations by running them in parallel on several threads. Tokio is designed for IO-bound applications where each individual task spends most of its time waiting for IO. If the only thing your application does is run computations in parallel, you should be using rayon. That said, it is still possible to "mix & match" if you need to do both.

* Reading a lot of files. Although it seems like Tokio would be useful for projects that simply need to read a lot of files, Tokio provides no advantage here compared to an ordinary threadpool. This is because operating systems generally do not provide asynchronous file APIs.

* Sending a single web request. The place where Tokio gives you an advantage is when you need to do many things at the same time. If you need to use a library intended for asynchronous Rust such as reqwest, but you don't need to do a lot of things at once, you should prefer the blocking version of that library, as it will make your project simpler. Using Tokio will still work, of course, but provides no real advantage over the blocking API. If the library doesn't provide a blocking API, see the chapter on bridging with sync code.
```
