<template>
  <div class="page-wrapper with-navbar with-sidebar">
    <div class="navbar">
      <div class="navbar-content">
        <button
          class="btn btn-action menu"
          type="button"
          @click="toggleSidebar"
        >
          <i class="fa-solid fa-bars"></i>
        </button>
        <span class="navbar-brand">Meme Machine</span>
        <span class="navbar-text text-monospace">v{{getPackageVersion()}}</span>
      </div>
    </div>
    <div class="sidebar">
      <!-- <h5 class="sidebar-title">Meme Machine</h5> -->
      <div class="sidebar-menu">
        <router-link to="/" class="sidebar-link" active-class="active"
          >Create Meme</router-link
        >
        <router-link to="/memes" class="sidebar-link" active-class="active"
          >Your Memes</router-link
        >
        <!-- <a href="" class="sidebar-link active"> Create Meme </a> -->
      </div>
    </div>
    <div class="content-wrapper text-light pt-5">
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts">
import { Vue, Options } from 'vue-class-component';
// eslint-disable-next-line @typescript-eslint/no-var-requires
const halfmoon = require('halfmoon');
// eslint-disable-next-line @typescript-eslint/no-var-requires
const pjson = require('../package.json')

@Options({
  methods: {
    toggleSidebar() {
      halfmoon.toggleSidebar();
    },
    getPackageVersion() {
      return pjson.version;
    }
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
</style>
