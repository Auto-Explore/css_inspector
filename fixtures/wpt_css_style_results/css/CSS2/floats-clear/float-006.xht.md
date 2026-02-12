# css/CSS2/floats-clear/float-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/float-006.xht"
}
```

## style[0]

```css
<![CDATA[
   #rel-pos-containing-block {position: relative;}

   #green-overlapping-abs-pos
   {
   background-color: green;
   border-bottom: green solid 2em;
   border-top: green solid 2em;
   left: 0em;
   padding: 3em;
   position: absolute;
   width: 8em;
   z-index: auto;
   }

   #zero-height-first-float
   {
   float: left;
   width: 18em;
   /*
   or any other width number which would
   represent less than the document box width
   */
   }

   #red-overlapped-second-float
   {
   background-color: red;
   float: left;
   padding: 5em;
   width: 4em;
   }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
