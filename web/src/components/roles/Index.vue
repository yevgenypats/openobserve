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
  <q-page class="q-pa-none" style="min-height: inherit">
    <q-table
      ref="qTable"
      :rows="tableData"
      :columns="tableColumns"
      row-key="id"
      :pagination="pagination"
      :filter="filterQuery"
      :filter-method="filterData"
    >
      <template #no-data>
        <NoData></NoData>
      </template>
      <template #body-cell-#="props">
        <q-td :props="props" side>{{ row_count++ }}</q-td>
      </template>
      <template #body-cell-actions="props">
        <q-td :props="props">
          <q-btn
            v-if="props.row.role_type != 'default'"
            icon="edit"
            :title="t('roles.edit')"
            padding="sm"
            unelevated
            size="sm"
            round
            flat
            @click="editRole(props)"
          ></q-btn>
        </q-td>
      </template>
      <template #top="scope">
        <div
          class="q-table__title full-width q-mb-md"
          data-test="user-title-text"
        >
          {{ t("roles.header") }}

          <div class="items-end float-right">
            <q-btn
              class="q-ml-md q-mb-xs text-bold no-border"
              padding="sm lg"
              color="secondary"
              no-caps
              :label="t(`roles.add`)"
              @click="addRole"
            ></q-btn>
          </div>
        </div>
        <div class="full-width row q-mb-xs items-start">
          <q-input
            v-model="filterQuery"
            filled
            dense
            class="col-6 q-pr-sm"
            :placeholder="t('roles.search')"
          >
            <template #prepend>
              <q-icon name="search" />
            </template>
          </q-input>
        </div>
        <QTablePagination
          :scope="scope"
          :pageTitle="t('roles.header')"
          :resultTotal="resultTotal"
          :perPageOptions="perPageOptions"
          position="top"
          @update:changeRecordPerPage="changePagination"
        />
      </template>

      <template #bottom="scope">
        <QTablePagination
          :scope="scope"
          :resultTotal="resultTotal"
          :perPageOptions="perPageOptions"
          position="bottom"
          @update:changeRecordPerPage="changePagination"
        />
      </template>
    </q-table>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeMount } from "vue";
import { useStore } from "vuex";
import { useRouter } from "vue-router";
import { useQuasar, type QTableProps, date } from "quasar";
import { useI18n } from "vue-i18n";
import config from "@/aws-exports";
import QTablePagination from "@/components/shared/grid/Pagination.vue";
import roles from "@/services/roles";

export default defineComponent({
  name: "RolePage",
  data() {
    return {};
  },
  methods: {
    addRole() {
      this.router.push({
        name: "AddRolePage",
        query: {
          org_identifier: this.store.state.selectedOrganization.identifier,
        },
      });
    },
    editRole(props: any) {
      this.router.push({
        name: "AddRolePage",
        query: {
          name: props.row.name,
          action: "edit",
          org_identifier: this.store.state.selectedOrganization.identifier,
        },
      });
    },
  },
  components: {
    QTablePagination,
  },
  setup() {
    const store = useStore();
    const router = useRouter();
    const { t } = useI18n();
    const $q = useQuasar();
    const tableData: any = ref([]);
    const resultTotal = ref<number>(0);
    const showUpdateUserDialog: any = ref(false);
    const showAddUserDialog: any = ref(false);
    const confirmDelete = ref<boolean>(false);
    const selectedUser: any = ref({});
    const orgData: any = ref(store.state.selectedOrganization);
    const isUpdated: any = ref(false);
    const qTable: any = ref(null);

    const perPageOptions: any = [
      { label: "5", value: 5 },
      { label: "10", value: 10 },
      { label: "20", value: 20 },
      { label: "50", value: 50 },
      { label: "100", value: 100 },
      { label: "All", value: 0 },
    ];
    const maxRecordToReturn = ref<number>(100);
    const selectedPerPage = ref<number>(20);
    const pagination: any = ref({
      rowsPerPage: 20,
    });

    const filterQuery = ref("");
    const filterData = (rows: any, terms: any) => {
      var filtered = [];
      terms = terms.toLowerCase();
      for (var i = 0; i < rows.length; i++) {
        if (
          rows[i]["first_name"]?.toLowerCase().includes(terms) ||
          rows[i]["last_name"]?.toLowerCase().includes(terms) ||
          rows[i]["email"]?.toLowerCase().includes(terms) ||
          rows[i]["role"].toLowerCase().includes(terms)
        ) {
          filtered.push(rows[i]);
        }
      }
      return filtered;
    };

    const changePagination = (val: { label: string; value: any }) => {
      selectedPerPage.value = val.value;
      pagination.value.rowsPerPage = val.value;
      qTable.value.setPagination(pagination.value);
    };

    const tableColumns: any = ref<QTableProps["columns"]>([
      {
        name: "#",
        label: "#",
        field: "#",
        align: "left",
      },
      {
        name: "name",
        field: "name",
        label: t("roles.name"),
        align: "left",
        sortable: true,
      },
      {
        name: "actions",
        field: "actions",
        label: t("roles.actions"),
        align: "left",
        sortable: true,
      },
    ]);

    onBeforeMount(() => {
      if (store.state.selectedOrganization) {
        roles
          .get(store.state.selectedOrganization.identifier)
          .then((response) => {
            tableData.value = response.data.data;
            resultTotal.value = response.data.total;
          });
      }
    });

    return {
      tableData,
      tableColumns,
      pagination,
      filterQuery,
      filterData,
      resultTotal,
      perPageOptions,
      maxRecordToReturn,
      selectedPerPage,
      showUpdateUserDialog,
      showAddUserDialog,
      confirmDelete,
      selectedUser,
      orgData,
      isUpdated,
      qTable,
      $q,
      t,
      store,
      router,
      changePagination,
      row_count: 1,
    };
  },
});
</script>
