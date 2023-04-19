import { flushPromises, mount, VueWrapper } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { installQuasar } from "../helpers/install-quasar-plugin";
import { Dialog, Notify } from "quasar";
import store from "../helpers/store"
import i18n from "@/locales";
import router from "../helpers/router"
import DateTimePickerVue from "../../../components/DateTimePicker.vue";

installQuasar({
    plugins: [Dialog, Notify],
});
const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);
  
describe("DateTimePicker", async () => {
    let wrapper: any;
    try {
        beforeEach(async () => {
            vi.useFakeTimers();
            wrapper = mount(DateTimePickerVue, {
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
    } catch (error) {
        console.log("---",error)
    }
    afterEach(() => {
        wrapper.unmount();
    });

    
    it("Should relative date-time button", () => {
        expect(wrapper.find('[data-test="date-time-btn"]').exists()).toBeTruthy();
    });
    
})