<template>
    <div style="height:100%; width:100%;">
        <div ref="plotRef"></div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, reactive, ref, watch } from 'vue';
import Plotly from "plotly.js";

export default defineComponent({
    name: "MapChart",
    props: {
        data: {
            type: Array,
            default: () => []
        }, 
        layout : {
            type: Object,
            default: () => null
        }
    },

    setup(props) {
        const plotRef: any = ref(null);

        onMounted(()=>{
            Plotly.newPlot(plotRef.value, props.data as any, props.layout, {showLink: false});
            
        })

        watch(
          () => [props.data, props.layout],
          async () => {
            Plotly.react(
                      plotRef.value,
                      props.data as any,
                      props.layout,
                      {
                          responsive: true,
                          displaylogo: false,
                          displayModeBar: false,
                      }
                  );
          },  { deep: true })
        
        return {
            plotRef,
        }
    }
})
</script>

<style lang="scss" scoped>

</style>