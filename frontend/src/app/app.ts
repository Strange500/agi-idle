import { Component, OnInit, signal, computed } from '@angular/core';
import { DecimalPipe, NgClass, NgFor } from '@angular/common';

// Path pointing to the wasm-pack output
import init, { GameState, ComputeUnit, ElectricityGenerator } from '../../public/wasm/core_engine';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [DecimalPipe, NgClass],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App implements OnInit {
  private game!: GameState;
  
  data = signal(0);
  energyCap = signal(0);
  energyUsed = signal(0);

  // Derived signals for the progress bar
  energyRatio = computed(() => {
    if (this.energyCap() === 0) return 0;
    return Math.min(100, (this.energyUsed() / this.energyCap()) * 100);
  });
  
  energyWarning = computed(() => this.energyRatio() >= 90);

  efficiency = computed(() => {
    const used = this.energyUsed();
    const cap = this.energyCap();
    if (used === 0) return 100;
    if (cap >= used) return 100;
    return (cap / used) * 100;
  });

  // Inventory Tracking
  inventory = signal({
    'Potato Battery': 1,
    'Pocket Calculator': 0,
    'Smartphone': 0,
    'Solar Panel': 0
  });

  // Dynamic Prices
  potatoBatteryPrice = computed(() => 25 * Math.pow(1.15, this.inventory()['Potato Battery']));
  solarPanelPrice = computed(() => 150 * Math.pow(1.15, this.inventory()['Solar Panel']));
  pocketCalculatorPrice = computed(() => 10 * Math.pow(1.15, this.inventory()['Pocket Calculator']));
  smartphonePrice = computed(() => 50 * Math.pow(1.15, this.inventory()['Smartphone']));

  async ngOnInit() {
    await init('wasm/core_engine_bg.wasm');
    this.game = new GameState();
    this.syncState();

    setInterval(() => {
      this.game.tick(0.1);
      this.syncState();
    }, 100);
  }

  onManualClick() {
    this.game.click();
    this.syncState();
  }

  private syncState() {
    this.data.set(this.game.data);
    this.energyCap.set(this.game.energy_capacity);
    this.energyUsed.set(this.game.energy_used);
  }

  buyPotatoBattery() {
    if (this.game.add_generator(ElectricityGenerator.new_potato_battery())) {
      this.inventory.update(inv => ({...inv, 'Potato Battery': inv['Potato Battery'] + 1}));
      this.syncState();
    }
  }

  buySolarPanel() {
    if (this.game.add_generator(ElectricityGenerator.new_solar_panel())) {
      this.inventory.update(inv => ({...inv, 'Solar Panel': inv['Solar Panel'] + 1}));
      this.syncState();
    }
  }

  buyPocketCalculator() {
    if (this.game.add_compute_unit(ComputeUnit.new_pocket_calculator())) {
      this.inventory.update(inv => ({...inv, 'Pocket Calculator': inv['Pocket Calculator'] + 1}));
      this.syncState();
    }
  }

  buySmartphone() {
    if (this.game.add_compute_unit(ComputeUnit.new_smartphone())) {
      this.inventory.update(inv => ({...inv, 'Smartphone': inv['Smartphone'] + 1}));
      this.syncState();
    }
  }
}