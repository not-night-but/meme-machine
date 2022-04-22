<template>
  <div
    class="card p-5 meme-tile"
    :class="{ hover: allowHover }"
    @click="onClick"
  >
    <img class="w-200 h-150 rounded" style="object-fit: cover" :src="path" />
    <div class="content" v-if="showTitle">
      <h4 class="content-title">{{ meme.formatName() }}</h4>
    </div>
  </div>
</template>

<script lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Options, Vue } from 'vue-class-component';
import { MemeRecord } from './classes';

@Options({
  name: 'meme-tile',
  props: {
    meme: {
      type: MemeRecord,
      required: false,
      default: null,
    },
    resourcePath: {
      type: String,
      required: false,
      default: '',
    },
    allowHover: {
      type: Boolean,
      required: false,
      default: false,
    },
    showTitle: {
      type: Boolean,
      required: false,
      default: true,
    },
    fullPath: {
      type: String,
      required: false,
      default: null,
    },
  },
  emits: ['selected'],
  computed: {
    path() {
      if (this.resourcePath == '' && this.fullPath == null) {
        return '';
      }

      const path =
        this.fullPath == null
          ? `${this.resourcePath}assets${this.meme.image_path}`
          : this.fullPath;
      return convertFileSrc(path.replace('\\\\?\\', ''));
    },
  },
  data() {
    return {
      hover: false,
    };
  },
  methods: {
    onClick() {
      this.$emit('selected', this.fullPath == null ? this.meme : this.fullPath);
    },
  },
})
export default class MemeTile extends Vue {}
</script>

<style lang="scss" scoped>
.meme-tile {
  margin: 0;
  margin-left: 0.5rem;
  margin-right: 0.5rem;
  transition: all 100ms ease-in-out;
  &.hover {
    cursor: pointer;
    &:hover {
      background: hsl(214, 12%, 13%);
      margin-top: -0.5rem;
      margin-bottom: 0.5rem;
      box-shadow: 0 0.5rem 1rem 0 rgba(0, 0, 0, 0.3);
    }
  }
}
</style>