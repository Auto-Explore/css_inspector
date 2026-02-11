# css/CSS2/zindex/z-index-stack-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-stack-003.xht"
}
```

## style[0]

```css
<![CDATA[
   #rel-pos-containing-block {position: relative;}

   #yellow-overlapping-abs-pos
   {
   background-color: yellow;
   height: 100px;
   left: 60px;
   position: absolute;
   top: 60px;
   width: 100px;
   z-index: auto;
   }

   #blue-first-float
   {
   background-color: blue;
   float: left;
   height: 100px;
   width: 100px;
   }

   #orange-second-float
   {
   background-color: orange;
   float: left;
   height: 100px;
   margin-left: -70px;
   margin-top: 30px;
   width: 100px;
   }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
