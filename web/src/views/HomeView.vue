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

<template>
  <q-page class="q-pa-lg">
    <div
      v-if="!no_data_ingest"
      class="q-pa-md row items-start q-gutter-md"
      style="margin: 0 auto; justify-content: center"
    >
      <q-card class="my-card">
        <q-card-section align="center" flat bordered class="my-card">
          <div class="text-subtitle1">{{ t("home.streams") }}</div>
          <div class="text-h6">{{ summary.streams_count }}</div>
          <div class="text-subtitle1">{{ t("home.totalDataIngested") }}</div>
          <div class="text-h6">{{ summary.ingested_data }}</div>
        </q-card-section>

        <q-separator />

        <q-card-actions align="center">
          <q-btn no-caps color="primary" flat
            >{{ t("home.view") }}
            <router-link
              exact
              :to="{ name: 'logstreams' }"
              class="absolute full-width full-height"
            ></router-link>
          </q-btn>
        </q-card-actions>
      </q-card>

      <q-card align="center" class="my-card">
        <q-card-section align="center" flat bordered class="my-card">
          <div class="text-subtitle1">{{ t("home.queryFunctions") }}</div>
          <div class="text-h6">{{ summary.query_fns }}</div>
          <div class="text-subtitle1">{{ t("home.ingestFunctions") }}</div>
          <div class="text-h6">{{ summary.ingest_fns }}</div>
        </q-card-section>
        <q-separator />
        <q-card-actions align="center">
          <q-btn no-caps color="primary" flat
            >{{ t("home.view") }}
            <router-link
              exact
              :to="{ name: 'functions' }"
              class="absolute full-width full-height"
            ></router-link>
          </q-btn>
        </q-card-actions>
      </q-card>

      <q-card class="my-card">
        <q-card-section align="center" flat bordered class="my-card">
          <div class="text-subtitle1">{{ t("home.scheduledAlert") }}</div>
          <div class="text-h6">{{ summary.scheduled_alerts }}</div>
          <div class="text-subtitle1">{{ t("home.rtAlert") }}</div>
          <div class="text-h6">{{ summary.rt_alerts }}</div>
        </q-card-section>
        <q-separator />
        <q-card-actions align="center">
          <q-btn no-caps color="primary" flat
            >{{ t("home.view") }}
            <router-link
              exact
              :to="{ name: 'alerts' }"
              class="absolute full-width full-height"
            ></router-link>
          </q-btn>
        </q-card-actions>
      </q-card>
    </div>

    <div
      v-if="no_data_ingest"
      class="q-pa-md row items-start q-gutter-md"
      style="margin: 0 auto; justify-content: center"
    >
      <q-card class="my-card">
        <q-card-section align="center" flat bordered class="my-card">
          <div class="text-h6">{{ t("home.noData") }}</div>
          <div class="text-subtitle1">{{ t("home.ingestionMsg") }}</div>
        </q-card-section>

        <q-separator />

        <q-card-actions align="center">
          <q-btn
            no-caps
            color="primary"
            @click="() => $router.push('/ingestion/logs/fluentbit')"
            flat
            >{{ t("home.findIngestion") }}
          </q-btn>
        </q-card-actions>
      </q-card>

      <q-card v-if="isCloud === 'true'" class="my-card">
        <q-card-section align="center" flat bordered class="my-card">
          <div class="text-h6">{{ t("home.inestedInSearch") }}</div>
          <div class="text-subtitle1">{{ t("home.searchInDemoOrg") }}</div>
        </q-card-section>

        <q-separator />
      </q-card>
    </div>
    <div class="row justify-center items-center">
      <video width="400" height="225" controls>
        <source
          src="https://videos.openobserve.ai/OpenObserve_Introduction.mp4"
          type="video/mp4"
        />
        Your browser does not support the video tag.
      </video>
    </div>
    <RenderDashboardCharts :draggable="false" :dashboardData='JSON.parse(JSON.stringify({"version":2,"dashboardId":"7100458381567205376Octeio","title":"Chart enhancement","description":"","role":"","owner":"","created":"2023-08-24T12:46:37.083Z","panels":[{"id":"Panel_ID4629710","type":"area","title":"Area 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":7,"w":6,"h":7,"i":1}},{"id":"Panel_ID6253810","type":"bar","title":"Bar 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":0,"w":6,"h":7,"i":2}},{"id":"Panel_ID889510","type":"h-bar","title":"H-bar 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":14,"w":4,"h":10,"i":3}},{"id":"Panel_ID3783010","type":"line","title":"line 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":7,"w":6,"h":7,"i":4}},{"id":"Panel_ID4135310","type":"pie","title":"pie 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":0,"w":6,"h":7,"i":5}},{"id":"Panel_ID3303810","type":"scatter","title":"scatter 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":24,"w":6,"h":7,"i":6}},{"id":"Panel_ID4731510","type":"table","title":"table 1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", sum(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"sum"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":4,"y":14,"w":3,"h":10,"i":7}},{"id":"Panel_ID4865310","type":"area","title":"area 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":38,"w":6,"h":7,"i":14}},{"id":"Panel_ID1533810","type":"bar","title":"bar 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#31c469","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":31,"w":6,"h":7,"i":15}},{"id":"Panel_ID7093810","type":"h-bar","title":"H-bar 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":45,"w":6,"h":7,"i":16}},{"id":"Panel_ID4302410","type":"line","title":"line 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":24,"w":6,"h":7,"i":18}},{"id":"Panel_ID9003810","type":"scatter","title":"scatter 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":38,"w":6,"h":7,"i":19}},{"id":"Panel_ID3306510","type":"table","title":"table 1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_docker_id as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Docker Id","alias":"x_axis_1","column":"kubernetes_docker_id","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":31,"w":6,"h":7,"i":20}},{"id":"Panel_ID4183310","type":"area","title":"area >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":52,"w":6,"h":7,"i":21}},{"id":"Panel_ID9907610","type":"bar","title":"bar >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":59,"w":6,"h":7,"i":22}},{"id":"Panel_ID9867010","type":"h-bar","title":"H-bar >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":66,"w":6,"h":7,"i":23}},{"id":"Panel_ID2098410","type":"stacked","title":"stacked >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":45,"w":6,"h":7,"i":24}},{"id":"Panel_ID5099010","type":"line","title":"line >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":73,"w":6,"h":7,"i":25}},{"id":"Panel_ID6143110","type":"scatter","title":"scatter >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":52,"w":6,"h":7,"i":26}},{"id":"Panel_ID7423210","type":"table","title":"table >1-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":80,"w":6,"h":7,"i":27}},{"id":"Panel_ID2755810","type":"area","title":"area >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":59,"w":6,"h":7,"i":28}},{"id":"Panel_ID8254410","type":"bar","title":"bar >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_container_name as \"x_axis_1\", stream as \"x_axis_2\", count(stream) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Container Name","alias":"x_axis_1","column":"kubernetes_container_name","color":null},{"label":"Stream","alias":"x_axis_2","column":"stream","color":null}],"y":[{"label":"Stream","alias":"y_axis_1","column":"stream","color":"#bac431","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":87,"w":6,"h":6,"i":29}},{"id":"Panel_ID6676110","type":"h-bar","title":"H-bar >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":66,"w":6,"h":7,"i":30}},{"id":"Panel_ID6361010","type":"line","title":"line >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":93,"w":6,"h":7,"i":31}},{"id":"Panel_ID1481010","type":"scatter","title":"scatter >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":73,"w":6,"h":7,"i":32}},{"id":"Panel_ID7660110","type":"table","title":"table >1->1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\", count(stream) as \"y_axis_2\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"},{"label":"Stream","alias":"y_axis_2","column":"stream","color":"#c23531","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":100,"w":6,"h":6,"i":33}},{"id":"Panel_ID9710","type":"bar","title":"test","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(kubernetes_pod_id) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Id","alias":"y_axis_1","column":"kubernetes_pod_id","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":80,"w":6,"h":7,"i":34}},{"id":"Panel_ID8680610","type":"bar","title":"cv","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", count(distinct(kubernetes_pod_name)) as \"y_axis_1\"  FROM \"gke-fluentbit\" GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null}],"y":[{"label":"Kubernetes Pod Name","alias":"y_axis_1","column":"kubernetes_pod_name","color":"#5960b2","aggregationFunction":"count-distinct"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":0,"y":106,"w":6,"h":7,"i":35}},{"id":"Panel_ID603710","type":"line","title":"m","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"promql","queries":[{"query":"topk(10, irate(zo_ingest_records{namespace=\"ziox-alpha1\"}[5m]))","customQuery":true,"fields":{"stream":"grafana_feature_toggles_info","stream_type":"metrics","x":[],"y":[],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":94,"w":6,"h":6,"i":36}},{"id":"Panel_ID3406910","type":"table","title":"tble","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT histogram(_timestamp) as \"x_axis_1\", count(kubernetes_host) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Timestamp","alias":"x_axis_1","column":"_timestamp","color":null,"aggregationFunction":"histogram"}],"y":[{"label":"Kubernetes Host","alias":"y_axis_1","column":"kubernetes_host","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":100,"w":6,"h":6,"i":37}},{"id":"Panel_ID2620910","type":"area-stacked","title":"Stacked Area 2-1","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":106,"w":6,"h":7,"i":38}},{"id":"Panel_ID1204010","type":"stacked","title":"stacked - bar","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT kubernetes_host as \"x_axis_1\", code as \"x_axis_2\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1, x_axis_2 ORDER BY x_axis_1, x_axis_2","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Kubernetes Host","alias":"x_axis_1","column":"kubernetes_host","color":null},{"label":"Code","alias":"x_axis_2","column":"code","color":null}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":6,"y":87,"w":6,"h":7,"i":39}},{"id":"Panel_ID6685910","type":"bar","title":"timeseries","description":"","config":{"show_legends":true,"legends_position":null},"queryType":"sql","queries":[{"query":"SELECT histogram(_timestamp) as \"x_axis_1\", count(code) as \"y_axis_1\"  FROM \"gke-fluentbit\"  GROUP BY x_axis_1 ORDER BY x_axis_1","customQuery":false,"fields":{"stream":"gke-fluentbit","stream_type":"logs","x":[{"label":"Timestamp","alias":"x_axis_1","column":"_timestamp","color":null,"aggregationFunction":"histogram"}],"y":[{"label":"Code","alias":"y_axis_1","column":"code","color":"#5960b2","aggregationFunction":"count"}],"z":[],"filter":[]},"config":{"promql_legend":""}}],"layout":{"x":7,"y":14,"w":5,"h":10,"i":40}}],"variables":null}))' :currentTimeObj="timeObj"/>
  </q-page>
