export class InputManager {
  leftPressed = false;
  rightPressed = false;
  private onChange: ((left: boolean, right: boolean) => void) | null = null;

  constructor() {
    window.addEventListener("keydown", (e) => this.handleKey(e, true));
    window.addEventListener("keyup", (e) => this.handleKey(e, false));
  }

  setOnChange(cb: (left: boolean, right: boolean) => void) {
    this.onChange = cb;
  }

  private handleKey(e: KeyboardEvent, down: boolean) {
    let changed = false;
    if (e.key === "a" || e.key === "A" || e.key === "ArrowLeft") {
      if (this.leftPressed !== down) {
        this.leftPressed = down;
        changed = true;
      }
    }
    if (e.key === "d" || e.key === "D" || e.key === "ArrowRight") {
      if (this.rightPressed !== down) {
        this.rightPressed = down;
        changed = true;
      }
    }
    if (changed && this.onChange) {
      this.onChange(this.leftPressed, this.rightPressed);
    }
  }
}
