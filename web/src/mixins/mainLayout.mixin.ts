import { onActivated, onMounted } from "vue";
import { useStore } from "vuex";
import organizationService from "@/services/organizations";
import { getImageURL } from "@/utils/zincutils";

const MainLayoutOpenSourceMixin = {
  setup() {
    const store: any = useStore();

    /**
     * Add function menu in left navigation
     * @param linksList
     * @returns linksList.value
     */
    const leftNavigationLinks = (linksList: any, t: any) => {
      linksList.value.splice(5, 0, {
        title: t("menu.function")+"1",
        icon: "img:" + getImageURL("images/common/function.svg"),
        link: "/functions",
      });

      return linksList.value;
    };

    /**
     * Get default organization
     */
    const getDefaultOrganization = async () => {
      await organizationService
        .os_list(0, 1000, "id", false, "", "default")
        .then((res: any) => {
          store.dispatch("setOrganizations", res.data.data);
        });
    };

    onActivated(() => {
      getDefaultOrganization();
    });

    onMounted(() => {
      getDefaultOrganization();
    });

    return {
      leftNavigationLinks,
      getDefaultOrganization,
    };
  },
};

export default MainLayoutOpenSourceMixin;
