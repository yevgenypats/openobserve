<!-- Copyright 2023 Zinc Labs Inc.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
-->

<!-- eslint-disable vue/v-on-event-hyphenation -->
<!-- eslint-disable vue/attribute-hyphenation -->
<template>
  <q-page class="q-pa-none" :key="store.state.selectedOrganization.identifier">
    <!-- searchBar at top -->
    <div
      class="q-table__top"
      style="
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
      "
    >
      <div class="q-table__title">{{ t("dashboard.header") }}</div>
      <q-input
        v-model="filterQuery"
        filled
        dense
        class="q-ml-auto"
        :placeholder="t('dashboard.search')"
      >
        <template #prepend>
          <q-icon name="search" />
        </template>
      </q-input>
      <q-btn
        class="q-ml-md text-bold"
        padding="sm lg"
        outline
        no-caps
        :label="t(`dashboard.import`)"
        @click="importDashboard"
      />
      <!-- add dashboard button -->
      <q-btn
        class="q-ml-md text-bold no-border"
        padding="sm lg"
        color="secondary"
        no-caps
        data-test="dashboard-add"
        :label="t(`dashboard.add`)"
        @click="addDashboard"
      />
    </div>
    <q-splitter
      v-model="splitterModel"
      unit="px"
      :limits="[200, 500]"
      style="height: calc(100vh - 122px)"
    >
      <template v-slot:before>
        <div class="text-bold q-px-md q-pt-sm">
          {{ t("dashboard.folderLabel") }}
        </div>
        <div class="dashboards-tabs">
          <q-tabs
            indicator-color="transparent"
            inline-label
            vertical
            v-model="activeFolderId"
          >
            <q-tab
              v-for="(tab, index) in store.state.organizationData.folders"
              :key="tab.folderId"
              :name="tab.folderId"
              content-class="tab_content full-width"
            >
              <div class="full-width row justify-between no-wrap">
                <span
                  style="
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                  "
                  :title="tab.name"
                  >{{ tab.name }}</span
                >
                <div>
                  <q-icon
                    v-if="index"
                    :name="outlinedEdit"
                    class="q-ml-sm"
                    @click.stop="editFolder(tab.folderId)"
                    style="cursor: pointer; justify-self: end"
                  />
                  <q-icon
                    v-if="index"
                    :name="outlinedDelete"
                    class="q-ml-sm"
                    @click.stop="showDeleteFolderDialogFn(tab.folderId)"
                    style="cursor: pointer; justify-self: end"
                  />
                </div>
              </div>
            </q-tab>
          </q-tabs>
          <div
            class="row justify-center full-width q-px-xs q-pb-xs"
            style="position: sticky; bottom: 0px"
          >
            <q-btn
              class="text-bold no-border full-width"
              padding="sm lg"
              color="secondary"
              no-caps
              :label="t('dashboard.newFolderBtnLabel')"
              @click.stop="addFolder"
            />
          </div>
        </div>
      </template>
      <template v-slot:after>
        <!-- add dashboard table -->
        <q-table
          ref="qTable"
          :rows="dashboards"
          :columns="columns"
          row-key="id"
          :pagination="pagination"
          :filter="filterQuery"
          :filter-method="filterData"
          :loading="loading"
          @row-click="onRowClick"
          data-test="dashboard-table"
        >
          <!-- if data not available show nodata component -->
          <template #no-data>
            <NoData />
          </template>
          <!-- add delete icon in actions column -->
          <template #body-cell-actions="props">
            <q-td :props="props">
              <q-btn
                v-if="props.row.actions == 'true'"
                :icon="outlinedDriveFileMove"
                :title="t('dashboard.move_to_another_folder')"
                class="q-ml-xs"
                padding="sm"
                unelevated
                size="sm"
                round
                flat
                @click.stop="showMoveDashboardPanel(props.row.id)"
              ></q-btn>
              <q-btn
                v-if="props.row.actions == 'true'"
                icon="content_copy"
                :title="t('dashboard.duplicate')"
                class="q-ml-xs"
                padding="sm"
                unelevated
                size="sm"
                round
                flat
                @click.stop="duplicateDashboard(props.row.id)"
              ></q-btn>
              <q-btn
                v-if="props.row.actions == 'true'"
                :icon="outlinedDelete"
                :title="t('dashboard.delete')"
                class="q-ml-xs"
                padding="sm"
                unelevated
                size="sm"
                round
                data-test="dashboard-delete"
                flat
                @click.stop="showDeleteDialogFn(props)"
              ></q-btn>
            </q-td>
          </template>
          <!-- searchBar at top -->
          <template #top="scope">
            <!-- table pagination -->
            <QTablePagination
              :scope="scope"
              :pageTitle="t('dashboard.header')"
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
              :maxRecordToReturn="maxRecordToReturn"
              position="bottom"
              @update:changeRecordPerPage="changePagination"
              @update:maxRecordToReturn="changeMaxRecordToReturn"
            />
          </template>
        </q-table>

        <!-- add dashboard -->
        <q-dialog
          v-model="showAddDashboardDialog"
          position="right"
          full-height
          maximized
        >
          <AddDashboard
            @updated="updateDashboardList"
            :activeFolderId="activeFolderId"
          />
        </q-dialog>

        <!-- add/edit folder -->
        <q-dialog
          v-model="showAddFolderDialog"
          position="right"
          full-height
          maximized
        >
          <AddFolder
            @update:modelValue="updateFolderList"
            :edit-mode="isFolderEditMode"
            :folder-id="selectedFolderToEdit ?? 'default'"
          />
        </q-dialog>

        <!-- move dashboard to another folder -->
        <q-dialog
          v-model="showMoveDashboardDialog"
          position="right"
          full-height
          maximized
        >
          <MoveDashboardToAnotherFolder
            @updated="handleDashboardMoved"
            :dashboard-id="selectedDashboardIdToMove"
            :activeFolderId="activeFolderId"
          />
        </q-dialog>

        <!-- delete dashboard dialog -->
        <ConfirmDialog
          title="Delete dashboard"
          data-test="dashboard-confirm-dialog"
          message="Are you sure you want to delete the dashboard?"
          @update:ok="deleteDashboard"
          @update:cancel="confirmDeleteDialog = false"
          v-model="confirmDeleteDialog"
        />

        <!-- delete folder dialog -->
        <ConfirmDialog
          title="Delete Folder"
          message="Are you sure you want to delete this Folder?"
          @update:ok="deleteFolder"
          @update:cancel="confirmDeleteFolderDialog = false"
          v-model="confirmDeleteFolderDialog"
        />
      </template>
    </q-splitter>
  </q-page>
