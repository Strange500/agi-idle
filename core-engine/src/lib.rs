use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameState {
    pub data: f64, 
    pub compute: f64,
    pub energy_capacity: f64,
    pub energy_used: f64,
    compute_units: Vec<ComputeUnit>,
    electricity_generators: Vec<ElectricityGenerator>,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ElectricityGenerator {
    name: String,
    pub energy_production: f64,
    pub initial_data_price: f64,
}

#[wasm_bindgen]
impl ElectricityGenerator {
    pub fn new_potato_battery() -> Self {
        Self {
            name: "Potato Battery".to_string(),
            energy_production: 0.5, // 5x Pocket Calculators
            initial_data_price: 25.0,
        }
    }

    pub fn new_solar_panel() -> Self {
        Self {
            name: "Solar Panel".to_string(),
            energy_production: 5.0, // 10x Smartphones
            initial_data_price: 150.0,
        }
    }

    pub fn new_wind_turbine() -> Self {
        Self {
            name: "Wind Turbine".to_string(),
            energy_production: 20.0,
            initial_data_price: 1000.0,
        }
    }

    pub fn new_hydro_dam() -> Self {
        Self {
            name: "Hydro Dam".to_string(),
            energy_production: 100.0,
            initial_data_price: 8000.0,
        }
    }

    pub fn new_nuclear_reactor() -> Self {
        Self {
            name: "Nuclear Reactor".to_string(),
            energy_production: 1000.0,
            initial_data_price: 50000.0,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ComputeUnit {
    name: String,
    pub compute_power: f64,
    pub energy_consumption: f64,
    pub initial_data_price: f64,
}

#[wasm_bindgen]
impl ComputeUnit {
    pub fn new_pocket_calculator() -> Self {
        Self {
            name: "Pocket Calculator".to_string(),
            compute_power: 1.0,
            energy_consumption: 0.1,
            initial_data_price: 10.0,
        }
    }

    pub fn new_smartphone() -> Self {
        Self {
            name: "Smartphone".to_string(),
            compute_power: 6.0,
            energy_consumption: 0.5,
            initial_data_price: 50.0,
        }
    }

    pub fn new_desktop_pc() -> Self {
        Self {
            name: "Desktop PC".to_string(),
            compute_power: 25.0,
            energy_consumption: 2.0,
            initial_data_price: 200.0,
        }
    }

    pub fn new_server_rack() -> Self {
        Self {
            name: "Server Rack".to_string(),
            compute_power: 150.0,
            energy_consumption: 10.0,
            initial_data_price: 1000.0,
        }
    }

    pub fn new_supercomputer() -> Self {
        Self {
            name: "Supercomputer".to_string(),
            compute_power: 1000.0,
            energy_consumption: 50.0,
            initial_data_price: 8000.0,
        }
    }

    pub fn new_quantum_computer() -> Self {
        Self {
            name: "Quantum Computer".to_string(),
            compute_power: 10000.0,
            energy_consumption: 250.0,
            initial_data_price: 50000.0,
        }
    }
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            data: 0.0,
            compute: 0.0,
            energy_capacity: 0.1, // Start with a basic capacity
            energy_used: 0.0,
            compute_units: vec![],
            electricity_generators: vec![ElectricityGenerator::new_potato_battery()],
        }
    }

    pub fn click(&mut self) {
        self.data += 1.0;
    }

    pub fn add_generator(&mut self, new_gen: ElectricityGenerator) -> bool {
        let count = self.electricity_generators.iter().filter(|g| g.name == new_gen.name).count();
        let price = new_gen.initial_data_price * (1.15_f64).powi(count as i32);

        if self.data < price {
            return false; // Not enough data
        }
        
        self.data -= price;
        self.energy_capacity += new_gen.energy_production;
        self.electricity_generators.push(new_gen);
        true
    }

    pub fn add_compute_unit(&mut self, new_unit: ComputeUnit) -> bool {
        let count = self.compute_units.iter().filter(|u| u.name == new_unit.name).count();
        let price = new_unit.initial_data_price * (1.15_f64).powi(count as i32);

        if self.data < price {
            return false; // Not enough data to add this unit
        }

        self.data -= price;
        self.compute += new_unit.compute_power;
        self.energy_used += new_unit.energy_consumption;
        self.compute_units.push(new_unit);
        true
    }

    pub fn tick(&mut self, delta_seconds: f64) {
        let mut efficiency = 1.0;
        if self.energy_used > self.energy_capacity && self.energy_used > 0.0 {
            efficiency = self.energy_capacity / self.energy_used;
        }
        self.data += self.compute * efficiency * delta_seconds;
    }
}