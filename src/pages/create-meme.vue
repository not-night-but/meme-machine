<template>
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
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
import MemeTile from '../components/tiles/meme-tile.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { resourceDir } from '@tauri-apps/api/path';
import { Input, MemeRecord } from '../classes';

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
    this.memes = ((await invoke('get_templates')) as MemeRecord[]).map(
      (meme) => {
        return new MemeRecord(meme);
      }
    );
    console.log(this.memes);
    this.resourcePath = await resourceDir();
  },
  methods: {
    onChange(text: string, index: number) {
      this.input.text_input[index] = text;
    },
    onSubmit() {
      invoke('create_meme', {
        input: this.input,
      });
      this.onCancel();
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
  },
})
export default class CreateMeme extends Vue {}
</script>

<style lang="scss">
</style>