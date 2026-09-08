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
            energy_production: 0.1,
            initial_data_price: 10.0,
        }
    }

    pub fn new_solar_panel() -> Self {
        Self {
            name: "Solar Panel".to_string(),
            energy_production: 0.5,
            initial_data_price: 50.0,
        }
    }

    pub fn new_wind_turbine() -> Self {
        Self {
            name: "Wind Turbine".to_string(),
            energy_production: 1.0,
            initial_data_price: 100.0,
        }
    }

    pub fn new_hydro_dam() -> Self {
        Self {
            name: "Hydro Dam".to_string(),
            energy_production: 2.0,
            initial_data_price: 200.0,
        }
    }

    pub fn new_nuclear_reactor() -> Self {
        Self {
            name: "Nuclear Reactor".to_string(),
            energy_production: 5.0,
            initial_data_price: 500.0,
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
            compute_power: 5.0,
            energy_consumption: 0.5,
            initial_data_price: 50.0,
        }
    }

    pub fn new_desktop_pc() -> Self {
        Self {
            name: "Desktop PC".to_string(),
            compute_power: 10.0,
            energy_consumption: 1.0,
            initial_data_price: 100.0,
        }
    }

    pub fn new_server_rack() -> Self {
        Self {
            name: "Server Rack".to_string(),
            compute_power: 20.0,
            energy_consumption: 2.0,
            initial_data_price: 200.0,
        }
    }

    pub fn new_supercomputer() -> Self {
        Self {
            name: "Supercomputer".to_string(),
            compute_power: 50.0,
            energy_consumption: 5.0,
            initial_data_price: 500.0,
        }
    }

    pub fn new_quantum_computer() -> Self {
        Self {
            name: "Quantum Computer".to_string(),
            compute_power: 100.0,
            energy_consumption: 10.0,
            initial_data_price: 1000.0,
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
        if self.data < new_gen.initial_data_price {
            return false; // Not enough data
        }
        
        self.data -= new_gen.initial_data_price;
        self.energy_capacity += new_gen.energy_production;
        self.electricity_generators.push(new_gen);
        true
    }

    pub fn add_compute_unit(&mut self, new_unit: ComputeUnit) -> bool {
        if self.energy_used + new_unit.energy_consumption > self.energy_capacity {
            return false; // Not enough energy capacity to add this unit
        }

        if self.data < new_unit.initial_data_price {
            return false; // Not enough data to add this unit
        }

        self.data -= new_unit.initial_data_price;
        self.compute += new_unit.compute_power;
        self.energy_used += new_unit.energy_consumption;
        self.compute_units.push(new_unit);
        true
    }

    pub fn tick(&mut self, delta_seconds: f64) {
        self.data += self.compute * delta_seconds;
    }
}