# css/CSS2/tables/separated-border-model-004c.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-004c.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red
  {
  background-color: red;
  height: 100px;
  position: absolute;
  width: 100px;
  z-index: -1;
  }

  div#overlapping-green-table
  {
  background-color: green;
  border-collapse: separate;
  border-spacing: 1em;
  color: green;
  display: table;
  font: 20px/1 serif;
  table-layout: fixed;
  width: 70px;
  /*
  The width of a CSS table is given by the greater of the value of
  the 'width' property as set for the CSS table and the sum of the columns
  width (plus border spacing):

  max(set width, sum of columns width plus border spacing)

  The set width (70px) is smaller than sum of columns width plus border
  spacing

    20px  : left-most border-spacing
 +
    20px  : div.td's width
 +
    20px  : middle border-spacing between 1st and 2nd cell
 +
    20px  : div.td's width
 +
    20px  : right-most border-spacing
  =======
   100px  : sum of columns plus border spacing
  */
  }

  div.tr {display: table-row;}

  div.td
  {
  display: table-cell;
  width: 1em;
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
