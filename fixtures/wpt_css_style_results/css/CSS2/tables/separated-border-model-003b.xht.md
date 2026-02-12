# css/CSS2/tables/separated-border-model-003b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-003b.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  background-color: black;
  border-spacing: 50px 0;
  table-layout: fixed;
  width: 500px;
  }

  table, td
  {
  border-left: black solid 11px;
  border-right: black solid 17px;
  padding: 25px 39px 25px 33px;
  }

  td {width: 100px;}

  /*

   11px (table's border-left)
   33px (table's padding-left)
   50px (left-most border-spacing)
   11px (td's border-left)
   33px (td's padding-left)
  100px (td's set content width)
   39px (td's padding-right)
   17px (td's border-right)
   50px (right-most border-spacing)
   39px (table's padding-right)
   17px (table's border-right)
  -----
  400px


  Since the set width (500px) for the XHTML/HTML table is greater than the
  sum of columns width (400px), then the extra (exceeding) space is distributed
  evenly among columns. So, here, the used width for the single td should
  be 200px, not 100px.

  */

  div
  {
  background-color: blue;
  height: 100px;
  width: 500px;
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
