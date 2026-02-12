# css/CSS2/tables/fixed-table-layout-002a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-002a.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 1em/1.25 serif;
  margin: 8px;
  }

  strong {line-height: 1;}

  table
  {
  border-collapse: separate;
  border-left: white solid 23px;
  border-right: white solid 34px;
  border-spacing: 19px 0px;
  color: white;
  height: 100px;
  table-layout: fixed;
  width: 758px;
  }

  col#first-column {width: 382px;}

  col#second-column {width: 262px;}

  td {padding: 0px 127px;}

  td#FirstTestedCell {border-right: lime solid 1px;}

  /*
  Width of table

  From left to right
  ------------------

    23px (left border of table)
 +
    19px (left-most border-spacing)
 +
   382px (specified width of first column)
 +
    19px (border-spacing separating the 2 cells)
 +
   262px (specified width of second column)
 +
    19px (right-most border-spacing)
 +
    34px (right border of table)
 ========
   758px

  So, here, the specified width of the table is not greater
  than the sum of the column widths (plus cell spacing or borders).
  So, the table's width in 'border-collapse: separate' remains as specified.

  The col#first-column specified width of 382px should include
  the horizontal padding of 127px and the solid lime 1px border-right.

  So, such 1px solid lime border-right should appear at
    23px (left border of table)
 +
    19px (left-most border-spacing)
 +
   382px (specified width of first column)
 ========
   424px inside document body's content box.
  */

  div#reference {margin-left: 23px;}
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
