# css/CSS2/tables/separated-border-model-004d.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/separated-border-model-004d.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table-width-set
  {
  background-color: green;
  border-left: white solid 32px;
  border-top: white solid 8px;
  border-spacing: 17px 0;
  display: table;
  padding: 0px;
  table-layout: fixed;
  width: 100px;
  /*
  The set width (100px) of this CSS table is larger than sum of columns plus
  border spacing

   17px   (left-most border-spacing)
 +
   16px   (div#td's width)
 +
   17px   (right-most border-spacing)
  ======
   50px : sum of columns plus border spacing

  100px (width of table)
 -
   50px (sum of columns plus border spacing)
  ======
   50px : such extra (exceeding) 50px width will be distributed
   over the columns, therefore given to the unique cell of such
   div#table-width-set .
  */
  }

  div.tr {display: table-row;}

  div.td
  {
  display: table-cell;
  height: 100px;
  width: 16px;
  }

  div#table-sum-of-columns
  {
  background-color: green;
  border-left: white solid 32px;
  border-top: white solid 8px;
  border-spacing: 42px 0;
  display: table;
  padding: 0px;
  table-layout: fixed;
  width: 9px;

  /*
  The width of a CSS table is given by the greater of the value of
  the 'width' property as set for the table element and the sum of the columns'
  width (plus border spacing):

  max(set width, sum of columns width plus border spacing)

  The set width (9px) is smaller than sum of columns (plus border spacing)

   42px   (left-most border-spacing)
 +
   16px   (div.td's width)
 +
   42px   (right-most border-spacing)
  ======
  100px : sum of columns plus border spacing
  */
  }

  div#reference
  {
  background-color: green;
  height: 100px;
  left: 32px; /* equal to the tables' border-left */
  position: relative;
  top: 8px;  /* equal to the tables' border-top */
  width: 100px;
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
