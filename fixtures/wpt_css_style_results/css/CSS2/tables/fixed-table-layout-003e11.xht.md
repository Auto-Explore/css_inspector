# css/CSS2/tables/fixed-table-layout-003e11.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-003e11.xht"
}
```

## style[0]

```css
<![CDATA[
  div, table {font: 1.25em/1.2 serif;}

  table
  {
  border-collapse: collapse;
  table-layout: fixed;
  width: 400px;
  }

  td {padding: 0px 10px 0px 38px;}

  td#tested-cell
  {
  background-color: blue;
  border-left: orange solid 20px;
  border-right: orange solid 52px;
  color: blue;
  width: 80px;
  }

  div#reference
  {
  background-color: black;
  color: black;
  margin-left: 108px;

  /*

  "
  In the fixed table layout algorithm, the width of each column is determined as follows:

  (...) a cell in the first row with a value other than 'auto' for the 'width' property determines the width for that column. (...)
    Any remaining columns equally divide the remaining horizontal table space (minus borders or cell spacing).
  "
  Section 17.5.2.1 Fixed table layout
  http://www.w3.org/TR/CSS21/tables.html#fixed-table-layout


  Middle column width calculations
  --------------------------------

   10px : half of border-left since such border must be split with cell in 1st column
 +
   38px : padding-left of the cell in the middle column
 +
   80px : width of the cell in the middle column
 +
   10px : padding-right of the cell in the middle column
 +
   26px : half of border-right since such border must be split with cell in 3rd column
  =======
  164px : width of the middle column


  So,
      400px : table set width
    -
      164px : width of the middle column
      ======
      236px

      So, each of the 2 remaining columns must be
      half of such extra horizontal table space,
      which is 236px divided by 2 == 118px.

  1st column width calculations
  -----------------------------

    38px    : padding-left of cell in 1st column
  +
    (solve) : width of cell in 1st column
  +
    10px    : padding-right of cell in 1st column
  +
    10px    : border-right of cell in 1st column
              which is half of border-left of cell in middle column
   ======
   118px    : width of 1st column

  So, the width of the cell in first column must be exactly 60px.


  3rd column width calculations
  -----------------------------

    26px    : half of border-right of cell in middle column
              since such border must be split with the cell in 3rd column
  +
    24px    : padding-left of cell in 3rd column
  +
    (solve) : width of cell in 3rd column
  +
    24px    : padding-right of cell in 3rd column
   =======
   118px    : width of 3rd column

   So, the width of the cell in 3rd column must be exactly 44px.


  Finally, the precise horizontal point where the border-right of cell
  in first column begins to be drawn, painted is:

    24px    : padding-left of cell in 1st column
  +
    60px    : width of cell in 1st column
  +
    24px    : padding-right of cell in 1st column
  =======
   108px

  Therefore the margin-left: 108px value of the div#reference.

  */

  width: 200px;
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
