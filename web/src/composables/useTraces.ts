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

import { reactive } from "vue";
import { useLocalTraceFilterField } from "@/utils/zincutils";

const defaultObject = {
  organizationIdetifier: "",
  runQuery: false,
  loading: false,

  config: {
    splitterModel: 20,
    lastSplitterPosition: 0,
    splitterLimit: [0, 40],
  },
  meta: {
    showTraceDetails: false,
    resultGrid: {
      wrapCells: false,
      rowsPerPage: 150,
    },
    scrollInfo: {},
  },
  data: {
    query: "",
    parsedQuery: {},
    errorMsg: "",
    errorCode: 0,
    additionalErrorMsg: "",
    stream: {
      streamLists: [],
      selectedStream: { label: "", value: "" },
      selectedStreamFields: [],
      selectedFields: <string[]>[],
      filterField: "",
      addToFilter: "",
      functions: [],
    },
    resultGrid: {
      currentDateTime: new Date(),
      currentPage: 0,
      columns: <any>[],
    },
    transforms: <any>[],
    queryResults: <any>[],
    sortedQueryResults: <any>[],
    streamResults: <any>[],
    histogram: <any>{},
    editorValue: "",
    datetime: {
      tab: "relative",
      relative: {
        period: {
          label: "Minutes",
          value: "Minutes",
        },
        value: 15,
      },
      absolute: {
        date: {
          from: "",
          to: "",
        },
        startTime: "00:00",
        endTime: "23:59",
      },
    },
    searchAround: {
      indexTimestamp: 0,
      size: <number>10,
      histogramHide: false,
    },
    traceDetails: {
      traceId: "",
      spanList: [],
      loading: false,
      selectedSpanId: null,
      showSpanDetails: false,
    },
  },
};

// let searchObj = reactive(structuredClone(defaultObject));
// let localLogsObj:any = useLocalLogsObj();
// let searchObj = {};
// if (typeof localLogsObj === "object") {
// searchObj = localLogsObj.value;
// } else {
let searchObj = reactive(Object.assign({}, defaultObject));
// }

const useLogs = () => {
  const resetSearchObj = () => {
    // delete searchObj.data;
    searchObj = reactive(Object.assign({}, defaultObject));
  };
  const updatedLocalLogFilterField = (): void => {
    const identifier: string = searchObj.organizationIdetifier || "default";
    const selectedFields: any =
      useLocalTraceFilterField()?.value != null
        ? useLocalTraceFilterField()?.value
        : {};
    selectedFields[
      `${identifier}_${searchObj.data.stream.selectedStream.value}`
    ] = searchObj.data.stream.selectedFields;
    useLocalTraceFilterField(selectedFields);
  };

  return { searchObj, resetSearchObj, updatedLocalLogFilterField };
};

export default useLogs;
