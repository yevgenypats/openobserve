// Copyright 2022 Zinc Labs Inc. and Contributors

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//      http:www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

import { describe, expect, it, beforeEach, vi, afterEach } from "vitest";
import { mount, flushPromises, DOMWrapper } from "@vue/test-utils";
import { installQuasar } from "../../helpers/install-quasar-plugin";
import { Dialog, Notify } from "quasar";

import Index from "@/plugins/traces/Index.vue";
import SearchResult from "@/plugins/traces/SearchResult.vue";
import i18n from "@/locales";
import store from "../../helpers/store";
import { rest } from "msw";
import traces from "../../mockData/traces";
import "plotly.js";
import DetailTable from "@/plugins/traces/TraceDetails.vue";
import router from "../../helpers/router";
import TraceChart from "src/plugins/traces/TraceChart.vue";
import exp from "node:constants";

const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);

installQuasar({
  plugins: [Dialog, Notify],
});

describe("Search Result", async () => {
  let wrapper: any;
  beforeEach(async () => {
    vi.useFakeTimers();
    wrapper = mount(Index, {
      attachTo: "#app",
      global: {
        provide: {
          store: store,
        },
        plugins: [i18n, router],
        stubs: {
          SearchBar: true,
          IndexList: true,
        },
      },
    });
    await flushPromises();
    vi.advanceTimersByTime(500);
    await flushPromises();
  });

  afterEach(() => {
    wrapper.unmount();
    vi.restoreAllMocks();
    // vi.clearAllMocks();
  });

  it("Should show the traces scatter chart", async () => {
    expect(
      wrapper.find('[data-test="traces-search-result-scatter-chart"]').exists()
    ).toBeTruthy();
  });

  it("Should render table header with columns", () => {
    const tableHeaders = wrapper
      .find('[data-test="traces-result-table-head"]')
      .findAll("th");

    expect(tableHeaders[0].text()).toBe("Start Time");
    expect(tableHeaders[1].text()).toBe("Operation");
    expect(tableHeaders[2].text()).toBe("Service");
    expect(tableHeaders[3].text()).toBe("Duration");
  });

  it("Should render table rows with data", () => {
    const tableBodyRows = wrapper.findAll(
      '[data-test="traces-result-table-body-row"]'
    );
    const tableData = tableBodyRows[0].findAll("td");
    expect(tableData[0].text()).toBe("May 04, 2023 15:36:34.636 +05:30");
    expect(tableData[1].text()).toBe("/api/{org_id}/{stream_name}/_json");
    expect(tableData[2].text()).toBe("zo-zo1-zincobserve-ingester-0");
    expect(tableData[3].text()).toBe("12ms");
  });

  it("Should not render trace details component", () => {
    expect(wrapper.find('[data-test="trace-details-popup"]').exists()).toBe(
      false
    );
  });

  describe("When user click on any trace", () => {
    const domWrapper = new DOMWrapper(document.body);
    beforeEach(async () => {
      global.server.use(
        rest.post(
          `${store.state.API_ENDPOINT}/api/${store.state.selectedOrganization.identifier}/_search`,
          (req, res, ctx) => {
            let resData;
            if (req.url.searchParams.get("type") === "traces")
              resData = traces.span_list;
            return res(ctx.status(200), ctx.json(resData));
          }
        )
      );
      await wrapper
        .findAll('[data-test="traces-result-table-body-row"]')[0]
        .trigger("click");
    });
    it("should emit get trace details event", async () => {
      expect(
        wrapper.findComponent(SearchResult).emitted()["get:traceDetails"]
      ).toBeTruthy();
      await flushPromises();
      expect(
        domWrapper.find('[data-test="trace-details-popup"]').isVisible()
      ).toBe(true);
    });
  });
});
