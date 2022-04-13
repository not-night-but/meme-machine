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

  constructor(value?: Partial<MemeRecord>) {
    Object.assign(this, value);
  }
}