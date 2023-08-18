<!-- Copyright 2022 Zinc Labs Inc. and Contributors

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http:www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License. 
-->

<template>
  <div>
    <component v-if="loadComponent" :is="componentName" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "vuex";
import { useI18n } from "vue-i18n";
import UsersCloud from "@/enterprise/components/users/User.vue";
import UsersOpenSource from "@/components/users/User.vue";
import config from "@/aws-exports";

export default defineComponent({
  name: "UserPage",
  data() {
    return {
      componentName: "",
      loadComponent: false,
    };
  },
  created() {
    // check condition here and set the componentName accordingly
    if (config.isCloud == "true") {
      this.componentName = "UsersCloud";
    } else {
      this.componentName = "UsersOpenSource";
    }
    this.loadComponent = true;
  },
  components: {
    UsersCloud,
    UsersOpenSource,
  },
  setup() {
    const store = useStore();
    const { t } = useI18n();

    return { store, t };
  },
});
</script>
