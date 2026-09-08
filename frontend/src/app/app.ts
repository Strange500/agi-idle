import { Component, OnInit, signal } from '@angular/core';
import { DecimalPipe } from '@angular/common';

// Path pointing to the wasm-pack output
import init, { GameState, ComputeUnit, ElectricityGenerator } from '../../public/wasm/core_engine';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [DecimalPipe],
  templateUrl: './app.html',
})
export class App implements OnInit {

  private game!: GameState;
  
  data = signal(0);
  energyCap = signal(0);
  energyUsed = signal(0);

  async ngOnInit() {
    await init('/wasm/core_engine_bg.wasm');
    this.game = new GameState();
    this.syncState();

    // Start tick loop (10 ticks/second)
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

  onAddGenerator() {
    this.game.add_generator(ElectricityGenerator.new_potato_battery());
    this.syncState();
  }

  onAddComputeUnit() {
    this.game.add_compute_unit(ComputeUnit.new_pocket_calculator());
    this.syncState();
  }
}