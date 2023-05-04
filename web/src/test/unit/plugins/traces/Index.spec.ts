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
import { mount, flushPromises } from "@vue/test-utils";
import { installQuasar } from "../../helpers/install-quasar-plugin";
import { Dialog, Notify } from "quasar";

import Index from "@/plugins/traces/Index.vue";
import i18n from "@/locales";
import store from "../../helpers/store";
import { rest } from "msw";
import "plotly.js";
import SearchResult from "@/plugins/traces/SearchResult.vue";
import searchService from "@/services/search";
import router from "../../helpers/router";

const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);

installQuasar({
  plugins: [Dialog, Notify],
});

describe("Traces Index", async () => {
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
      },
    });
    await flushPromises();
  });

  afterEach(() => {
    wrapper.unmount();
    vi.restoreAllMocks();
    // vi.clearAllMocks();
  });

  it("Should render traces search bar", async () => {
    expect(
      wrapper.get('[data-test="traces-search-bar"]').exists()
    ).toBeTruthy();
  });

  it("Should hide traces search bar when the stream list is empty", async () => {
    vi.advanceTimersByTime(500);

    wrapper.vm.searchObj.data.stream.streamLists = [];
    await flushPromises();
    expect(
      wrapper.get('[data-test="traces-search-bar"]').attributes().style
    ).toContain("display: none;");
  });

  it("Should render index list component", async () => {
    vi.advanceTimersByTime(500);

    expect(
      wrapper.get('[data-test="traces-search-index-list"]').exists()
    ).toBeTruthy();
  });

  it("Should render search result component when there are traces results", async () => {
    vi.advanceTimersByTime(500);

    expect(
      wrapper.get('[data-test="traces-search-result"]').exists()
    ).toBeTruthy();
  });

  it("Should hide search result component when there are no traces results or found", async () => {
    // Set searchObj.data.queryResults.hits to an empty array.
    // Set searchObj.data.stream.selectedStream.label to a non-empty string.
    // Render the component.
    // Expect the search result component to not be displayed.
    global.server.use(
      rest.post(
        `${store.state.API_ENDPOINT}/api/${store.state.selectedOrganization.identifier}/_search?type="traces"`,
        (req, res, ctx) => {
          return res(
            ctx.status(200),
            ctx.json({
              took: 10,
              hits: [],
              total: 0,
              from: 0,
              size: 150,
              scan_size: 0,
            })
          );
        }
      )
    );

    vi.advanceTimersByTime(500);
    await flushPromises();
    expect(
      wrapper.get('[data-test="traces-search-result"]').attributes().style
    ).toContain("display: none;");
  });

  it("Should render error message when error message is set", async () => {
    global.server.use(
      rest.post(
        `${store.state.API_ENDPOINT}/api/${store.state.selectedOrganization.identifier}/_search?type="traces"`,
        (req, res, ctx) => {
          return res(
            ctx.status(400),
            ctx.json({
              error: "Query Error",
              code: 400,
            })
          );
        }
      )
    );

    vi.advanceTimersByTime(500);
    // Set searchObj.data.errorMsg to a non-empty string.
    // Set searchObj.data.stream.selectedStream.label to a non-empty string.
    // Render the component.
    // Expect the error message to be displayed.
    await flushPromises();
    expect(
      wrapper.find('[data-test="traces-search-error-message"]').text()
    ).toBe("Query Error");
  });

  //   it("Should get logs data when scrolled in search results", async () => {
  //     const search = vi.spyOn(searchService, "search");
  //     wrapper.findComponent(SearchResult).vm.$emit("update:scroll");
  //     await vi.advanceTimersByTime(500);
  //     expect(search).toHaveBeenCalledTimes(1);
  //   });
});
