#![doc = include_str!("../readme.md")]
#![allow(unused)]

use product::Product;
use rand::Rng;

mod product;

fn main() {
    // Mock
    let mut products_matrix = Product::generate_products_matrix(300, 300);

    let n = 30;

    let c1 = 2_f64;
    let c2 = 2_f64;
    let w = 0.6;
    let vmax = 5_f64;
    let precision = 0.01;

    let mut rng = rand::thread_rng();
    let mut g_best = products_matrix[rng.gen_range(0..products_matrix.len())]
        [rng.gen_range(0..products_matrix[0].len())]
    .clone();

    for _ in 0..n {
        let mut current_g_best_position = (usize::MAX, usize::MAX);
        for (r, products_row) in products_matrix.iter_mut().enumerate() {
            for (c, product) in products_row.iter_mut().enumerate() {
                // Update fitness.
                let current_product_fitness = product.calculate_fitness();
                if current_product_fitness > product.p_best() {
                    product.set_p_best(current_product_fitness);
                }

                // Record the new bestG position.
                if product.p_best() > g_best.p_best() {
                    current_g_best_position = (r, c);
                }
            }
        }

        // Check boundary: if current calculated bestG sub previous bestG,
        // results in [0, precision], break iteration.
        let r = current_g_best_position.0;
        let c = current_g_best_position.1;
        if r != usize::MAX && c != usize::MAX {
            if f64::abs(g_best.p_best() - products_matrix[r][c].p_best()) <= precision {
                println!("early out.");
                break;
            }

            // Update bestG.
            g_best = products_matrix[r][c].clone();
        }

        // Update all products' position and velocity.
        products_matrix.iter_mut().for_each(|products_row| {
            products_row.iter_mut().for_each(|product| {
                product.update_position();
                product.update_velocity(&g_best, c1, c2, w, vmax);
            })
        })
    }

    println!("The best product: {}", g_best);
    println!("Step unit: 1")
}
