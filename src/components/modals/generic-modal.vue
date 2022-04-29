<template>
  <div class="modal w-full h-full" :id="name" tabindex="-1" role="dialog">
    <div class="modal-dialog" role="document">
      <div class="modal-content w-550 h-auto">
        <a
          href="#"
          class="close"
          @click="$emit('hide')"
          role="button"
          aria-label="Close"
        >
          <span aria-hidden="true">&times;</span>
        </a>
        <h5 class="modal-title">
          <slot name="title"> {{ name }} </slot>
        </h5>
        <p>
          <slot name="body">This is where the modal body goes.</slot>
        </p>
        <div class="text-right mt-20">
          <!-- text-right = text-align: right, mt-20 = margin-top: 2rem (20px) -->
          <a href="#" class="btn mr-5" role="button" @click="$emit('hide')"
            >Close</a
          >
          <a
            href="#"
            class="btn btn-primary"
            role="button"
            @click="$emit('hide')"
            >I understand</a
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
// eslint-disable-next-line @typescript-eslint/no-var-requires
const halfmoon = require('halfmoon');

@Options({
  props: {
    name: {
      type: String,
      required: true,
    },
    show: {
      type: Boolean,
      required: true,
    },
  },
  emits: ['hide'],
  data() {
    return {};
  },
  watch: {
    show() {
      halfmoon.toggleModal(this.name);
    },
  },
  methods: {},
  computed: {},
})
export default class GenericModal extends Vue {}
</script>

<style lang="scss" scoped>
</style>