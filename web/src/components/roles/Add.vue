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
  <q-page class="q-pa-md role-page">
    <div class="row items-center no-wrap">
      <div class="col">
        <div v-if="beingUpdated" class="text-h6">
          {{ t("roles.edit") }}
        </div>
        <div v-else class="text-h6">{{ t("roles.add") }}</div>
      </div>
    </div>

    <q-separator />
    <div>
      <q-form @submit.prevent="onSubmit" ref="formData">
        <q-input
          v-model="formData.role_name"
          :label="t('roles.name') + ' *'"
          color="input-border"
          bg-color="input-bg"
          class="q-py-sm showLabelOnTop"
          stack-label
          outlined
          filled
          dense
          :prefix="roleNamePrefix"
          :hint="t('roles.nameHint')"
        />

        <Resources ref="resourceRef"></Resources>

        <div class="flex justify-center q-mt-lg">
          <q-btn
            v-close-popup
            class="q-mb-md text-bold"
            :label="t('organization.cancel')"
            text-color="light-text"
            padding="sm md"
            no-caps
            @click="router.replace({ name: 'RoleList' })"
          />
          <q-btn
            :label="t('organization.save')"
            class="q-mb-md text-bold no-border q-ml-md"
            color="secondary"
            padding="sm xl"
            type="submit"
            no-caps
          />
        </div>
      </q-form>
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeMount, onMounted } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { useQuasar } from "quasar";
import { useI18n } from "vue-i18n";
import Resources from "@/components/roles/Resource.vue";
import roles from "@/services/roles";

const defaultValue: any = () => {
  return {
    role_name: "",
    resources: [],
  };
};

export default defineComponent({
  name: "AddRole",
  components: {
    Resources,
  },
  methods: {
    onSubmit() {
      const selectedGrants = this.resourceRef.currentGrant;
      this.formData.resources = selectedGrants;
      this.formData.methods = this.resourceRef.resourceMethod;
      roles
        .create(this.store.state.selectedOrganization.identifier, this.formData)
        .then((response) => {
          this.q.notify({
            message: this.t("roles.addSuccess"),
            color: "positive",
            icon: "check",
          });
          this.$router.push({
            name: "RoleList",
            query: {
              org_identifier: this.store.state.selectedOrganization.identifier,
            },
          });
        });
    },
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const { t } = useI18n();
    const q = useQuasar();
    const formData: any = ref(defaultValue());
    const beingUpdated = ref(false);
    const resourceRef = ref();
    const roleNamePrefix = store.state.selectedOrganization.identifier + "_";

    onBeforeMount(() => {
      if (router.currentRoute.value.query.action == "edit") {
        beingUpdated.value = true;
        if (
          router.currentRoute.value.query.hasOwnProperty("name") &&
          router.currentRoute.value.query.name != ""
        ) {
          formData.value.role_name =
            router.currentRoute.value.query.name?.replace(
              store.state.selectedOrganization.identifier + "_",
              ""
            );
        }
      }
    });

    return {
      store,
      router,
      t,
      q,
      formData,
      beingUpdated,
      resourceRef,
      roleNamePrefix,
    };
  },
});
</script>

<style lang="scss">
.role-page {
  .q-field__prefix {
    padding: 0px !important;
  }

  .q-field__bottom {
    padding: 4px 0px 0px 0px !important;
  }
}
</style>
