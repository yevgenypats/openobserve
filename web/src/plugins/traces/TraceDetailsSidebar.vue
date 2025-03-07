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

<template>
  <div
    class="flex justify-start items-center q-px-sm hearder_bg border border-bottom border-top"
    :style="{ height: '30px' }"
  >
    <div :style="{ width: 'calc(100% - 22px)' }" class="q-pb-none ellipsis">
      Span Details
    </div>
    <q-btn
      dense
      icon="close"
      class="align-right no-border q-pa-xs"
      size="xs"
      @click="closeSidebar"
    ></q-btn>
  </div>
  <div class="q-pb-sm q-pt-xs flex">
    <div
      :title="span.operation_name"
      class="q-px-sm q-pb-none ellipsis non-selectable"
      style="font-size: 14px"
    >
      <span class="text-grey-9">Operation Name: </span>{{ span.operation_name }}
    </div>
    <div
      class="q-px-sm ellipsis non-selectable"
      :title="span.service_name"
      style="font-size: 14px"
    >
      <span class="text-grey-9">Service Name: </span> {{ span.service_name }}
    </div>
  </div>
  <q-tabs
    v-model="activeTab"
    dense
    inline-label
    class="text-bold q-mx-sm span_details_tabs"
  >
    <q-tab
      name="tags"
      :label="t('common.tags')"
      style="text-transform: capitalize"
    />
    <q-tab
      name="process"
      :label="t('common.process')"
      style="text-transform: capitalize"
    />
    <q-tab
      name="events"
      :label="t('common.events')"
      style="text-transform: capitalize"
    />
    <q-tab
      name="attributes"
      :label="t('common.attributes')"
      style="text-transform: capitalize"
    />
  </q-tabs>
  <q-separator style="width: 100%" />
  <q-tab-panels v-model="activeTab" class="span_details_tab-panels">
    <q-tab-panel name="tags">
      <table class="q-my-sm">
        <tbody>
          <template v-for="(val, key) in tags" :key="key">
            <tr>
              <td class="q-py-xs q-px-sm text-grey-9">{{ key }}</td>
              <td class="q-py-xs q-px-sm text-primary">
                {{ val }}
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </q-tab-panel>
    <q-tab-panel name="process">
      <table class="q-my-sm">
        <tbody>
          <template v-for="(val, key) in processes" :key="key">
            <tr>
              <td class="q-py-xs q-px-sm text-grey-9">{{ key }}</td>
              <td class="q-py-xs q-px-sm text-primary">
                {{ val }}
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </q-tab-panel>
    <q-tab-panel name="attributes">
      <table class="q-my-sm">
        <tbody>
          <template v-for="(val, key) in spanDetails.attrs" :key="key">
            <tr>
              <td class="q-py-xs q-px-sm text-grey-9">{{ key }}</td>
              <td class="q-py-xs q-px-sm text-primary">
                {{ val }}
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </q-tab-panel>
    <q-tab-panel name="events">
      <q-virtual-scroll
        type="table"
        ref="searchTableRef"
        style="max-height: 100%"
        :items="spanDetails.events"
      >
        <template v-slot:before>
          <thead class="thead-sticky text-left">
            <tr>
              <th
                v-for="(col, index) in eventColumns"
                :key="'result_' + index"
                class="table-header"
                :data-test="`trace-events-table-th-${col.label}`"
              >
                {{ col.label }}
              </th>
            </tr>
          </thead>
        </template>

        <template v-slot="{ item: row, index }">
          <q-tr
            :data-test="`trace-event-detail-${
              row[store.state.zoConfig.timestamp_column]
            }`"
            :key="'expand_' + index"
            @click="expandEvent(index)"
            style="cursor: pointer"
            class="pointer"
          >
            <q-td
              v-for="column in eventColumns"
              :key="index + '-' + column.name"
              class="field_list"
              style="cursor: pointer"
            >
              <div class="flex row items-center no-wrap">
                <q-btn
                  v-if="column.name === '@timestamp'"
                  :icon="
                    expandedEvents[index.toString()]
                      ? 'expand_more'
                      : 'chevron_right'
                  "
                  dense
                  size="xs"
                  flat
                  class="q-mr-xs"
                  @click.stop="expandEvent(index)"
                ></q-btn>
                {{ column.prop(row) }}
              </div>
            </q-td>
          </q-tr>
          <q-tr v-if="expandedEvents[index.toString()]">
            <td colspan="2">
              <pre class="log_json_content">{{ row }}</pre>
            </td>
          </q-tr>
        </template>
      </q-virtual-scroll>
      <div
        class="full-width text-center q-pt-lg text-bold"
        v-if="!spanDetails.events.length"
      >
        No events present for this span
      </div>
    </q-tab-panel>
  </q-tab-panels>
</template>

<script lang="ts">
import { cloneDeep } from "lodash-es";
import { date, type QTableProps } from "quasar";
import { defineComponent, onBeforeMount, ref, watch, type Ref } from "vue";
import { useStore } from "vuex";
import { useI18n } from "vue-i18n";

export default defineComponent({
  name: "TraceDetailsSidebar",
  props: {
    span: {
      type: Object,
      default: () => null,
    },
  },
  emits: ["close"],
  setup(props, { emit }) {
    const { t } = useI18n();
    const activeTab = ref("tags");
    const tags: Ref<{ [key: string]: string }> = ref({});
    const processes: Ref<{ [key: string]: string }> = ref({});
    const closeSidebar = () => {
      emit("close");
    };
    const spanDetails: any = ref({
      attrs: {},
      events: [],
    });
    const pagination: any = ref({
      rowsPerPage: 0,
    });

    watch(
      () => props.span,
      () => {
        spanDetails.value = getFormattedSpanDetails();
      }
    );

    onBeforeMount(() => {
      spanDetails.value = getFormattedSpanDetails();
    });

    const store = useStore();
    const expandedEvents: any = ref({});
    const eventColumns = ref([
      {
        name: "@timestamp",
        field: (row: any) =>
          date.formatDate(
            Math.floor(row[store.state.zoConfig.timestamp_column] / 1000000),
            "MMM DD, YYYY HH:mm:ss.SSS Z"
          ),
        prop: (row: any) =>
          date.formatDate(
            Math.floor(row[store.state.zoConfig.timestamp_column] / 1000000),
            "MMM DD, YYYY HH:mm:ss.SSS Z"
          ),
        label: "Timestamp",
        align: "left",
        sortable: true,
      },
      {
        name: "source",
        field: (row: any) => JSON.stringify(row),
        prop: (row: any) => JSON.stringify(row),
        label: "source",
        align: "left",
        sortable: true,
      },
    ]);

    const expandEvent = (index: number) => {
      if (expandedEvents.value[index.toString()])
        delete expandedEvents.value[index.toString()];
      else expandedEvents.value[index.toString()] = true;
    };

    const getSpanKind = (id: number) => {
      const spanKindMapping: { [key: number]: string } = {
        1: "Server",
        2: "Client",
        3: "Producer",
        4: "Consumer",
        5: "Internal",
      };
      return spanKindMapping[id] || id;
    };

    const getFormattedSpanDetails = () => {
      const spanDetails: { attrs: any; events: any[] } = {
        attrs: {},
        events: [],
      };

      spanDetails.attrs = cloneDeep(props.span);

      if (spanDetails.attrs.events) delete spanDetails.attrs.events;

      spanDetails.attrs.duration = spanDetails.attrs.duration + "us";
      spanDetails.attrs[store.state.zoConfig.timestamp_column] =
        date.formatDate(
          Math.floor(
            spanDetails.attrs[store.state.zoConfig.timestamp_column] / 1000
          ),
          "MMM DD, YYYY HH:mm:ss.SSS Z"
        );
      spanDetails.attrs.span_kind = getSpanKind(spanDetails.attrs.span_kind);

      spanDetails.events = JSON.parse(props.span.events).map(
        (event: any) => event
      );

      return spanDetails;
    };

    const span_details = new Set([
      "span_id",
      "trace_id",
      "operation_name",
      store.state.zoConfig.timestamp_column,
      "start_time",
      "end_time",
      "duration",
      "busy_ns",
      "idle_ns",
      "events",
    ]);

    watch(
      () => props.span,
      () => {
        Object.keys(props.span).forEach((key: string) => {
          if (!span_details.has(key)) {
            tags.value[key] = props.span[key];
          }
        });

        processes.value["service_name"] = props.span["service_name"];
        processes.value["service_service_instance"] =
          props.span["service_service_instance"];
        processes.value["service_service_version"] =
          props.span["service_service_version"];
      },
      {
        deep: true,
        immediate: true,
      }
    );

    return {
      t,
      activeTab,
      closeSidebar,
      eventColumns,
      expandedEvents,
      expandEvent,
      pagination,
      spanDetails,
      store,
      tags,
      processes,
    };
  },
});
</script>

<style scoped lang="scss">
.span_details_tab-panels {
  table {
    border-collapse: collapse;
    width: 100%;
    /* Other styling properties */
  }

  th,
  td {
    border: 1px solid #f0f0f0;
    text-align: left;
    padding: 4px 8px !important;
    font-size: 13px;
    /* Other styling properties */
  }
}
.attr-text {
  font-size: 12px;
  font-family: monospace;
}
.table-header {
  // text-transform: capitalize;

  .table-head-chip {
    background-color: $accent;
    padding: 0px;

    .q-chip__content {
      margin-right: 0.5rem;
      font-size: 0.75rem;
      color: $dark;
    }

    .q-chip__icon--remove {
      height: 1rem;
      width: 1rem;
      opacity: 1;
      margin: 0;

      &:hover {
        opacity: 0.7;
      }
    }

    .q-table th.sortable {
      cursor: pointer;
      text-transform: capitalize;
      font-weight: bold;
    }
  }

  &.isClosable {
    padding-right: 26px;
    position: relative;

    .q-table-col-close {
      transform: translateX(26px);
      position: absolute;
      margin-top: 2px;
      color: grey;
      transition: transform 0.3s cubic-bezier(0.25, 0.8, 0.5, 1);
    }
  }

  .q-table th.sortable {
    cursor: pointer;
    text-transform: capitalize;
    font-weight: bold;
  }

  .log_json_content {
    white-space: pre-wrap;
  }
}
.q-table__top {
  padding-left: 0;
  padding-top: 0;
}

.q-table thead tr,
.q-table tbody td,
.q-table th,
.q-table td {
  height: 25px;
  padding: 0px 5px;
  font-size: 0.75rem;
}

.q-table__bottom {
  width: 100%;
}

.q-table__bottom {
  min-height: 40px;
  padding-top: 0;
  padding-bottom: 0;
}

.q-td {
  overflow: hidden;
  min-width: 100px;

  .expanded {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    word-break: break-all;
  }
}

.thead-sticky tr > *,
.tfoot-sticky tr > * {
  position: sticky;
  opacity: 1;
  z-index: 1;
  background: #f5f5f5;
}

.q-table--dark .thead-sticky tr > *,
.q-table--dark .tfoot-sticky tr > * {
  background: #565656;
}
.thead-sticky tr:last-child > * {
  top: 0;
}

.tfoot-sticky tr:first-child > * {
  bottom: 0;
}

.field_list {
  padding: 0px;
  margin-bottom: 0.125rem;
  position: relative;
  overflow: visible;
  cursor: default;
  font-size: 12px;
  font-family: monospace;

  .field_overlay {
    position: absolute;
    height: 100%;
    right: 0;
    top: 0;
    background-color: #ffffff;
    border-radius: 6px;
    padding: 0 6px;
    visibility: hidden;
    display: flex;
    align-items: center;
    transition: all 0.3s linear;

    .q-icon {
      cursor: pointer;
      opacity: 0;
      transition: all 0.3s linear;
      margin: 0 1px;
    }
  }

  &:hover {
    .field_overlay {
      visibility: visible;

      .q-icon {
        opacity: 1;
      }
    }
  }
}
.span_details_tab-panels {
  height: calc(100% - 102px);
  overflow-y: auto;
  overflow-x: hidden;
}

.hearder_bg {
  border-top: 1px solid $border-color;
  background-color: color-mix(in srgb, currentColor 5%, transparent);
}
</style>

<style lang="scss">
.span_details_tabs {
  .q-tab__indicator {
    display: none;
  }
  .q-tab--active {
    border-bottom: 1px solid var(--q-primary);
  }
}

.span_details_tab-panels {
  .q-tab-panel {
    padding: 8px 0 8px 8px;
  }
}
</style>
