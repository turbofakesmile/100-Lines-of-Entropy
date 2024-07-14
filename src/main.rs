use nalgebra::ComplexField;
use rand::Rng;
use tokio; 

#[derive(Debug, Default)]
struct HotSource<T, D> {
    status: bool,
    temperature: T,
    density: D,
}

trait Heat {
    fn heat_generation(&self) -> i32;
}

impl Heat for HotSource<i32, i32> {
    fn heat_generation(&self) -> i32 {
        let temperature = self.temperature as f64;
        let density = self.density as f64;

        let specific_heat_capacity = 4.18; 
        let mass = density * 1000.0;

        let heat_generated =
            ComplexField::sqrt(specific_heat_capacity * mass * temperature).round() as i32;
        heat_generated
    }
}

trait Entropy {
    fn extract_entropy_level(&self) -> f64;
}

impl Entropy for HotSource<i32, i32> {
    fn extract_entropy_level(&self) -> f64 {
        let temperature = self.temperature as f64;
        let density = self.density as f64;

        let k = 1.380649e-23;
        let volume = 1.0;
        let internal_energy = temperature * density;
        let entropy = k * volume * (internal_energy.ln() + 1.0);

        entropy
    }
}

async fn heat_generate(hot_source: &mut HotSource<i32, i32>) -> i32 {
    let mut rng = rand::thread_rng();

    hot_source.temperature =
        ComplexField::round(20.0 + rng.gen_range(0.0..15.0) * (rng.gen::<f64>().sin() * 10.0))
            as i32;
    hot_source.density =
        ComplexField::round(1000.0 + rng.gen_range(0.0..500.0) * (rng.gen::<f64>().cos() * 5.0))
            as i32;

    println!("Temperature generated: {}", hot_source.temperature);
    println!("Density generated: {}", hot_source.density);

    let heat = hot_source.heat_generation();
    println!("Heat generated: {}", heat);

    let entropy = hot_source.extract_entropy_level();
    println!(
        "
        ███████╗███╗░░██╗████████╗██████╗░░█████╗░██████╗░██╗░░░██╗
        ██╔════╝████╗░██║╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗╚██╗░██╔╝
        █████╗░░██╔██╗██║░░░██║░░░██████╔╝██║░░██║██████╔╝░╚████╔╝░
        ██╔══╝░░██║╚████║░░░██║░░░██╔══██╗██║░░██║██╔═══╝░░░╚██╔╝░░
        ███████╗██║░╚███║░░░██║░░░██║░░██║╚█████╔╝██║░░░░░░░░██║░░░
        ╚══════╝╚═╝░░╚══╝░░░╚═╝░░╚═╝░░╚═╝░╚════╝░╚═╝░░░░░░░░╚═╝░░░: {}",
        entropy
    );

    heat
}

fn activate_hot_source(hot_source: &mut HotSource<i32, i32>) {
    hot_source.status = true;
    hot_source.temperature = 100;
    hot_source.density = 1;
}

#[tokio::main]
async fn main() {
    let mut hot_source = HotSource {
        status: false,
        temperature: 0,
        density: 0,
    };

    activate_hot_source(&mut hot_source);
    println!("The hot source is active!");

    let heat = heat_generate(&mut hot_source).await;
    println!("Total Heat generated: {}", heat);
    println!("The hot source is now inactive!");
    hot_source.status = false;
}
