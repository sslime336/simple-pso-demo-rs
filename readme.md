# simple-pso-demo-rs

简体中文 | [English](readme-en.md)

粒子群算法的简单应用模拟:

基于粒子群算法计算最有价值货物，基于 Rust 实现

*按照课程要求，部分代码由 ChatGPT 生成，并进行了修改*

## 解决的问题

模拟了从同一类型不同品牌货物中选取相对最具有价值货物的情况。使用粒子群算法解决。

本项目中的货物简单定义为：

```rust
pub struct Product {
    /// 自适度。
    p_best: f64,

    /// x.0 ~ x.3:
    /// - 进价
    /// - 售价
    /// - 市场需求
    x: (f64, f64, f64),

    /// 速度向量。
    v: (i32, i32, i32),

    w1: f64,
    w2: f64,
}
```

单个粒子（货物）的最适度（价值）通过如下公式计算：

```text
((进价 - 售价) * w1) * 市场需求 * w2
```

所有的粒子（货物）位于一个二维的空间中，通过粒子群算法在指定数量的迭代中，计算出当前空间中相对
最具有价值的货物。

所有粒子步长的基本单位为 1，当出现售价低于进价的情况时，粒子只更新速度，不进行位移。