</template>

<script lang="ts">
import { useQuasar } from "quasar";
import { defineComponent, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useStore } from "vuex";
import orgService from "../services/organizations";
import config from "../aws-exports";
import { formatSizeFromMB } from "@/utils/zincutils";
import RenderDashboardCharts from "./Dashboards/RenderDashboardCharts.vue";
export default defineComponent({
    name: "PageHome",
    components:{
        RenderDashboardCharts
    },
    setup() {
    const store = useStore();
    const { t } = useI18n();
    const summary: any = ref([]);
    const $q = useQuasar();
    const no_data_ingest = ref(false);
    const isCloud = config.isCloud;
    const getSummary = (org_id: any) => {
      const dismiss = $q.notify({
        spinner: true,
        message: "Please wait while loading summary...",
      });
      orgService
        .get_organization_summary(org_id)
        .then((res) => {
          if (
            res.data.streams.length == 0 &&
            res.data.functions.length == 0 &&
            res.data.alerts.length == 0
          ) {
            no_data_ingest.value = true;
            summary.value = {};
            dismiss();
            return;
          }
          let sum = 0;
          if (res.data.streams.length > 0) {
            sum = res.data.streams.reduce(
              (
                acc: number,
                val: {
                  [x: string]: any;
                  storage_size: any;
                }
              ) => {
                return acc + val.stats.storage_size;
              },
              0
            );
          }

          let ingest_fns = 0;
          let query_fns = 0;
          if (res.data.functions.length > 0) {
            res.data.functions.forEach((fn: { stream_name: any }) => {
              if (fn.stream_name && fn.stream_name != "") {
                ingest_fns += 1;
              } else {
                query_fns += 1;
              }
            });
          }

          let rt_alerts = 0;
          let scheduled_alerts = 0;
          if (res.data.alerts.length > 0) {
            res.data.alerts.forEach((alert: { is_real_time: any }) => {
              if (alert.is_real_time) {
                rt_alerts += 1;
              } else {
                scheduled_alerts += 1;
              }
            });
          }
          summary.value = {
            streams_count: res.data.streams.length,
            ingested_data: formatSizeFromMB(sum.toFixed(2)),
            ingest_fns: ingest_fns,
            query_fns: query_fns,
            rt_alerts: rt_alerts,
            scheduled_alerts: scheduled_alerts,
          };
          no_data_ingest.value = false;
          dismiss();
        })
        .catch((err) => {
          console.log(err);
          dismiss();
          $q.notify({
            type: "negative",
            message: "Error while pulling summary.",
            timeout: 2000,
          });
        });
    };

    getSummary(store.state.selectedOrganization.identifier);

    return {
      t,
      store,
      summary,
      no_data_ingest,
      getSummary,
      isCloud,
      timeObj:{
              start_time:new Date(new Date().getTime() - 15 * 60 * 1000) ,
              end_time: new Date(),
            }
    };
  },
  computed: {
    selectedOrg() {
      return this.store.state.selectedOrganization?.identifier;
    },
  },
  watch: {
    selectedOrg(newVal: any, oldVal: any) {
      if (newVal != oldVal || this.summary.value == undefined) {
        this.summary = {};
        this.getSummary(this.store.state?.selectedOrganization?.identifier);
      }
    },
  },
});
</script>

<style scoped lang="scss">
.pointer-to-demo {
  margin-left: auto;
  margin-right: 0;
}

.pointer-description {
  display: flex;
  align-items: center;
}

.my-card {
  background-color: rgba(0, 0, 0, 0.045);
}
</style>
