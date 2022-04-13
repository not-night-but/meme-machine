<template>
  <div>Hello, I'm Meme machine!!</div>

  <h1>These are the currently supported memes:</h1>
  <div v-for="(meme, index) in memes" :key="index">
    <p>{{ meme.name }}</p>
    <img style="max-width: 200px" :src="getFileSrc(meme.image_path)" />
  </div>

  <br />

  <select v-model="selected" @change="onSelect">
    <option disabled value="">Please select one</option>
    <option v-for="(meme, index) in memes" :key="index" :value="index">
      {{ meme.name }}
    </option>
  </select>

  <div v-if="selected != null">
    <p v-for="(text, index) in memes[selected].text_instances" :key="index">
      <input type="text" @change="onChange($event.target.value, index)" />
    </p>
    <button type="button" @click="onSubmit">Submit</button>
  </div>
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
import { invoke, convertFileSrc } from '@tauri-apps/api/tauri';
import { resourceDir } from '@tauri-apps/api/path';
import { Dir, readTextFile } from '@tauri-apps/api/fs';
import { Input, MemeRecord } from './classes';
require('halfmoon/css/halfmoon-variables.min.css');
// eslint-disable-next-line @typescript-eslint/no-var-requires
const halfmoon = require('halfmoon');

@Options({
  data() {
    return {
      resourcePath: '',
      input: new Input() as Input,
      memes: [] as MemeRecord[],
      selected: null,
    };
  },
  async mounted() {
    halfmoon.onDOMContentLoaded();
    this.memes = JSON.parse(
      await readTextFile('assets/memes.json', {
        dir: Dir.Resource,
      })
    ) as MemeRecord[];
    this.resourcePath = await resourceDir();
  },
  methods: {
    onSelect() {
      this.input.name = this.memes[this.selected].name;
      this.input.text_input = new Array<string>(
        this.memes[this.selected].text_instances.length
      );
    },
    onChange(text: string, index: number) {
      this.input.text_input[index] = text;
    },
    onSubmit() {
      invoke('create_meme', {
        input: this.input,
      }).then((result) => {
        console.log(result);
      });
    },
    getFileSrc(filePath: string) {
      if (this.resourcePath == '') {
        return '';
      }

      const path = `${this.resourcePath}assets${filePath}`;

      console.log(path);
      return convertFileSrc(path.replace('\\\\?\\', ''));
    },
  },
})
export default class App extends Vue {}
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

// nav {
//   padding: 30px;

//   a {
//     font-weight: bold;
//     color: #2c3e50;

//     &.router-link-exact-active {
//       color: #42b983;
//     }
//   }
// }
</style>
