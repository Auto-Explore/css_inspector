# css/CSS2/tables/fixed-table-layout-027.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-027.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-collapse: collapse;
  table-layout: fixed;
  width: 100px;
  }

  td#middle-green-cell
  {
  background-color: green;
  border-left: green solid 25px;
  border-right: green solid 25px;
  width: 50%;
  }

  td#left-red-cell, td#right-red-cell {background-color: red;}

  td#left-red-cell {border-right: red solid 24px;}

  td#right-red-cell {border-left: red solid 25px;}

  td {padding: 50px 0px;}
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
