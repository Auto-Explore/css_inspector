# css/CSS2/tables/fixed-table-layout-021.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-021.xht"
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
  the columns, and borders or cell spacing."
  http://www.w3.org/TR/CSS21/tables.html#fixed-table-layout

  So,

    533px : total table width
  -
     58px : total horizontal border width
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
  width: 22%;
  /* 400px multiplied by 22% = 88px */
  }

  col#third
  {
  background-color: orange;
  width: 31%;
  /* 400px multiplied by 31% = 124px */

  /*
  100% - (13% + 22% + 31%) == 34% for last column
  */
  }

  td {padding: 1px 10px;}

  td#first-cell {color: fuchsia;}

  td#second-cell {color: olive;}

  td#third-cell {color: orange;}

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
    15px : 2nd border-spacing
  =======
   111px
  */
  position: relative;
  width: 88px;
  }

  div#reference3rd
  {
  background-color: orange;
  bottom: 2.2em;
  color: orange;
  left: 214px;
  /*
    29px : border-left of table
  +
    15px : 1st border-spacing
  +
    52px : width of first column
  +
    15px : 2nd border-spacing
  +
    88px : width of second column
  +
    15px : 3rd border-spacing
  =======
   214px
  */
  position: relative;
  width: 124px;
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