</template>

<script lang="ts">
// @ts-nocheck
import {
  computed,
  defineComponent,
  onActivated,
  onMounted,
  ref,
  watch,
} from "vue";
import { useStore } from "vuex";
import { useQuasar, date } from "quasar";
import { useI18n } from "vue-i18n";

import dashboardService from "../../services/dashboards";
import AddDashboard from "../../components/dashboards/AddDashboard.vue";
import QTablePagination from "../../components/shared/grid/Pagination.vue";
import NoData from "../../components/shared/grid/NoData.vue";
import { useRoute, useRouter } from "vue-router";
import { toRaw } from "vue";
import { getImageURL, verifyOrganizationStatus } from "../../utils/zincutils";
import ConfirmDialog from "../../components/ConfirmDialog.vue";
import {
  deleteDashboardById,
  deleteFolderById,
  getAllDashboards,
  getAllDashboardsByFolderId,
  getDashboard,
  getFoldersList,
} from "../../utils/commons.ts";
import {
  outlinedDelete,
  outlinedDriveFileMove,
  outlinedEdit,
} from "@quasar/extras/material-icons-outlined";
import AddFolder from "../../components/dashboards/AddFolder.vue";
import MoveDashboardToAnotherFolder from "@/components/dashboards/MoveDashboardToAnotherFolder.vue";

