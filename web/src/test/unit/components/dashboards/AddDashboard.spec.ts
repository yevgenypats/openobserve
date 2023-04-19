import { flushPromises, mount, VueWrapper } from "@vue/test-utils";
import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import AddDashboard from "../../../../components/dashboards/AddDashboard.vue"
import { installQuasar } from "../../helpers/install-quasar-plugin";
import i18n from "../../../../locales";
import { Dialog, Notify } from "quasar";
import store from '@/test/unit/helpers/store'
import dashboards from "../../../../services/dashboards";
import http from "../../../../services/http";

installQuasar({
    plugins: [Dialog, Notify],
});

const defaultValue = {
    id: '',
    name: '',
    description: ''
}

const data = {
    id: 1,
    name: 'Demo Dashboard',
    description: 'Demo Descriptions'
}

describe("AddDashboard", () => {
    console.log("inside describe");

    let wrapper: VueWrapper<any>;

    beforeEach(async () => {
        console.log("inside before each");
        try {
            wrapper = mount(AddDashboard, {
                shallow: false,
                props: {
                    modelValue: data
                },
                global: {
                    plugins: [i18n],
                    provide: {
                        store: store
                    },
                    mocks: {
                        $q: {
                        notify: vi.fn().mockImplementation(() => vi.fn()),
                        },
                    },
                },
            });

        } catch (error) {
                console.log("-err", error);
                
        }

        console.log("after mount", wrapper);

        await flushPromises();
    });
    afterEach(() => {
        // axios.post.mockReset()
        console.log("-wr",wrapper );
        
        wrapper.unmount()
    })
    it("renders the component", async () => {
        expect(wrapper.exists()).toBe(true);
    });

    it("shows 'Create Dashboard' as the default label", async () => {
        const label = wrapper.find("[data-test='create-dash']").text();
        expect(label).toBe("Create a New Dashboard");
    });

    it("disables the save button when the Dashboard name is empty", async () => {
        //without set name check button is disabled or not
        expect(wrapper.find("[data-test=add-dash]").attributes('disabled')).toBe("")
    });

    it("enable the save button when the Dashboard name is not empty", async () => {
        await wrapper.find("[data-test=dash-name]").setValue("My Dashboard2");
        expect(wrapper.find("[data-test=add-dash]").attributes('disabled')).toBeUndefined(); 
    });

    it("emits the dashboard data when the form is submitted", async () => {

        //without set name check button is disabled or not
        expect(wrapper.find("[data-test=add-dash]").attributes('disabled')).toBe("")
        //set name and check button is not disabled
        await wrapper.find("[data-test=dash-name]").setValue("My Dashboard2");
        expect(wrapper.find("[data-test=add-dash]").attributes('disabled')).toBeUndefined(); 


        // const nameInput = wrapper.get("[data-test=dash-name]")
        // nameInput.trigger("focus")

        // nameInput.trigger('keydown', {
        //     key: 'enter'
        // })

        // nameInput.trigger('keyup.enter')

        await wrapper.vm.$nextTick()

        //form onSubmit is not trigger. so, we click button click manually
        const spy = vi.spyOn(dashboards,'create').mockImplementationOnce((organization: string, dashboardID: String, data: any) => {return http().get(`/dummy/`)})
        await wrapper.vm.onSubmit()
        expect(spy).toHaveBeenCalledOnce()

        await flushPromises();


        // await wrapper.find("[data-test=dash-description]").setValue("My Dashboard");

        // await wrapper.find("[data-test=add-dash]").trigger("click")

        // expect(wrapper.find("[data-test=add-dash]").exists()).toBe(true);

        // wrapper.find("[data-test='add-dash']").

        // expect(logSpy).toHaveBeenCalledTimes(3)
        // await nextTick()
        // console.log(logSpy.mock.calls);

        // expect(logSpy).toHaveBeenCalledWith()
        // expect(wrapper.find("[data-test='add-dash']")).toBeCalled()
    });

    it("check name value is blank and submit the data that gives validation error", async () => {
      await wrapper.find("[data-test=dash-name]").setValue("My Dashboard2");
      await wrapper.find("[data-test=dash-name]").setValue("");
      console.log(await wrapper.vm.onSubmit());

      expect(await wrapper.vm.onSubmit()).toBeFalsy()
    });


});