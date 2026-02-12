# css/CSS2/tables/separated-border-model-004a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-004a.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  left: auto;
  position: absolute;
  top: auto;
  width: 100px;
  z-index: -1;
  }

  div#table
  {
  background-color: green;
  border-spacing: 2px 5px;
  color: green;
  display: table;
  font: 20px/1 serif;
  padding-top: 5px;
  table-layout: fixed;
  width: 100px;
  /*
  The set width (100px) is larger than sum of columns' width plus cell spacing

    2px : left-most border-spacing
 +
   40px : div.td's width: leftmost cell width in first row)
 +
    2px : middle border-spacing
 +
   40px : div.td's width: rightmost cell width in first row)
 +
    2px : right-most border-spacing
  ======
   86px : sum of columns plus cell spacing


  100px : set width of table
 -
   86px : sum of columns plus cell spacing
  ======
   14px : such extra (exceeding) 14px width will be distributed over
    the columns.
  */
  }

  div.tr {display: table-row;}

  div.td {display: table-cell;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
