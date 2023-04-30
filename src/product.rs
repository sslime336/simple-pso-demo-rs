use std::fmt::Display;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Product {
    /// Individual fitness.
    p_best: f64,

    /// There is a three-dimensional space, whose coordinate axis x, y, z's meaning
    /// is below:
    /// 
    /// x.0 (x)- restocking_price
    /// 
    /// x.1 (y)- selling_price
    /// 
    /// x.2 (z)- market_demand
    /// 
    /// Score value is 1.
    x: (f64, f64, f64),

    /// Velocity vector.
    v: (i32, i32, i32),

    w1: f64,
    w2: f64,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Product {{\n\trestocking_price: {},\n\tselling_price: {},\n\tmarket_demand: {},\n}}\nvalue: {}",
            self.x.0, self.x.1, self.x.2, self.calculate_fitness()
        ))
    }
}

impl Product {
    pub const fn new(x: (f64, f64, f64), v: (i32, i32, i32)) -> Self {
        Self {
            p_best: f64::MIN,
            x,
            v,
            w1: 1.2,
            w2: 0.4,
        }
    }

    #[inline]
    pub fn restocking_price(&self) -> f64 {
        self.x.0
    }

    #[inline]
    pub fn selling_price(&self) -> f64 {
        self.x.1
    }

    #[inline]
    pub fn market_demand(&self) -> f64 {
        self.x.2
    }

    #[inline]
    pub fn p_best(&self) -> f64 {
        self.p_best
    }

    pub fn set_p_best(&mut self, new_bestp: f64) {
        self.p_best = new_bestp
    }

    pub fn calculate_fitness(&self) -> f64 {
        ((self.selling_price() - self.restocking_price()) * self.w1)
            * self.w2
            * self.market_demand() as f64
    }

    pub fn update_velocity(&mut self, g_best: &Product, c1: f64, c2: f64, w: f64, vmax: f64) {
        let mut rng = rand::thread_rng();
        let r1: f64 = rng.gen_range(0_f64..1_f64);
        let r2: f64 = rng.gen_range(0_f64..1_f64);
        let v1 = w * self.v.0 as f64
            + c1 * r1 * (self.p_best - self.calculate_fitness())
            + c2 * r2 * (g_best.p_best - self.calculate_fitness());
        let v2 = w * self.v.1 as f64
            + c1 * r1 * (self.x.1 - self.selling_price())
            + c2 * r2 * (g_best.x.1 - self.selling_price());
        let v3 = w * self.v.2 as f64
            + c1 * r1 * (self.x.2 - self.market_demand())
            + c2 * r2 * (g_best.x.2 - self.market_demand());
        self.v = (
            v1.min(vmax).max(-vmax) as i32,
            v2.min(vmax).max(-vmax) as i32,
            v3.min(vmax).max(-vmax) as i32,
        );
    }

    pub fn update_position(&mut self) {
        let v1 = self.v.0 as f64;
        let v2 = self.v.1 as f64;
        let v3 = self.v.2 as f64;
        let (p1, p2, p3) = &mut self.x;
        if *p1 + v1 > 0.0 {
            *p1 += v1;
        }
        if *p2 + v2 > *p1 && *p3 + v3 > 0.0 {
            *p2 += v2;
            *p3 += v3;
        }
    }

    fn random_v() -> i32 {
        if rand::thread_rng().gen_range(-1..1) > 0 {
            1
        } else {
            -1
        }
    }

    /// Mock the given number of [`Product`]s.
    pub fn generate_products(n: i32) -> Vec<Product> {
        let mut products = vec![];

        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let restocking_price = rng.gen_range(20.0..60.0);
            let selling_price = rng.gen_range(restocking_price..100.0);
            let shelf_life = rng.gen_range(30.0..90.0);
            let market_demand = rng.gen_range(1.0..10.0);
            let product = Product::new(
                (restocking_price, selling_price, market_demand),
                (Self::random_v(), Self::random_v(), Self::random_v()),
            );
            products.push(product);
        }

        products
    }

    #[deprecated]
    pub fn generate_products_matrix(rows: usize, cols: usize) -> Vec<Vec<Product>> {
        let mut products_matrix = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rows {
            let mut product_row = Vec::new();
            for _ in 0..cols {
                let restocking_price = rng.gen_range(20.0..60.0);
                let selling_price = rng.gen_range(restocking_price..100.0);
                let shelf_life = rng.gen_range(30.0..90.0);
                let market_demand = rng.gen_range(1.0..10.0);
                let product = Product::new(
                    (restocking_price, selling_price, market_demand),
                    (Self::random_v(), Self::random_v(), Self::random_v()),
                );
                product_row.push(product);
            }
            products_matrix.push(product_row);
        }

        products_matrix
    }

    pub fn set_w1(&mut self, w1: f64) {
        self.w1 = w1;
    }

    pub fn set_w2(&mut self, w2: f64) {
        self.w2 = w2;
    }
}
