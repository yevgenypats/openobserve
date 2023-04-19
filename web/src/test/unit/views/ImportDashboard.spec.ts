import { Dashboards } from '@/views/Dashboards/Dashboards.vue';
import { flushPromises, mount, VueWrapper } from "@vue/test-utils";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { Dialog, Notify } from "quasar";
import i18n from "@/locales";
import store from "../helpers/store";
import router from "../helpers/router";
import { installQuasar } from "../helpers/install-quasar-plugin";
import ImportDashboardVue from "../../../views/Dashboards/ImportDashboard.vue";
import dashboards from '../mockData/dashboards';

installQuasar({
  plugins: [Dialog, Notify],
});

const node = document.createElement("div");
node.setAttribute("id", "app");
document.body.appendChild(node);

   // Mock dummy JSON data
   const dummyJsonData = {
    fileName: 'myFileComponent.json',
    content: { name: 'John', age: 30 }
  };

describe("ImportDashboard", async () => {
  let wrapper: any;
  try {
    beforeEach(async () => {
      vi.useFakeTimers();
      wrapper = mount(ImportDashboardVue, {
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
  it("import button disabled without file", async () => {
    // chose json file to import dashboard
     const button = wrapper.find('[data-test="import-btn"]');
    expect(wrapper.find('[data-test="import-btn"]').attributes("disabled")).toBe('');


  });

  it("Chose json file and import", async () => {
    // chose json file to import dashboard
    const spy = vi.spyOn(wrapper.vm, "importFile");
    const input = wrapper.find('[data-test="import-file"]').trigger('click', dummyJsonData);
    
    // Check that the imported file is the JSON file
    const importedFileContent = wrapper.findComponent(ImportDashboardVue).props('fileContent');
    expect(importedFileContent).toEqual(dummyJsonData.content);

    // const str = JSON.stringify(dashboards.dashboards.get.list[0]);
    // const blob = new Blob([str]);
    // const file = new File([blob], 'dashboard.json', {
    //   type: 'application/JSON',
    // });
    // File.prototype.text = vi.fn().mockResolvedValueOnce(str);

    // quasar q-file test select file for testing
    // const fileList = createFileList(createFile());
    // input.element.files = fileList;

     // Check that the import button was only called once

    // set value of a ref variable on a mounted vue component for testing



    

    await flushPromises();

    const button = wrapper.find('[data-test="import-btn"]');
    button.trigger("click");
    await flushPromises();
    // spy on the downloadDashboard method
    expect(wrapper.vm.importFile).toBeDefined();
    expect(spy).toHaveBeenCalledOnce();
  })


  // write a test case for upload file in input field


});

const createFile = (size = 44320, name = 'dashboard.json', type = 'application/json') =>
  new File([new ArrayBuffer(size)], name , {
    type: type,
  });

  const createFileList = (file: any) => {
    const fileList = new FileList();
    fileList[0] = file;
    fileList.item = index => fileList[index]; // override method functionality
    return fileList;
  }
