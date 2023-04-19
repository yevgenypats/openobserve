export default {
    dashboards: {
      get: {
        list: [
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
          ],
      },
    }
  };
  