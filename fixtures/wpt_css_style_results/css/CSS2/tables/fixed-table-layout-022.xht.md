# css/CSS2/tables/fixed-table-layout-022.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-022.xht"
}
```

## style[0]

```css
<![CDATA[
  table, div {font: 1.25em/1 Ahem;}

  table
  {
  border: white solid;
  border-width: 0px 29px; /* horizontal table border width is 58px total */
  border-collapse: separate;
  border-spacing: 15px 4px; /* horizontal border-spacing width is 75px total */
  table-layout: fixed;
  width: 533px;

  /*
  "With this (fast) algorithm, the horizontal layout of
  the table does not depend on the contents of the cells;
  it only depends on the table's width, the width of
  the columns, and [table] borders or cell spacing."

  So,

    533px : total table width
  -
     58px : total horizontal border-spacing width
  -
     75px : total horizontal border-spacing width
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
  300px - (52px + 124px) = 124px for last column
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
  left: 44px; /* 29px border-left of table + 15px border-spacing == 44px */
  position: relative;
  width: 52px;
  }

  div#reference2nd
  {
  background-color: olive;
  bottom: 1.1em;
  color: olive;
  left: 111px;
  /*
    29px : border-left of table
  +
    15px : 1st border-spacing
  +
    52px : width of first column
  +
    15px border-spacing
  ======
   111px
  */
  position: relative;
  width: 100px;
  }

  div#reference3rd
  {
  background-color: orange;
  bottom: 2.2em;
  color: orange;
  left: 226px;
  /*
    29px : border-left of table
  +
    15px : 1st border-spacing
  +
    52px : width of first column
  +
    15px : 2nd border-spacing
  +
   100px : width of second column
  +
    15px : 3rd border-spacing
  =======
   226px
  */
  position: relative;
  width: 124px;
  }

  div#reference4th
  {
  background-color: lime;
  bottom: 3.3em;
  color: lime;
  left: 365px;
  /*
    29px : border-left of table
  +
    15px : 1st border-spacing
  +
    52px : width of first column
  +
    15px : 2nd border-spacing
  +
   100px : width of second column
  +
    15px : 3rd border-spacing
  +
   124px : width of third column
  +
    15px : 4th border-spacing
  =======
   365px
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
