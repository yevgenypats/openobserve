import { createStore } from "vuex";

const store = createStore({
  state: {
    organizationPasscode: 11,
    API_ENDPOINT: "http://localhost:8080",
    selectedOrganization: {
      label: "default Organization",
      id: 159,
      identifier: "default_organization_01",
      user_email: "example@gmail.com",
      subscription_type: "",
    },
    currentuser: {
      role: "",
      first_name: 'Test',
      last_name: 'Test'
    },
    userInfo: {
      at_hash: "QicVZWM5kDY6hOzf",
      email_verified: false,
      given_name: "example",
      picture: "",
      aud: "31ds0mr4psua0p58353l3t6j61",
      token_use: "id",
      auth_time: 1678689752,
      name: "example",
      exp: 1678776152,
      iat: 1678689753,
      family_name: "example",
      email: "example@gmail.com",
    },
    zoConfig: {
      version: "v0.2.0",
      commit_hash: "dc2b38c0f8be27bde395922d61134f09a3b4c",
      build_date: "2023-03-11T03:55:28Z",
      functions_enabled: true,
      default_fts_keys: ["log", "message", "msg", "content", "data"],
      default_functions: [
        {
          name: "match_all",
          text: "match_all('v')",
        },
        {
          name: "match_all_ignore_case",
          text: "match_all_ignore_case('v')",
        },
        {
          name: "str_match",
          text: "match_all('v')",
        },
        {
          name: "str_match_ignore_case",
          text: "match_all_ignore_case('v')",
        },
        {
          name: "re_match",
          text: "re_match(field, 'pattern')",
        },
        {
          name: "re_not_match",
          text: "re_not_match(field, 'pattern')",
        },
      ],
    },
    allDashboardList:[
      {
        "dashboardId": "7054033672646823936",
        "title": "test",
        "description": "",
        "role": "",
        "owner": "root@example.com",
        "created": "2023-04-18T10:11:23.320+00:00",
        "panels": [
            {
                "id": "Panel_ID5484210",
                "type": "bar",
                "fields": {
                    "stream": "cloudwatch-cloudtrail",
                    "stream_type": "logs",
                    "x": [
                        {
                            "label": "Timestamp",
                            "alias": "x_axis_1",
                            "column": "_timestamp",
                            "color": null,
                            "aggregationFunction": "histogram"
                        }
                    ],
                    "y": [
                        {
                            "label": "Logstream",
                            "alias": "y_axis_1",
                            "column": "logstream",
                            "color": "#5960b2",
                            "aggregationFunction": "count"
                        }
                    ],
                    "filter": []
                },
                "config": {
                    "title": "1",
                    "description": "",
                    "show_legends": true
                },
                "query": "SELECT histogram(_timestamp) as \"x_axis_1\", count(logstream) as \"y_axis_1\"  FROM \"cloudwatch-cloudtrail\"  GROUP BY \"x_axis_1\" ORDER BY \"x_axis_1\"",
                "customQuery": false
            }
        ],
        "layouts": [
            {
                "x": 0,
                "y": 0,
                "w": 12,
                "h": 13,
                "i": 1,
                "panelId": "Panel_ID5484210",
                "static": false
            }
        ]
      }
    ]
  },
});

export default store;
