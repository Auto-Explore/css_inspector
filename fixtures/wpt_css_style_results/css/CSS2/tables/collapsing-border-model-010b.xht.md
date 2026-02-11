# css/CSS2/tables/collapsing-border-model-010b.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/collapsing-border-model-010b.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table
  {
  background-color: red;
  border-collapse: collapse;
  border-spacing: 50px;
  color: green;
  float: left; /* so that the CSS table only uses shrink-to-fit width */
  font: 1em/1 Ahem;
  }

  div#table-header {display: table-header-group;}

  div#table-footer {display: table-footer-group;}

  div#table-body {display: table-row-group;}

  div.table-row {display: table-row;}

  div.table-cell
  {
  background-color: green;
  border: 0.5em solid green;
  display: table-cell;
  padding: 5px;
  }

  div#table-body div.table-cell, div.table-row > div:first-child {padding: 0px;}
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
