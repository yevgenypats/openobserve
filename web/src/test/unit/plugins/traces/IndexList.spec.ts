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
import IndexList from "@/plugins/traces/IndexList.vue";
import i18n from "@/locales";
import store from "../../helpers/store";
import "plotly.js";
import SearchResult from "@/plugins/traces/SearchResult.vue";
import router from "../../helpers/router";

const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);

installQuasar({
  plugins: [Dialog, Notify],
});

describe("Trace Index List", async () => {
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
        stubs: {},
      },
    });
    await flushPromises();
    vi.advanceTimersByTime(2000);
    await flushPromises();
  });

  afterEach(() => {
    wrapper.unmount();
    vi.restoreAllMocks();
    // vi.clearAllMocks();
  });

  it("Should show search field input", async () => {
    expect(
      wrapper
        .findComponent(IndexList)
        .find('[data-test="trace-search-index-list-field-search-input"]')
        .exists()
    ).toBeTruthy();
  });
  it("Should show fields table", async () => {
    expect(
      wrapper
        .findComponent(IndexList)
        .find('[data-test="trace-search-index-list-fields-table"]')
        .exists()
    ).toBeTruthy();
  });

  it("Should add field to query when clicked on filter field", async () => {
    const field = "trace_id";
    await wrapper
      .findComponent(IndexList)
      .find(`[data-test="trace-search-index-list-filter-${field}-field-btn"]`)
      .trigger("click");
    expect(wrapper.vm.searchObj.data.query).toContain(field);
  });

  it("Should filter fields when searched for specific field", async () => {
    const field = "trace_id";
    // As we don't display _timestamp field checking it dont contains _timestamp
    expect(wrapper.findComponent(IndexList).text()).not.toContain("_timestamp");
    await wrapper
      .findComponent(IndexList)
      .find(`[data-test="trace-search-index-list-field-search-input"]`)
      .setValue(field);
    expect(wrapper.findComponent(IndexList).text()).toContain(field);
  });
});
