import { flushPromises, mount, VueWrapper } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Dialog, Notify } from "quasar";
import i18n from "@/locales";
import { installQuasar } from "../../helpers/install-quasar-plugin";
import ExportDashboardVue from "../../../../components/dashboards/ExportDashboard.vue";
import store from "../../helpers/store";
import router from "../../helpers/router";

installQuasar({
  plugins: [Dialog, Notify],
});

const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);

describe("ExportDashboard", async () => {
  let wrapper: any;
  try {
    beforeEach(async () => {
      vi.useFakeTimers();
      wrapper = mount(ExportDashboardVue, {
        attachTo: "#app",
        props: { dashboardId: "7054033672646823936"},
        global: {
          provide: {
            store: store,
          },
          plugins: [i18n, router],
        },
      });
      await flushPromises();
    });
  } catch (error) {
    console.log("---", error);
  }
  afterEach(() => {
    wrapper.unmount();
  });

  // write a test case for button click
  it("button click", async () => {
    const button = wrapper.find('[data-test="export-btn"]');
    const spy = vi.spyOn(wrapper.vm, "downloadDashboard");
    button.trigger("click");
    await flushPromises();
    console.log("00-",wrapper.vm.downloadDashboard);
    // spy on the downloadDashboard method
    expect(wrapper.vm.downloadDashboard).toBeDefined();
    expect(spy).toHaveBeenCalledOnce();
  });

});
