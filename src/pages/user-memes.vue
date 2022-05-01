<template>
  <div class="d-flex justify-content-start">
    <div v-for="(meme, index) in user_memes" :key="index">
      <!-- <meme-tile :fullPath="meme" :allow-hover="true" :showTitle="false" /> -->
      <user-meme-tile :path="meme" @selected="onSelect" />
      <GenericModal
        :show="showModal"
        @hide="showModal = false"
        :name="'meme-modal'"
      >
        <template v-slot:title>
          <h4>View Meme</h4>
        </template>
        <template v-slot:body>
          <img
            class="w-auto h-auto rounded"
            style="object-fit: contain"
            :src="selectedPath"
          />
        </template>
      </GenericModal>
    </div>
  </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { Vue, Options } from 'vue-class-component';
import UserMemeTile from '../components/tiles/user-meme-tile.vue';
import GenericModal from '../components/modals/generic-modal.vue';

@Options({
  async mounted() {
    this.user_memes = await invoke('get_user_memes');
  },
  components: { UserMemeTile, GenericModal },
  data() {
    return {
      user_memes: [],
      showModal: false,
      selectedPath: '',
    };
  },
  methods: {
    onSelect(path: string) {
      this.selectedPath = path;
      this.showModal = true;
    },
  },
  computed: {},
})
export default class UserMemes extends Vue {}
</script>

<style lang="scss" scoped>
</style>