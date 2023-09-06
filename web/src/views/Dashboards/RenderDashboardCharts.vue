<!-- Copyright 2023 Zinc Labs Inc.

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

<!-- eslint-disable vue/v-on-event-hyphenation -->
<!-- eslint-disable vue/attribute-hyphenation -->
<template>
      <VariablesValueSelector :variablesConfig="dashboardData?.variables" :selectedTimeDate="currentTimeObj" 
        @variablesData="variablesDataUpdated"/>
      <div class="displayDiv">
        <grid-layout v-if="dashboardData.panels?.length > 0" :layout.sync="getDashboardLayout(dashboardData)" :col-num="12" :row-height="30"
          :is-draggable="draggable" :is-resizable="draggable" :vertical-compact="true" :autoSize="true"
          :restore-on-drag="true" :use-css-transforms="false">
          <grid-item class="plotlyBackground" v-for="item in dashboardData.panels" :key="item.id"
            :x="getPanelLayout(item,'x')" :y="getPanelLayout(item,'y')"
            :w="getPanelLayout(item,'w')" :h="getPanelLayout(item,'h')"
            :i="getPanelLayout(item,'i')" :minH="getMinimumHeight(item.type)" :minW="getMinimumWidth(item.type)" @resized="resizedEvent" @moved="movedEvent"
            drag-allow-from=".drag-allow">
            <div style="height: 100%;">
              <PanelContainer @updated:chart="onUpdatePanel" @duplicatePanel="onDuplicatePanel" :showOption="draggable" :draggable="draggable" :data="item"
                :selectedTimeDate="currentTimeObj" :variablesData="variablesData"
                :width="getPanelLayout(item,'w')" :height="getPanelLayout(item,'h')">
              </PanelContainer>
            </div>
          </grid-item>
        </grid-layout>
      </div>
      <div v-if="!dashboardData.panels?.length">
       <!-- if data not available show nodata component -->
        <NoPanel @update:Panel="addPanelData" />
      </div>
  </template>
  
  <script lang="ts">
  // @ts-nocheck
  import {
    defineComponent,
    ref,
    watch,
    onActivated,
    nextTick,
onMounted,
onUnmounted,
  } from "vue";
  import { useStore } from "vuex";
  import { useQuasar } from "quasar";
  import { useI18n } from "vue-i18n";
  import VueGridLayout from "vue3-grid-layout";
  import { useRouter } from "vue-router";
  import {
    addPanel
  } from "../../utils/commons.ts";
  import { toRaw, unref, reactive } from "vue";
  import PanelContainer from "../../components/dashboards/PanelContainer.vue";
  import { useRoute } from "vue-router";
  import { deletePanel, updateDashboard } from "../../utils/commons";
  import NoPanel from "../../components/shared/grid/NoPanel.vue";
  import VariablesValueSelector from "../../components/dashboards/VariablesValueSelector.vue";
  
  export default defineComponent({
    name: "RenderDashboardCharts",
    props:["draggable","dashboardData","currentTimeObj"],
    components: {
      GridLayout: VueGridLayout.GridLayout,
      GridItem: VueGridLayout.GridItem,
      PanelContainer,
      NoPanel,
      VariablesValueSelector
  },
    setup(props) {
      const { t } = useI18n();
      const route = useRoute();
      const router = useRouter();
      const store = useStore();
      const currentDashboardData = reactive({
        data: {},
      });
      const $q = useQuasar();
      const currentDurationSelectionObj = ref ({})
      // const currentTimeObj = ref({});
      const refreshInterval = ref(0);
      const selectedDate = ref()
  
      // variables data
      const variablesData = reactive({});
      const variablesDataUpdated = (data: any) => {
        Object.assign(variablesData,data)
      }
  
      // onActivated(async () => {        
      //   await loadDashboard();        
      // })


      // console.log("props",props);
      

      onMounted(()=>{
        console.log("mounted");
        // console.log(props);
      })

      onActivated(async()=>{
        console.log("activated");
        await nextTick();
        console.log("props",props);
      })

      // watch(()=>props,()=>{
      //   console.log("watch");
      // },{deep:true})

      onUnmounted(()=>{
        console.log("unmounted");
      })

      const loadDashboard = async () => {
        
        // let data = JSON.parse(JSON.stringify(await getDashboard(
        //   store,
        //   route.query.dashboard
        // )))
        // currentDashboardData.data = data;

        // console.log(props);
        
  
        // if variables data is null, set it to empty list
        if(!(props.dashboardData?.variables && props.dashboardData?.variables?.list.length)) {
          variablesData.isVariablesLoading = false
          variablesData.values = []
        }
       
      };
      //create a duplicate panel
      const onDuplicatePanel = async (data: any): Promise<void> => {
  
        // Show a loading spinner notification.
        const dismiss = $q.notify({
          spinner: true,
          message: "Please wait...",
          timeout: 2000,
        });
  
        // Generate a unique panel ID.
        const panelId = "Panel_ID" + Math.floor(Math.random() * (99999 - 10 + 1)) + 10;
  
        // Duplicate the panel data with the new ID.
        const panelData = JSON.parse(JSON.stringify(data));
        panelData.id = panelId;
  
        try {
          // Add the duplicated panel to the dashboard.
          await addPanel(store, route.query.dashboard, panelData);
  
          // Show a success notification.
          $q.notify({
            type: "positive",
            message: `Panel Duplicated Successfully`,
          });
  
          // Navigate to the new panel.
          return router.push({
            path: "/dashboards/add_panel",
            query: { dashboard: String(route.query.dashboard), panelId: panelId }
          });
        } catch (err) {
          // Show an error notification.
          $q.notify({
            type: "negative",
            message: err?.response?.data["error"]
              ? JSON.stringify(err?.response?.data["error"])
              : 'Panel duplication failed',
          });
        }
  
        // Hide the loading spinner notification.
        dismiss();
  
      };
  
      // save the dashboard value
      const saveDashboard = async () => {
        const dashboardId = route.query.dashboard
        await updateDashboard(
          store,
          store.state.selectedOrganization.identifier,
          dashboardId,
          currentDashboardData.data
        );
  
        $q.notify({
          type: "positive",
          message: "Dashboard updated successfully.",
          timeout: 5000,
        });
  
      };
  
      //add panel
      const addPanelData = () => {
        return router.push({
          path: "/dashboards/add_panel",
          query: { dashboard: route.query.dashboard },
        });
      };
      
      // watch(selectedDate, () => {
      //   const c = toRaw(unref(selectedDate.value));
      //   currentDurationSelectionObj.value = selectedDate.value
      //   currentTimeObj.value = getConsumableDateTime(currentDurationSelectionObj.value);
      // })
  
      // ------- work with query params ----------
      // onActivated(async() => {
      //   const params = route.query
  
      //   if(params.refresh) {
      //     refreshInterval.value = parseDuration(params.refresh)
      //   }
  
      //   if(params.period || (params.to && params.from)){
      //     selectedDate.value = getDurationObjectFromParams(params)
      //   }
  
      //   // resize charts if needed
      //   await nextTick();
      //   window.dispatchEvent(new Event("resize"))
      // })
  
      // whenever the refreshInterval is changed, update the query params
      // watch([refreshInterval, selectedDate], () => {
      //   router.replace({
      //     query: {
      //       org_identifier: store.state.selectedOrganization.identifier,
      //       dashboard: route.query.dashboard,
      //       refresh: generateDurationLabel(refreshInterval.value),
      //       ...getQueryParamsForDuration(selectedDate.value)
      //     }
      //   })
      // })
  
  
      const onUpdatePanel = async(panelDataElementValue: any) => {
        
        await deletePanel(
          store,
          route.query.dashboard,
          panelDataElementValue.id
        );
        await loadDashboard()
      }
  
      const movedEvent = (i, newX, newY) => {
        saveDashboard()
      }
  
      const resizedEvent = (i, newX, newY, newHPx, newWPx) => {
        window.dispatchEvent(new Event("resize"));
        saveDashboard()
      }
  
      const getDashboardLayout = (currentDashboardData)=>{
        //map on each panels and return array of layouts
        return currentDashboardData.panels?.map((item) => item.layout)||[];
      }
  
      const getPanelLayout = (currentDashboardData, position) => {      
            if (position == "x") {
              return currentDashboardData.layout?.x;
            } else if (position == "y") {
              return currentDashboardData?.layout?.y;
            } else if (position == "w") {
              return currentDashboardData?.layout?.w;
            } else if (position == "h") {
              return currentDashboardData?.layout?.h;
            } else if (position == "i") {
              return currentDashboardData?.layout?.i;
            }
        return 0;
      }
  
      const getMinimumHeight = (type) => {
        switch (type) {
          case "area":
          case "bar":
          case "h-bar":
          case "line":
          case "pie":
          case "scatter":
          case "table":
            return 4;
            break;
  
          default:
            break;
        }
      }
  
      const getMinimumWidth = (type) => {
        switch (type) {
          case "area":
          case "bar":
          case "h-bar":
          case "line":
          case "pie":
          case "scatter":
          case "table":
            return 3;
            break;
  
          default:
            break;
        }
      }
  
      return {
        currentDashboardData,
        addPanelData,
        onDuplicatePanel,
        t,
        onUpdatePanel,
        movedEvent,
        resizedEvent,
        getPanelLayout,
        getMinimumHeight,
        getMinimumWidth,
        variablesData,
        variablesDataUpdated,
        getDashboardLayout
      };
    }
  });
  </script>
  
  <style lang="scss" scoped>
  .q-table {
    &__top {
      border-bottom: 1px solid $border-color;
      justify-content: flex-end;
    }
  }
  
  .vue-grid-layout {
    // background: #eee;
  }
  
  .vue-grid-layout {
      transition: none;
    }
  
    .vue-grid-item {
      transition: none;
    }
  
  // .vue-grid-item:not(.vue-grid-placeholder) {
  //   background: #ccc;
  //   border: 1px solid black;
  // }
  
  .vue-grid-item {
    border: 1px solid black;
  }
  
  .vue-grid-item .resizing {
    opacity: 0.9;
  }
  
  .vue-grid-item .static {
    background: #cce;
  }
  
  .vue-grid-item .text {
    font-size: 24px;
    text-align: center;
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    margin: auto;
    height: 100%;
    width: 100%;
  }
  
  .vue-grid-item .no-drag {
    height: 100%;
    width: 100%;
  }
  
  .vue-grid-item .minMax {
    font-size: 12px;
  }
  
  .vue-grid-item .add {
    cursor: pointer;
  }
  
  .vue-draggable-handle {
    position: absolute;
    width: 20px;
    height: 20px;
    top: 0;
    left: 0;
    background: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='10' height='10'><circle cx='5' cy='5' r='5' fill='#999999'/></svg>") no-repeat;
    background-position: bottom right;
    padding: 0 8px 8px 0;
    background-repeat: no-repeat;
    background-origin: content-box;
    box-sizing: border-box;
    cursor: pointer;
  }
  
  .layoutJSON {
    background: #ddd;
    border: 1px solid black;
    margin-top: 10px;
    padding: 10px;
  }
  
  .eventsJSON {
    background: #ddd;
    border: 1px solid black;
    margin-top: 10px;
    padding: 10px;
    height: 100px;
    overflow-y: scroll;
  }
  
  .displayDiv {
    clear: both;
    // padding: 1.625em 0 0;
    // overflow: auto;
  }
  
  .plotlyBackground {
    background: #00000000 !important;
    border-radius: 4px;
    border-color: #c2c2c27a !important;
  }
  </style>
  