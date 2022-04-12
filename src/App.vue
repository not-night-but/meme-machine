<template>
  <div>Hello, I'm Meme machine!!</div>
  <br />
  <h1>Data</h1>
  <ul>
    <li>{{ resourcePath }}</li>
  </ul>
</template>

<script lang="ts">
import { Vue, Options } from "vue-class-component";
import { invoke } from "@tauri-apps/api/tauri";
import { resourceDir } from "@tauri-apps/api/path";
require("halfmoon/css/halfmoon-variables.min.css");
// eslint-disable-next-line @typescript-eslint/no-var-requires
const halfmoon = require("halfmoon");

class Input {
  text_input: string[] = [];
  name = "";
  constructor(value: Partial<Input>) {
    Object.assign(this, value);
  }
}

@Options({
  data() {
    return {
      resourcePath: null,
    };
  },
  async mounted() {
    let input = new Input({
      name: "sword",
      text_input: ["Does this work?", "It Does!!!"],
    });
    halfmoon.onDOMContentLoaded();
    invoke("create_meme", { input: input }).then((result) => {
      console.log(result);
    });
    this.resourcePath = await resourceDir();
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

nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
