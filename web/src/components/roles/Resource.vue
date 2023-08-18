<!--Copyright 2022 Zinc Labs Inc.and Contributors

 Licensed under the Apache License, Version 2.0(the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

http: www.apache.org / licenses / LICENSE - 2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License. 
-->

<!-- eslint-disable vue/v-on-event-hyphenation -->
<!-- eslint-disable vue/attribute-hyphenation -->
<template>
  <q-page class="q-mt-md">
    <div>
      {{ selectedResources }}
      <!--Provide Link with Icon for Expand All and Collapse all-->
      <div class="row">
        <q-btn @click="expandAll" label="Expand All" class="q-mr-xs" />
        <q-btn @click="collapseAll" label="Collapse All" class="q-mr-xs" />
      </div>
      <div class="q-gutter-md q-mt-xs">
        <div v-for="(endpoints, section) in resourceData" :key="section">
          <q-card>
            <q-card-section
              @click="expanded[section] = !expanded[section]"
              class="cursor-pointer q-card__section--vert"
            >
              <q-card-title class="text-h6">{{ section }}</q-card-title>

              <q-btn
                color="grey"
                round
                flat
                dense
                :icon="
                  expanded[section]
                    ? 'keyboard_arrow_up'
                    : 'keyboard_arrow_down'
                "
                class="float-right"
              />
            </q-card-section>

            <q-separator />

            <q-slide-transition>
              <q-card-section v-show="expanded[section]">
                <q-list class="row">
                  <q-item
                    v-for="item in endpoints"
                    :key="item.name"
                    class="col-3"
                  >
                    <q-item-section>
                      <q-item-label
                        >{{ item.name }} ({{ item.method }})</q-item-label
                      >
                      <q-item-label caption>{{
                        item.description
                      }}</q-item-label>
                    </q-item-section>

                    <q-item-section>
                      <q-toggle v-model="currentGrant[item.name]" />
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-card-section>
            </q-slide-transition>
          </q-card>
        </div>
      </div>
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeMount, onUpdated } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { useQuasar } from "quasar";
import { useI18n } from "vue-i18n";
import resources from "@/services/resources";
import roles from "@/services/roles";

export default defineComponent({
  name: "ResourcePage",
  methods: {
    expandAll() {
      for (const section in this.resourceData) {
        this.expanded[section] = true;
      }
    },
    collapseAll() {
      for (const section in this.resourceData) {
        this.expanded[section] = false;
      }
    },
  },
  setup(props: any) {
    const store = useStore();
    const router = useRouter();
    const { t } = useI18n();
    const $q = useQuasar();
    const resourceData = ref();
    const currentGrant: any = ref({});
    const resourceMethod: any = ref({});
    let selectedResources: any = [];

    onBeforeMount(() => {
      if (router.currentRoute.value.query.action == "edit") {
        let roleName = router.currentRoute.value.query.name || "";
        roles
          .getSelectedResources(
            store.state.selectedOrganization.identifier,
            roleName
          )
          .then((response) => {
            response.data.data.map(function (resource: any) {
              selectedResources[resource.resources] = true;
            });

            getResources("edit");
          });
      } else {
        getResources("add");
      }
    });

    const getResources = (action: string) => {
      resources.getAll().then((response) => {
        resourceData.value = response.data.data;
        for (const section in response.data.data) {
          for (const item of response.data.data[section]) {
            if (action == "add") {
              currentGrant.value[item.name] = item.default_permission;
            } else {
              currentGrant.value[item.name] = selectedResources[item.name]
                ? true
                : false;
            }

            resourceMethod.value[item.name] = item.method;
          }
        }
      });
    };

    const getSelectedGrants = () => {
      return currentGrant;
    };

    return {
      store,
      router,
      t,
      $q,
      resourceData,
      currentGrant,
      resourceMethod,
      getSelectedGrants,
      expanded: ref({}),
      actions: ref({}),
    };
  },
});
</script>

<style scoped>
.q-card__section--vert {
  padding: 5px 10px;
}
.q-item__label {
  text-transform: capitalize;
}
</style>
