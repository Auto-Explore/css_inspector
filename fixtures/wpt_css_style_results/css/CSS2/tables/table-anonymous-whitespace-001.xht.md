# css/CSS2/tables/table-anonymous-whitespace-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-anonymous-whitespace-001.xht"
}
```

## style[0]

```css
<![CDATA[
    /* Must use <div> tables for this test because parsers do weird things
       to the contents of <table>-related elements. */
    .pre {
       white-space: pre;
    }
    .table, table {
       display: table;
       border: solid silver;
       border-spacing: 1px;
       margin: 0.5em;
     }
    .row {
       display: table-row;
    }
    .cell, td {
       display: table-cell;
       border: solid blue;
    }
    table, td {
      padding: 0;
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
