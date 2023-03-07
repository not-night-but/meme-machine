<template>
  <div class="card p-5 meme-tile" @click.stop.prevent="onClick">
    <img
      class="w-200 h-150 rounded"
      style="object-fit: cover"
      :src="assetPath"
    />
    <div class="position-relative download-button">
      <button
        class="btn btn-sm position-absolute bottom-0 right-0 mr-10 mb-5"
        @click.stop.prevent="downloadOnClick"
      >
        <i class="fa-solid fa-download"></i>
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Vue, Options } from 'vue-class-component';

@Options({
  props: {
    path: {
      type: String,
      required: true,
    },
  },
  emits: ['selected'],
  methods: {
    onClick() {
      this.$emit('selected', this.assetPath);
    },
    downloadOnClick() {
      this.$emit('download', this.assetPath);
    }
  },
  computed: {
    assetPath() {
      return convertFileSrc(this.path);
    },
  },
})
export default class UserMemeTile extends Vue {}
</script>

<style lang="scss" scoped>
.meme-tile {
  margin: 0;
  margin-left: 0.5rem;
  margin-right: 0.5rem;
  transition: all 100ms ease-in-out;
  cursor: pointer;

  & > div.download-button {
    display: none;
    & > button {
      background: hsl(214, 12%, 11%);
    }
  }

  &:hover {
    background: hsl(214, 12%, 13%);
    margin-top: -0.5rem;
    margin-bottom: 0.5rem;
    box-shadow: 0 0.5rem 1rem 0 rgba(0, 0, 0, 0.3);

    & > div.download-button {
      display: block;
      & > button:hover {
        background: hsl(214, 12%, 15%);
      }
    }
  }
}
</style>