export default defineComponent({
  name: "Dashboards",
  components: {
    AddDashboard,
    QTablePagination,
    NoData,
    ConfirmDialog,
    AddFolder,
    MoveDashboardToAnotherFolder,
  },
  setup() {
    const store = useStore();
    const { t } = useI18n();
    const $q = useQuasar();
    const dashboard = ref({});
    const showAddDashboardDialog = ref(false);
    const showAddFolderDialog = ref(false);
    const qTable: any = ref(null);
    const router = useRouter();
    const route = useRoute();
    const orgData: any = ref(store.state.selectedOrganization);
    const confirmDeleteDialog = ref<boolean>(false);
    const selectedDelete = ref(null);
    const splitterModel = ref(200);
    const activeFolderId = ref(null);
    const isFolderEditMode = ref(false);
    const selectedFolderDelete = ref(null);
    const selectedFolderToEdit = ref(null);
    const confirmDeleteFolderDialog = ref<boolean>(false);
    const selectedDashboardIdToMove = ref(null);
    const showMoveDashboardDialog = ref(false);

    const columns = ref<QTableProps["columns"]>([
      {
        name: "#",
        label: "#",
        field: "#",
        align: "left",
      },
      {
        name: "name",
        field: "name",
        label: t("dashboard.name"),
        align: "left",
        sortable: true,
      },
      {
        name: "identifier",
        field: "identifier",
        label: t("dashboard.identifier"),
        align: "left",
        sortable: true,
      },
      {
        name: "description",
        field: "description",
        label: t("dashboard.description"),
        align: "left",
        sortable: true,
      },
      {
        name: "owner",
        field: "owner",
        label: t("dashboard.owner"),
        align: "left",
        sortable: true,
      },
      {
        name: "created",
        field: "created",
        label: t("dashboard.created"),
        align: "left",
        sortable: true,
      },
      {
        name: "actions",
        field: "actions",
        label: t("dashboard.actions"),
        align: "center",
      },
    ]);
    const perPageOptions = [
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

    onMounted(async () => {
      //get folders list
      await getFoldersList(store);

      //initial activeFolderId will be null
      //if route has query and we have a folder in folder list then set activeFolderId to that folder
      // else default as a folder

      activeFolderId.value = null;
      if (
        route.query.folder &&
        store.state.organizationData.folders.find(
          (it: any) => it.folderId === route.query.folder
        )
      ) {
        activeFolderId.value = route.query.folder;
      } else {
        activeFolderId.value = "default";
      }
    });

    watch(
      activeFolderId,
      async () => {
        const dismiss = $q.notify({
          spinner: true,
          message: "Please wait while loading dashboards...",
        });
        await getAllDashboardsByFolderId(store, activeFolderId.value);
        dismiss();
        router.push({
          path: "/dashboards",
          query: {
            org_identifier: store.state.selectedOrganization.identifier,
            folder: activeFolderId.value,
          },
        });
      },
      { deep: true }
    );

    const changePagination = (val: { label: string; value: any }) => {
      selectedPerPage.value = val.value;
      pagination.value.rowsPerPage = val.value;
      qTable.value.setPagination(pagination.value);
    };
    const changeMaxRecordToReturn = async (val: any) => {
      maxRecordToReturn.value = val;
      await getDashboards();
    };
    const addDashboard = () => {
      showAddDashboardDialog.value = true;
    };
    const addFolder = () => {
      isFolderEditMode.value = false;
      showAddFolderDialog.value = true;
    };
    const importDashboard = () => {
      router.push({
        path: "/dashboards/import",
        query: {
          org_identifier: store.state.selectedOrganization.identifier,
          folder: activeFolderId.value || "default",
        },
      });
    };

    const duplicateDashboard = async (dashboardId: any) => {
      const dismiss = $q.notify({
        spinner: true,
        message: "Please wait...",
        timeout: 2000,
      });

      try {
        // Get the dashboard
        const dashboard = await getDashboard(
          store,
          dashboardId,
          activeFolderId.value ?? "default"
        );

        // Duplicate the dashboard
        const data = JSON.parse(JSON.stringify(dashboard));

        //change title owner name and created date
        data.title = `${data.title} - Copy`;
        data.owner = store.state.userInfo.name;
        data.created = new Date().toISOString();

        await dashboardService.create(
          store.state.selectedOrganization.identifier,
          data,
          activeFolderId.value || "default"
        );

        await getDashboards();

        $q.notify({
          type: "positive",
          message: `Dashboard Duplicated Successfully`,
        });
      } catch (err) {
        $q.notify({
          type: "negative",
          message: err?.response?.data["error"]
            ? JSON.stringify(err?.response?.data["error"])
            : "Dashboard duplication failed",
        });
      }

      dismiss();
    };

    const routeToViewD = (row) => {
      return router.push({
        path: "/dashboards/view",
        query: {
          org_identifier: store.state.selectedOrganization.identifier,
          dashboard: row.identifier,
          folder: activeFolderId.value || "default",
        },
      });
    };
    const getDashboards = async () => {
      const dismiss = $q.notify({
        spinner: true,
        message: "Please wait while loading dashboards...",
      });
      await getAllDashboards(store, activeFolderId.value ?? "default");
      dismiss();
    };
    const dashboards = computed(function () {
      const dashboardList = toRaw(
        store.state.organizationData?.allDashboardList[activeFolderId.value] ??
          []
      );
      return dashboardList.map((board: any, index) => {
        return {
          "#": index < 9 ? `0${index + 1}` : index + 1,
          id: board.dashboardId,
          name: board.title,
          identifier: board.dashboardId,
          description: board.description,
          owner: board.owner,
          created: date.formatDate(board.created, "YYYY-MM-DDTHH:mm:ssZ"),
          actions: "true",
        };
      });
    });

    const resultTotal = computed(function () {
      return (
        store.state.organizationData?.allDashboardList[activeFolderId.value]
          ?.length || 0
      );
    });

    const deleteDashboard = async () => {
      if (selectedDelete.value) {
        try {
          //delete dashboard by id and folder id
          await deleteDashboardById(
            store,
            selectedDelete.value.id,
            activeFolderId.value ?? "default"
          );
          $q.notify({
            type: "positive",
            message: "Dashboard deleted successfully.",
            timeout: 5000,
          });
        } catch (err) {
          $q.notify({
            type: "negative",
            message: "Dashboard deletion failed",
            timeout: 2000,
          });
        }
      }
    };

    const showDeleteDialogFn = (props: any) => {
      selectedDelete.value = props.row;
      confirmDeleteDialog.value = true;
    };

    //after adding Folder need to update the Folder list
    const updateFolderList = async (it: any) => {
      showAddFolderDialog.value = false;
      isFolderEditMode.value = false;
    };

    const editFolder = (folderId: any) => {
      selectedFolderToEdit.value = folderId;
      isFolderEditMode.value = true;
      showAddFolderDialog.value = true;
    };

    const showDeleteFolderDialogFn = (folderId: any) => {
      selectedFolderDelete.value = folderId;
      confirmDeleteFolderDialog.value = true;
    };

    const showMoveDashboardPanel = (dashboardId: any) => {
      selectedDashboardIdToMove.value = dashboardId;
      showMoveDashboardDialog.value = true;
    };

    const handleDashboardMoved = async () => {
      showMoveDashboardDialog.value = false;
    };

    const deleteFolder = async () => {
      if (selectedFolderDelete.value) {
        try {
          //delete folder
          await deleteFolderById(store, selectedFolderDelete.value);

          //check activeFolderId to be deleted
          if (activeFolderId.value === selectedFolderDelete.value)
            activeFolderId.value = "default";

          $q.notify({
            type: "positive",
            message: `Folder deleted successfully.`,
            timeout: 2000,
          });
        } catch (err) {
          $q.notify({
            type: "negative",
            message: err.response.data.message || "Folder deletion failed",
            timeout: 2000,
          });
        } finally {
          confirmDeleteFolderDialog.value = false;
        }
      }
    };

    return {
      t,
      qTable,
      store,
      orgData,
      router,
      loading: ref(false),
      dashboards,
      dashboard,
      columns,
      showAddDashboardDialog,
      addDashboard,
      importDashboard,
      pagination,
      resultTotal,
      perPageOptions,
      selectedPerPage,
      changePagination,
      maxRecordToReturn,
      changeMaxRecordToReturn,
      outlinedDelete,
      outlinedEdit,
      outlinedDriveFileMove,
      routeToViewD,
      showDeleteDialogFn,
      confirmDeleteDialog,
      filterQuery: ref(""),
      filterData(rows: string | any[], terms: string) {
        const filtered = [];
        terms = terms.toLowerCase();
        for (let i = 0; i < rows.length; i++) {
          if (rows[i]["name"].toLowerCase().includes(terms)) {
            filtered.push(rows[i]);
          }
        }
        return filtered;
      },
      deleteDashboard,
      duplicateDashboard,
      getDashboards,
      getImageURL,
      verifyOrganizationStatus,
      splitterModel,
      activeFolderId,
      addFolder,
      showAddFolderDialog,
      isFolderEditMode,
      updateFolderList,
      editFolder,
      deleteFolder,
      showDeleteFolderDialogFn,
      confirmDeleteFolderDialog,
      showMoveDashboardPanel,
      selectedFolderToEdit,
      selectedDashboardIdToMove,
      showMoveDashboardDialog,
      handleDashboardMoved,
    };
  },
  methods: {
    //after adding dashboard need to update the dashboard list
    async updateDashboardList(dashboardId: any, folderId: any) {
      this.showAddDashboardDialog = false;

      this.$q.notify({
        type: "positive",
        message: `Dashboard added successfully.`,
      });

      this.$router.push({
        path: "/dashboards/view/",
        query: {
          org_identifier: this.store.state.selectedOrganization.identifier,
          dashboard: dashboardId,
          folder: folderId,
        },
      });
    },
    onRowClick(evt, row) {
      this.routeToViewD(row);
    },
  },
});
</script>

<style lang="scss" scoped>
.q-table {
  &__top {
    border-bottom: 1px solid $border-color;
    justify-content: flex-end;
  }
}

.dashboards-tabs {
  .q-tabs {
    &--vertical {
      margin: 5px;
      .q-tab {
        justify-content: flex-start;
        padding: 0 1rem 0 1.25rem;
        border-radius: 0.5rem;
        text-transform: capitalize;
        &__content.tab_content {
          .q-tab {
            &__icon + &__label {
              padding-left: 0.875rem;
              font-weight: 600;
            }
          }
        }
        &--active {
          background-color: $accent;
          color: black;
        }
      }
    }
  }
}
</style>
