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
    class="flex wrap justify-start items-center bg-white"
    :class="defocusSpan ? 'defocus' : ''"
    :style="{
      zIndex: 2,
      borderBottom:
        (isSpanSelected && `2px solid ${span.style.color}`) || 'none',
    }"
  >
    <div
      class="flex justify-between items-end cursor-pointer span-block relative-position bg-white"
      :style="{
        height: spanDimensions.height + 'px',
        width: '100%',
        paddingBottom: '6px',
      }"
      ref="spanBlock"
      @click="selectSpan"
    >
      <div
        :style="{
          width: '100%',
          overflow: 'hidden',
        }"
        class="cursor-pointer flex items-center no-wrap position-relative"
        :class="defocusSpan ? 'defocus' : ''"
        @click="selectSpan"
      >
        <div
          :style="{
            height: spanDimensions.barHeight + 'px',
            width: getWidth + '%',
            left: getLeftPosition + '%',
            position: 'relative',
          }"
          class="flex justify-start items-center no-wrap"
          ref="spanMarkerRef"
        >
          <q-icon
            dense
            round
            flat
            name="expand_more"
            class="collapse-btn"
            :style="{
              rotate: isSpanSelected ? '0deg' : '270deg',
            }"
            @click.prevent.stop="toggleSpanDetails()"
          />

          <div
            :style="{
              width: 'calc(100% - 21px)',
              height: '100%',
              borderRadius: '2px',
              backgroundColor: span.style?.color || '#58508d',
            }"
          />
        </div>
        <div
          :style="{
            position: 'absolute',
            ...getDurationStyle,
            transition: 'all 0.5s ease',
            zIndex: 1,
          }"
          class="text-caption"
        >
          {{ formatTimeWithSuffix(span.durationUs) }}
        </div>
        <q-resize-observer debounce="300" @resize="onResize" />
      </div>
    </div>
    <template v-if="isSpanSelected">
      <span-details
        :style="{
          borderTop: `2px solid ${span.style.color}`,
        }"
        :span="span"
        :spanData="spanData"
        :baseTracePosition="baseTracePosition"
      ></span-details>
    </template>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed, ref } from "vue";
import useTraces from "@/composables/useTraces";
import { getImageURL, formatTimeWithSuffix } from "@/utils/zincutils";
import SpanDetails from "./SpanDetails.vue";

export default defineComponent({
  name: "SpanBlock",
  props: {
    span: {
      type: Object,
      default: () => null,
    },
    baseTracePosition: {
      type: Object,
      default: () => null,
    },
    depth: {
      type: Number,
      default: 0,
    },
    styleObj: {
      type: Object,
      default: () => {},
    },
    showCollapse: {
      type: Boolean,
      default: true,
    },
    isCollapsed: {
      type: Boolean,
      default: false,
    },
    spanDimensions: {
      type: Object,
      default: () => {},
    },
    spanData: {
      type: Object,
      default: () => ({}),
    },
  },
  emits: ["toggleCollapse"],
  components: { SpanDetails },
  setup(props, { emit }) {
    const { searchObj } = useTraces();
    const spanBlock: any = ref(null);
    const spanBlockWidth = ref(0);
    const onePixelPercent = ref(0);
    const defocusSpan = computed(() => {
      if (!searchObj.data.traceDetails.selectedSpanId) return false;
      return searchObj.data.traceDetails.selectedSpanId !== props.span.spanId;
    });
    const selectSpan = () => {
      searchObj.data.traceDetails.showSpanDetails = true;
      searchObj.data.traceDetails.selectedSpanId = props.span.spanId;
    };
    const toggleSpanCollapse = () => {
      emit("toggleCollapse", props.span.spanId);
    };

    const isSpanSelected = computed(() => {
      return searchObj.data.traceDetails.expandedSpans.includes(
        props.span.spanId
      );
    });

    const spanMarkerRef = ref(null);

    const getLeftPosition = computed(() => {
      const left =
        props.span.startTimeMs - props.baseTracePosition["startTimeMs"];

      // if (props.span.startTimeMs < props.baseTracePosition["startTimeMs"]) {
      //   const left =
      //     props.baseTracePosition["startTimeMs"] - props.span.startTimeMs;
      //   // props.baseTracePosition + props.baseTracePosition["durationMs"];
      //   return -(left / props.baseTracePosition?.durationMs) * 100;
      // }
      // // console.log(
      // //   props.span.startTimeMs,
      // //   props.baseTracePosition["startTimeMs"],
      // //   left,
      // //   props.baseTracePosition?.durationMs
      // // );
      return (left / props.baseTracePosition?.durationMs) * 100;
    });
    const getWidth = computed(() => {
      return Number(
        (
          (props.span?.durationMs / props.baseTracePosition?.durationMs) *
          100
        ).toFixed(2)
      );
    });
    const getDurationStyle = computed(() => {
      const style: any = {
        top: "10px",
      };
      const onePercent = Number((spanBlockWidth.value / 100).toFixed(2));
      const labelWidth = 60;
      if (
        (getLeftPosition.value + getWidth.value) * onePercent + labelWidth >
        spanBlockWidth.value
      ) {
        style.right = 0;
        style.top = "0";
      } else if (getLeftPosition.value > 50) {
        style.left =
          getLeftPosition.value * onePercent - labelWidth + 10 + "px";
      } else {
        style.left =
          (getLeftPosition.value +
            (Math.floor(getWidth.value) ? getWidth.value : 1)) *
            onePercent +
          "px";
      }
      return style;
    });

    const getSpanStartTime = computed(() => {
      return props.span.startTimeMs - props.baseTracePosition["startTimeMs"];
    });

    const onResize = () => {
      spanBlockWidth.value = spanBlock.value.clientWidth;
    };

    const toggleSpanDetails = () => {
      if (!isSpanSelected.value) {
        searchObj.data.traceDetails.expandedSpans.push(props.span.spanId);
      } else {
        searchObj.data.traceDetails.expandedSpans =
          searchObj.data.traceDetails.expandedSpans.filter(
            (val) => props.span.spanId !== val
          );
      }
    };

    return {
      formatTimeWithSuffix,
      selectSpan,
      toggleSpanCollapse,
      getImageURL,
      getLeftPosition,
      getWidth,
      getDurationStyle,
      spanBlock,
      onResize,
      onePixelPercent,
      getSpanStartTime,
      spanMarkerRef,
      toggleSpanDetails,
      defocusSpan,
      isSpanSelected,
    };
  },
});
</script>

<style scoped lang="scss">
.defocus {
  opacity: 0.3;
}

.collapse-btn {
  background-color: #ffffff;
  opacity: 0.6;
}

.collapse-container {
  border: 1px solid #ececec;
}

.light-grey {
  background-color: #ececec;
}
</style>
