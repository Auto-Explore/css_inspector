# css/css-exclusions/css3-exclusions/exclusions-wrap-flow-04.xht

```json
{
  "format_version": 3,
  "file": "css/css-exclusions/css3-exclusions/exclusions-wrap-flow-04.xht"
}
```

## style[0]

```css

         /*<![CDATA[*/

     #containing-block {
         position: relative;
         width: 400px;
         background: red;
         font-size: 0px; /* collapses the 'strut' on each line, see CSS 2.1 section 10.8.1 */
     }

     #top-fragment, #left-fragment, #right-fragment, #bottom-fragment-a, #bottom-fragment-b {
         display: inline-block;
         background: black;
     }

     #top-fragment {
         width: 400px;
         height: 150px;
     }

     #bottom-fragment-a {
         width: 150px;
         height:150px;
     }

     #bottom-fragment-b {
         width: 250px;
         height: 150px;
     }

     #exclusion {
         position: absolute;
         wrap-flow: clear;
         top: 150px;
         left: 150px;
         width: 100px;
         height: 100px;
         background: #0f0;
     }

     #clear-area-a, #clear-area-b {
         position: absolute;
         background: #080;
         width: 150px;
         height: 100px;
         top: 150px;
     }

     #clear-area-a {
         left: 0px;
     }

     #clear-area-b {
         left: 250px;
     }

     /*]]>*/
     
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “wrap-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
