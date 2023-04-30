#![doc = include_str!("../readme.md")]
#![allow(unused)]

use product::Product;
use rand::Rng;

mod product;

fn main() {
    // Mock 3000 products.
    // Products are in a three-dimensional space.
    // The `products` contains all the products in this space.
    let mut products = Product::generate_products(3000);

    let n = 30;

    let c1 = 2_f64;
    let c2 = 2_f64;
    let w = 0.6;
    let vmax = 5_f64;
    let precision = 0.01;

    let mut rng = rand::thread_rng();
    let range = rng.gen_range(0..products.len());
    let mut g_best = products[range].clone();

    for _ in 0..n {
        let mut current_g_best_idx = usize::MAX;
        for (idx, product) in products.iter_mut().enumerate() {
            // Update fitness.
            let current_product_fitness = product.calculate_fitness();
            if current_product_fitness > product.p_best() {
                product.set_p_best(current_product_fitness);
            }

            // Record the new bestG position.
            if product.p_best() > g_best.p_best() {
                current_g_best_idx = idx;
            }
        }

        // Check boundary: if current calculated bestG sub previous bestG,
        // results in [0, precision], break iteration.
        if current_g_best_idx != usize::MAX {
            if f64::abs(g_best.p_best() - products[current_g_best_idx].p_best()) <= precision {
                println!("early out.");
                break;
            }
            // Update bestG.
            g_best = products[current_g_best_idx].clone();
        }

        // Update all products' position and velocity.
        products.iter_mut().for_each(|product| {
            // Update the position of each product due to last velocity.
            // If the updated position is invalid, nothing would be changed.
            product.update_position();
            product.update_velocity(&g_best, c1, c2, w, vmax);
        })
    }

    println!("The best product: {}", g_best);
    println!("Step unit: 1")
}
