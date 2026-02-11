# css/CSS2/tables/fixed-table-layout-018.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-018.xht"
}
```

## style[0]

```css
<![CDATA[
  table, div {font: 1.25em/1 Ahem;}

  table
  {
  border: solid white;
  border-width: 0px 6px;
  border-collapse: separate;
  border-spacing: 2px 4px;
  table-layout: fixed;
  width: 422px;
  }

  col#test
  {
  background-color: orange;
  width: 40%;
  }

  td#third-cell {color: orange;}

  div, td {padding: 1px 0px;}

  div#reference
  {
  background-color: blue;
  color: blue;
  left: 172px;
  /*
  3 horizontal border-spacing and the table
  border-left separate the start
  of table box and the start of 3rd column.
  The first 2 columns should each be 80px exactly
  since "Any remaining columns equally divide the
  remaining horizontal table space (minus [table] borders or
  cell spacing)." So:

     6px : table border-left
  +
     2px : 1st border-spacing
  +
    80px : 1st column : (422 - 10 - 12) mult by (60% divided by 3)
  +
     2px : 2nd border-spacing
  +
    80px : 2nd column : (422 - 10 - 12) mult by (60% divided by 3)
  +
     2px : 3rd border-spacing
  =========
   172px
  */

  position: relative;
  width: 160px;

  /*

    422px : total width of table
  -
     12px : total width of horizontal borders of table
  -
     10px : 5 times horizontal border-spacing
   ========
    400px
   mult by
     40%
   ========
    160px

  */
  }
  ]]>
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
