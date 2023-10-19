// Copyright 2023 Zinc Labs Inc.

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//      http:www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

import { useStore } from "vuex";
import { datadogRum } from "@datadog/browser-rum";
import { datadogLogs } from "@datadog/browser-logs";
import "@datadog/browser-rum/bundle/datadog-rum";
import config from "@/aws-exports";

const userActivityTracking = () => {
  const store = useStore();
  const initializeTracking = () => {
    datadogRum.init({
      applicationId: "04d3b6de-51c1-49d5-a693-504293e3d36f",
      clientToken: "pub7021787ca082769936566dbefdd0d13c",
      site: "us5.datadoghq.com",
      service: "openobserve",
      env: config.environment,
      sessionSampleRate: 100,
      premiumSampleRate: 100,
      trackUserInteractions: true,
      trackResources: true,
      trackLongTasks: true,
      defaultPrivacyLevel: "mask-user-input",
    });

    datadogLogs.init({
      clientToken: "pub7021787ca082769936566dbefdd0d13c",
      site: "us5.datadoghq.com",
      forwardErrorsToLogs: true,
      sessionSampleRate: 100,
      service: "openobserve",
    });

    datadogRum.startSessionReplayRecording();
  };

  const setUser = (userInfo: any) => {
    datadogRum.setUser({
      name: userInfo.given_name + " " + userInfo.family_name,
      email: userInfo.email,
    });
  };

  const setUserProperty = (key: string, value: any) => {
    datadogRum.removeUserProperty(key);
    datadogRum.setUserProperty(key, value);
  };

  const setGlobalContext = (obj: {}) => {
    datadogRum.setGlobalContext(obj);
  };

  const setGlobalContextProperty = (key: string, value: any) => {
    datadogRum.removeGlobalContextProperty(key);
    datadogRum.setGlobalContextProperty(key, value);
  };

  const catchError = (error: any) => {
    datadogRum.addError(error, {
      pageURL: window.location.href,
    });
  };

  return {
    setUser,
    initializeTracking,
    setUserProperty,
    setGlobalContextProperty,
    setGlobalContext,
    catchError,
  };
};

export default userActivityTracking;
