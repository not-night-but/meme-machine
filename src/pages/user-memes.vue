<template>
  <div class="d-flex">
    <div v-for="(meme, index) in user_memes" :key="index">
      <user-meme-tile :path="meme" @selected="onSelect" @download="onDownload"/>
    </div>
    <GenericModal
      :show="showMemeModal"
      @hide="showMemeModal = false"
      :name="'meme-modal'"
    >
      <template v-slot:title>
        <h4>View Meme</h4>
      </template>
      <template v-slot:body>
        <div>
          <img
            class="user-meme rounded"
            :src="selectedPath"
          />
        </div>
      </template>
    </GenericModal>
    <GenericModal
      :show="showDownloadModal"
      @hide="showDownloadModal = false"
      :name="'download-modal'"
    >
      <template v-slot:title>
        <h4>Save Meme</h4>
      </template>
      <template v-slot:body>
        <input type="file"/>
      </template>
    </GenericModal>
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
      showMemeModal: false,
      showDownloadModal: false,
      selectedPath: '',
    };
  },
  methods: {
    onSelect(path: string) {
      this.selectedPath = path;
      this.showMemeModal = true;
    },
    onDownload(path: string) {
      this.selectedPath = path;
      this.showDownloadModal = true;
    }
  },
  computed: {},
})
export default class UserMemes extends Vue {}
</script>

<style lang="scss" scoped>
  div {
    .user-meme {
      margin: auto;
      display: block;
      max-height: 40rem;
      max-width: 40rem;
      height: auto;
      width: auto;
    }
  }
</style>