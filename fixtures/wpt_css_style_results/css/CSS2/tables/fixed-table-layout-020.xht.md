# css/CSS2/tables/fixed-table-layout-020.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-020.xht"
}
```

## style[0]

```css
<![CDATA[
  table, div {font: 1.25em/1 Ahem;}

  table
  {
  border: white solid;
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

  td {padding: 1px 30px;}

  td#third-cell {color: orange;}

  div#reference
  {
  background-color: blue;
  color: blue;
  left: 172px;
  padding: 1px 0px;
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
