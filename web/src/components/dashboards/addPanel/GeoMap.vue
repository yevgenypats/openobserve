<template>
    <div style="height:100%; width:100%;">
        <div ref="plotRef"></div>
    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, reactive, ref } from 'vue';
import Plotly from "plotly.js";

export default defineComponent({
    name: "GeoMap",

    setup() {

       const map= [
            {
            "location": "Belarus",
            "amount": 17.5
            },
            {
            "location": "Moldova",
            "amount": 16.8
            },
            {
            "location": "Lithuania",
            "amount": 15.4
            },
            {
            "location": "Russia",
            "amount": 15.1
            },
            {
            "location": "Romania",
            "amount": 14.4
            },
            {
            "location": "Ukraine",
            "amount": 13.9
            },
            {
            "location": "Andorra",
            "amount": 13.8
            },
            {
            "location": "Hungary",
            "amount": 13.3
            },
            {
            "location": "Czech Republic",
            "amount": 13
            },
            {
            "location": "Slovakia",
            "amount": 13
            },
            {
            "location": "Portugal",
            "amount": 12.9
            },
            {
            "location": "Serbia",
            "amount": 12.6
            },
            {
            "location": "Grenada",
            "amount": 12.5
            },
            {
            "location": "Poland",
            "amount": 12.5
            },
            {
            "location": "Latvia",
            "amount": 12.3
            },
            {
            "location": "Finland",
            "amount": 12.3
            },
            {
            "location": "South Korea",
            "amount": 12.3
            },
            {
            "location": "France",
            "amount": 12.2
            },
            {
            "location": "Australia",
            "amount": 12.2
            },
            {
            "location": "Croatia",
            "amount": 12.2
            },
            {
            "location": "Ireland",
            "amount": 11.9
            },
            {
            "location": "Luxembourg",
            "amount": 11.9
            },
            {
            "location": "Germany",
            "amount": 11.8
            },
            {
            "location": "Slovenia",
            "amount": 11.6
            },
            {
            "location": "United Kingdom",
            "amount": 11.6
            },
            {
            "location": "Denmark",
            "amount": 11.4
            },
            {
            "location": "Bulgaria",
            "amount": 11.4
            },
            {
            "location": "Spain",
            "amount": 11.2
            },
            {
            "location": "Belgium",
            "amount": 11
            },
            {
            "location": "South Africa",
            "amount": 11
            },
            {
            "location": "New Zealand",
            "amount": 10.9
            },
            {
            "location": "Gabon",
            "amount": 10.9
            },
            {
            "location": "Namibia",
            "amount": 10.8
            },
            {
            "location": "Switzerland",
            "amount": 10.7
            },
            {
            "location": "Saint Lucia",
            "amount": 10.4
            },
            {
            "location": "Austria",
            "amount": 10.3
            },
            {
            "location": "Estonia",
            "amount": 10.3
            },
            {
            "location": "Greece",
            "amount": 10.3
            },
            {
            "location": "Kazakhstan",
            "amount": 10.3
            },
            {
            "location": "Canada",
            "amount": 10.2
            },
            {
            "location": "Nigeria",
            "amount": 10.1
            },
            {
            "location": "Netherlands",
            "amount": 9.9
            },
            {
            "location": "Uganda",
            "amount": 9.8
            },
            {
            "location": "Rwanda",
            "amount": 9.8
            },
            {
            "location": "Chile",
            "amount": 9.6
            },
            {
            "location": "Argentina",
            "amount": 9.3
            },
            {
            "location": "Burundi",
            "amount": 9.3
            },
            {
            "location": "United States",
            "amount": 9.2
            },
            {
            "location": "Cyprus",
            "amount": 9.2
            },
            {
            "location": "Sweden",
            "amount": 9.2
            },
            {
            "location": "Venezuela",
            "amount": 8.9
            },
            {
            "location": "Paraguay",
            "amount": 8.8
            },
            {
            "location": "Brazil",
            "amount": 8.7
            },
            {
            "location": "Sierra Leone",
            "amount": 8.7
            },
            {
            "location": "Montenegro",
            "amount": 8.7
            },
            {
            "location": "Belize",
            "amount": 8.5
            },
            {
            "location": "Cameroon",
            "amount": 8.4
            },
            {
            "location": "Botswana",
            "amount": 8.4
            },
            {
            "location": "Saint Kitts and Nevis",
            "amount": 8.2
            },
            {
            "location": "Guyana",
            "amount": 8.1
            },
            {
            "location": "Peru",
            "amount": 8.1
            },
            {
            "location": "Panama",
            "amount": 8
            },
            {
            "location": "Niue",
            "amount": 8
            },
            {
            "location": "Palau",
            "amount": 7.9
            },
            {
            "location": "Norway",
            "amount": 7.7
            },
            {
            "location": "Tanzania",
            "amount": 7.7
            },
            {
            "location": "Georgia",
            "amount": 7.7
            },
            {
            "location": "Uruguay",
            "amount": 7.6
            },
            {
            "location": "Angola",
            "amount": 7.5
            },
            {
            "location": "Laos",
            "amount": 7.3
            },
            {
            "location": "Japan",
            "amount": 7.2
            },
            {
            "location": "Mexico",
            "amount": 7.2
            },
            {
            "location": "Ecuador",
            "amount": 7.2
            },
            {
            "location": "Dominica",
            "amount": 7.1
            },
            {
            "location": "Iceland",
            "amount": 7.1
            },
            {
            "location": "Thailand",
            "amount": 7.1
            },
            {
            "location": "Bosnia and Herzegovina",
            "amount": 7.1
            },
            {
            "location": "Sao Tome and Principe",
            "amount": 7.1
            },
            {
            "location": "Malta",
            "amount": 7
            },
            {
            "location": "Albania",
            "amount": 7
            },
            {
            "location": "Bahamas",
            "amount": 6.9
            },
            {
            "location": "Dominican Republic",
            "amount": 6.9
            },
            {
            "location": "Mongolia",
            "amount": 6.9
            },
            {
            "location": "Cape Verde",
            "amount": 6.9
            },
            {
            "location": "Barbados",
            "amount": 6.8
            },
            {
            "location": "Burkina Faso",
            "amount": 6.8
            },
            {
            "location": "Italy",
            "amount": 6.7
            },
            {
            "location": "Trinidad and Tobago",
            "amount": 6.7
            },
            {
            "location": "China",
            "amount": 6.7
            },
            {
            "location": "Macedonia",
            "amount": 6.7
            },
            {
            "location": "Saint Vincent and the Grenadines",
            "amount": 6.6
            },
            {
            "location": "Equatorial Guinea",
            "amount": 6.6
            },
            {
            "location": "Suriname",
            "amount": 6.6
            },
            {
            "location": "Vietnam",
            "amount": 6.6
            },
            {
            "location": "Lesotho",
            "amount": 6.5
            },
            {
            "location": "Haiti",
            "amount": 6.4
            },
            {
            "location": "Cook Islands",
            "amount": 6.4
            },
            {
            "location": "Colombia",
            "amount": 6.2
            },
            {
            "location": "Ivory Coast",
            "amount": 6
            },
            {
            "location": "Bolivia",
            "amount": 5.9
            },
            {
            "location": "Swaziland",
            "amount": 5.7
            },
            {
            "location": "Zimbabwe",
            "amount": 5.7
            },
            {
            "location": "Seychelles",
            "amount": 5.6
            },
            {
            "location": "Cambodia",
            "amount": 5.5
            },
            {
            "location": "Puerto Rico",
            "amount": 5.4
            },
            {
            "location": "Netherlands Antilles",
            "amount": 5.4
            },
            {
            "location": "Philippines",
            "amount": 5.4
            },
            {
            "location": "Costa Rica",
            "amount": 5.4
            },
            {
            "location": "Armenia",
            "amount": 5.3
            },
            {
            "location": "Cuba",
            "amount": 5.2
            },
            {
            "location": "Nicaragua",
            "amount": 5
            },
            {
            "location": "Jamaica",
            "amount": 4.9
            },
            {
            "location": "Ghana",
            "amount": 4.8
            },
            {
            "location": "Liberia",
            "amount": 4.7
            },
            {
            "location": "Uzbekistan",
            "amount": 4.6
            },
            {
            "location": "Chad",
            "amount": 4.4
            },
            {
            "location": "United Arab Emirates",
            "amount": 4.3
            },
            {
            "location": "Kyrgyzstan",
            "amount": 4.3
            },
            {
            "location": "India",
            "amount": 4.3
            },
            {
            "location": "Turkmenistan",
            "amount": 4.3
            },
            {
            "location": "Kenya",
            "amount": 4.3
            },
            {
            "location": "Ethiopia",
            "amount": 4.2
            },
            {
            "location": "Honduras",
            "amount": 4
            },
            {
            "location": "Guinea-Bissau",
            "amount": 4
            },
            {
            "location": "Zambia",
            "amount": 4
            },
            {
            "location": "Republic of the Congo",
            "amount": 3.9
            },
            {
            "location": "Guatemala",
            "amount": 3.8
            },
            {
            "location": "Central African Republic",
            "amount": 3.8
            },
            {
            "location": "North Korea",
            "amount": 3.7
            },
            {
            "location": "Sri Lanka",
            "amount": 3.7
            },
            {
            "location": "Mauritius",
            "amount": 3.6
            },
            {
            "location": "Samoa",
            "amount": 3.6
            },
            {
            "location": "Democratic Republic of the Congo",
            "amount": 3.6
            },
            {
            "location": "Nauru",
            "amount": 3.5
            },
            {
            "location": "Gambia",
            "amount": 3.4
            },
            {
            "location": "Federated States of Micronesia",
            "amount": 3.3
            },
            {
            "location": "El Salvador",
            "amount": 3.2
            },
            {
            "location": "Fiji",
            "amount": 3
            },
            {
            "location": "Papua New Guinea",
            "amount": 3
            },
            {
            "location": "Kiribati",
            "amount": 3
            },
            {
            "location": "Tajikistan",
            "amount": 2.8
            },
            {
            "location": "Israel",
            "amount": 2.8
            },
            {
            "location": "Sudan",
            "amount": 2.7
            },
            {
            "location": "Malawi",
            "amount": 2.5
            },
            {
            "location": "Lebanon",
            "amount": 2.4
            },
            {
            "location": "Azerbaijan",
            "amount": 2.3
            },
            {
            "location": "Mozambique",
            "amount": 2.3
            },
            {
            "location": "Togo",
            "amount": 2.3
            },
            {
            "location": "Nepal",
            "amount": 2.2
            },
            {
            "location": "Brunei",
            "amount": 2.1
            },
            {
            "location": "Benin",
            "amount": 2.1
            },
            {
            "location": "Singapore",
            "amount": 2
            },
            {
            "location": "Turkey",
            "amount": 2
            },
            {
            "location": "Madagascar",
            "amount": 1.8
            },
            {
            "location": "Solomon Islands",
            "amount": 1.7
            },
            {
            "location": "Tonga",
            "amount": 1.6
            },
            {
            "location": "Tunisia",
            "amount": 1.5
            },
            {
            "location": "Tuvalu",
            "amount": 1.5
            },
            {
            "location": "Qatar",
            "amount": 1.5
            },
            {
            "location": "Vanuatu",
            "amount": 1.4
            },
            {
            "location": "Djibouti",
            "amount": 1.3
            },
            {
            "location": "Malaysia",
            "amount": 1.3
            },
            {
            "location": "Syria",
            "amount": 1.2
            },
            {
            "location": "Maldives",
            "amount": 1.2
            },
            {
            "location": "Mali",
            "amount": 1.1
            },
            {
            "location": "Eritrea",
            "amount": 1.1
            },
            {
            "location": "Algeria",
            "amount": 1
            },
            {
            "location": "Iran",
            "amount": 1
            },
            {
            "location": "Oman",
            "amount": 0.9
            },
            {
            "location": "Brunei",
            "amount": 0.9
            },
            {
            "location": "Morocco",
            "amount": 0.9
            },
            {
            "location": "Jordan",
            "amount": 0.7
            },
            {
            "location": "Bhutan",
            "amount": 0.7
            },
            {
            "location": "Guinea",
            "amount": 0.7
            },
            {
            "location": "Burma",
            "amount": 0.7
            },
            {
            "location": "Afghanistan",
            "amount": 0.7
            },
            {
            "location": "Senegal",
            "amount": 0.6
            },
            {
            "location": "Indonesia",
            "amount": 0.6
            },
            {
            "location": "Timor-Leste",
            "amount": 0.6
            },
            {
            "location": "Iraq",
            "amount": 0.5
            },
            {
            "location": "Somalia",
            "amount": 0.5
            },
            {
            "location": "Egypt",
            "amount": 0.4
            },
            {
            "location": "Niger",
            "amount": 0.3
            },
            {
            "location": "Yemen",
            "amount": 0.3
            },
            {
            "location": "Comoros",
            "amount": 0.2
            },
            {
            "location": "Saudi Arabia",
            "amount": 0.2
            },
            {
            "location": "Bangladesh",
            "amount": 0.2
            },
            {
            "location": "Kuwait",
            "amount": 0.1
            },
            {
            "location": "Libya",
            "amount": 0.1
            },
            {
            "location": "Mauritania",
            "amount": 0.1
            },
            {
            "location": "Pakistan",
            "amount": 0.1
            }
            ]
        

        const trace:any = [{
            type: 'choropleth',
            locationmode: 'country names',
            locations: map.map((it: any)=> it.location),
            z: map.map((it: any)=> it.amount),
            text: map.map((it: any)=> it.location),
            autocolorscale: true
        }]
        
        const data:any = [{
            type: 'choropleth',
            locationmode: 'country names',

            locations:  map.map((it: any)=> it.location),
            z: map.map((it: any)=> it.amount),
            text: map.map((it: any)=> it.location),
            colorscale: [
                [0,'rgb(5, 10, 172)'],[0.35,'rgb(40, 60, 190)'],
                [0.5,'rgb(70, 100, 245)'], [0.6,'rgb(90, 120, 245)'],
                [0.7,'rgb(106, 137, 247)'],[1,'rgb(220, 220, 220)']],
            autocolorscale: false,
            reversescale: true,
            marker: {
                line: {
                    color: 'rgb(180,180,180)',
                    width: 0.5
                }
            },
            tick0: 0,
            zmin: 0,
            dtick: 1000,
            colorbar: {
                autotic: false,
                tickprefix: '$',
                title: 'GDP<br>Billions US$'
            }
        }];
        const plotRef: any = ref(null);
        var layout = {
          title: '2014 Global GDP<br>Source: <a href="https://www.cia.gov/library/publications/the-world-factbook/fields/2195.html"> CIA World Factbook</a>',
          geo:{
              showframe: false,
              showcoastlines: false,
              projection:{
                  type: 'mercator'
              }
          }
      };

onMounted(()=>{
    Plotly.newPlot(plotRef.value, data, layout, {showLink: false});
    
})
        
        return {
            map,
            plotRef,
            trace
        }
    }
})
</script>

<style lang="scss" scoped>

</style>