# css/CSS2/tables/fixed-table-layout-023.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-023.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  margin: 8px;
  width: 640px;
  }

  table, div {font: 1.25em/1 Ahem;}

  table
  {
  border: white solid;
  border-width: 0px 11px; /* horizontal table border width is 22px total */
  border-collapse: separate;
  border-spacing: 18px 4px; /* horizontal border-spacing width is 90px total */
  table-layout: fixed;
  width: 80%; /*  640px mult by 80% == 512px */

  /*
  "With this (fast) algorithm, the horizontal layout of
  the table does not depend on the contents of the cells;
  it only depends on the table's width, the width of
  the columns, and [table] borders or cell spacing."

  So,

    512px : total table width
  -
     90px : total horizontal border-spacing width
  -
     22px : total horizontal border-spacing width
  ========
    400px : total to split among the 4 columns
  */

  }

  col#first
  {
  background-color: fuchsia;
  width: 13%;
  /* 400px multiplied by 13% = 52px */
  }

  col#second
  {
  background-color: olive;
  width: 100px;
  }

  col#third
  {
  background-color: orange;
  width: 31%;
  /* 400px multiplied by 31% = 124px */

  /*
  400px - (52px + 100px + 124px) = 124px for last column
  */
  }

  col#fourth {background-color: lime;}

  td {padding: 1px 10px;}

  td#first-cell {color: fuchsia;}

  td#second-cell {color: olive;}

  td#third-cell {color: orange;}

  td#fourth-cell {color: lime;}

  div {padding: 1px 0px;}

  div#reference1st
  {
  background-color: fuchsia;
  color: fuchsia;
  left: 29px; /* 11px border-left of table + 18px border-spacing == 29px */
  position: relative;
  width: 52px;
  }

  div#reference2nd
  {
  background-color: olive;
  bottom: 1.1em;
  color: olive;
  left: 99px;
  /*
    11px : border-left of table
  +
    18px : 1st border-spacing
  +
    52px : width of first column
  +
    18px : 2nd border-spacing
  =======
    99px
  */
  position: relative;
  width: 100px;
  }

  div#reference3rd
  {
  background-color: orange;
  bottom: 2.2em;
  color: orange;
  left: 217px;
  /*
    11px : border-left of table
  +
    18px : 1st border-spacing
  +
    52px : width of first column
  +
    18px : 2nd border-spacing
  +
   100px : width of second column
  +
    18px : 3rd border-spacing
  =======
   217px
  */
  position: relative;
  width: 124px;
  }

  div#reference4th
  {
  background-color: lime;
  bottom: 3.3em;
  color: lime;
  left: 359px;
  /*
    11px : border-left of table
  +
    18px : 1st border-spacing
  +
    52px : width of first column
  +
    18px : 2nd border-spacing
  +
   100px : width of second column
  +
    18px : 3rd border-spacing
  +
   124px : width of third column
  +
    18px : 4th border-spacing
  =======
   359px
  */
  position: relative;
  width: 124px;
  }
  ]]>
```

```json
{
  "errors": 6,
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
