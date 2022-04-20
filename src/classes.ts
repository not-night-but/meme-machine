export class Input {
  text_input: string[] = [];
  name = "" as string;
  constructor(value?: Partial<Input>) {
    Object.assign(this, value);
  }
}

export class MemeRecord {
  name = "";
  image_path = "";
  text_instances: [number, number, number][] = [];
  text_color: [number, number, number] = [0, 0, 0];
  text_scale: [number, number] = [1, 1];

  public formatName(): string {
    return this.name.replace('-', ' ').split(' ').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
  }

  constructor(value?: Partial<MemeRecord>) {
    Object.assign(this, value);
  }
}