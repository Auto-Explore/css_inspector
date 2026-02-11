# css/CSS2/tables/fixed-table-layout-017.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-017.xht"
}
```

## style[0]

```css
<![CDATA[
  table, div {font: 1.25em/1 Ahem;}

  table
  {
  border-collapse: separate;
  border-spacing: 4px;
  table-layout: fixed;
  width: 420px;
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
  3 horizontal border-spacing separate the start
  of table box and the start of 3rd column.
  The first 2 columns should each be 80px exactly
  since "Any remaining columns equally divide the
  remaining horizontal table space (minus [table] borders or
  cell spacing)." So:

     0px : table border-left
  +
     4px : 1st border-spacing
  +
    80px : 1st column : (420 - 20) mult by (60% divided by 3)
  +
     4px : 2nd border-spacing
  +
    80px : 2nd column : (420 - 20) mult by (60% divided by 3)
  +
     4px: 3rd border-spacing
  ========
   172px
  */

  position: relative;
  width: 160px;

  /*

    420px : total width of table
  -
     20px : 5 times horizontal border-spacing
  -
      0px : total of horizontal table borders
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
