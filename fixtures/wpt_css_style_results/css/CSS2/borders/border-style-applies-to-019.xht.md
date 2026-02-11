# css/CSS2/borders/border-style-applies-to-019.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/borders/border-style-applies-to-019.xht"
}
```

## style[0]

```css
<![CDATA[
  #table
  {
  border-collapse: collapse;
  display: table;
  table-layout: fixed;
  width: 200px;
  }

  #test
  {
  border-color: red;
  border-style: hidden;
  display: table-column-group;
  }

  .col {display: table-column;}

  .row
  {
  display: table-row;
  }

  .cell
  {
  display: table-cell;
  height: 100px;
  }
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
