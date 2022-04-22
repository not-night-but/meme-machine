<template>
  <div class="d-flex justify-content-start">
    <div v-for="(meme, index) in user_memes" :key="index">
      <meme-tile :fullPath="meme" :allow-hover="true" :showTitle="false" />
    </div>
  </div>
</template>

<script lang="ts">
import { Dir, readTextFile } from '@tauri-apps/api/fs';
import { Vue, Options } from 'vue-class-component';
import MemeTile from './meme-tile.vue';

@Options({
  async mounted() {
    this.user_memes = JSON.parse(
      await readTextFile('memes/user_memes.json', {
        dir: Dir.Resource,
      })
    );
  },
  components: { MemeTile },
  data() {
    return {
      user_memes: [],
    };
  },
  methods: {},
  computed: {},
})
export default class UserMemes extends Vue {}
</script>

<style lang="scss" scoped>
</style>