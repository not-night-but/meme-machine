<template>
  <div
    class="meme-tile card p-0"
    :class="{ hover: allowHover }"
    @click="onClick"
  >
    <img class="img-fluid h-150 rounded" :src="path" />
    <div class="content p-0 overflow-hidden" style="white-space: nowrap;">
      <h4 class="content-title">{{ meme.formatName() }}</h4>
    </div>
  </div>
</template>

<script lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Options, Vue } from 'vue-class-component';
import { MemeRecord } from '../../classes';

@Options({
  name: 'meme-tile',
  props: {
    meme: {
      type: MemeRecord,
      required: true,
    },
    resourcePath: {
      type: String,
      required: true,
    },
    allowHover: {
      type: Boolean,
      required: false,
      default: false,
    },
  },
  emits: ['selected'],
  computed: {
    path() {
      if (this.resourcePath == '') {
        return '';
      }

      const path = `${this.resourcePath}assets${this.meme.image_path}`;
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
      this.$emit('selected', this.meme);
    },
  },
})
export default class MemeTile extends Vue {}
</script>

<style lang="scss" scoped>
.meme-tile {
  width: 15rem;
  margin-left: 2.5rem;
  margin-right: 2.5rem;
  transition: all 100ms ease-in-out;
  &.hover {
    cursor: pointer;
    &:hover {
      background: hsl(214, 12%, 13%);
      margin-top: -0.25rem;
      margin-bottom: 0.25rem;
      box-shadow: 0 0.5rem 1rem 0 rgba(0, 0, 0, 0.3);
    }
  }
}
</style>