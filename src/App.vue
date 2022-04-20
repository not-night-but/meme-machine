<template>
  <div class="page-wrapper with-navbar with-sidebar">
    <div class="navbar">
      <div class="navbar-content">
        <button
          class="btn btn-action menu"
          type="button"
          @click="toggleSidebar"
        >
          <img src="../public/menu.svg" alt="" />
        </button>
        <span class="navbar-brand">Meme Machine</span>
        <span class="navbar-text text-monospace">v0.3.0</span>
      </div>
    </div>
    <div class="sidebar">
      <!-- <h5 class="sidebar-title">Meme Machine</h5> -->
      <div class="sidebar-menu">
        <a href="" class="sidebar-link active"> Create Meme </a>
      </div>
    </div>
    <div class="content-wrapper text-light pt-5">
      <div v-if="selected == null">
        <div class="d-flex justify-content-start">
          <div v-for="(meme, index) in memes" :key="index">
            <meme-tile
              :meme="meme"
              :resource-path="resourcePath"
              :allow-hover="true"
              @selected="meme_OnClick"
            />
          </div>
        </div>
      </div>

      <br />

      <!-- <div class="d-flex justify-content-center">
        <select
          class="form-control w-quarter"
          v-model="selected"
          @change="onSelect"
        >
          <option disabled value="">Please select one</option>
          <option v-for="(meme, index) in memes" :key="index" :value="index">
            {{ meme.formatName() }}
          </option>
        </select>
      </div> -->

      <div v-if="selected != null">
        <div class="d-flex justify-content-center">
          <meme-tile :meme="selected" :resourcePath="resourcePath" />
        </div>
        <p v-for="(text, index) in selected.text_instances" :key="index">
          <input type="text" @change="onChange($event.target.value, index)" />
        </p>
        <div class="d-flex justify-content-center">
          <button class="btn" type="button" @click="onCancel">Cancel</button>
          <button class="btn btn-primary" type="button" @click="onSubmit">
            Submit
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
import { invoke } from '@tauri-apps/api/tauri';
import { resourceDir } from '@tauri-apps/api/path';
import { Dir, readTextFile } from '@tauri-apps/api/fs';
import { Input, MemeRecord } from './classes';
import MemeTile from './meme-tile.vue';
require('halfmoon/css/halfmoon-variables.min.css');
// eslint-disable-next-line @typescript-eslint/no-var-requires
const halfmoon = require('halfmoon');

@Options({
  components: {
    MemeTile,
  },
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
    ).map((x: MemeRecord) => {
      return new MemeRecord(x);
    });
    this.resourcePath = await resourceDir();
  },
  methods: {
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
    onCancel() {
      this.input = new Input();
      this.selected = null;
    },
    meme_OnClick(meme: MemeRecord) {
      this.selected = meme;
      this.input.name = meme.name;
      this.input.text_input = new Array<string>(meme.text_instances.length);
    },
    toggleSidebar() {
      halfmoon.toggleSidebar();
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

.menu {
  padding-top: 0.4rem;
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
