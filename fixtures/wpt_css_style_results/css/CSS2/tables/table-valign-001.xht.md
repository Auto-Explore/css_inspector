# css/CSS2/tables/table-valign-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/table-valign-001.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      background: red;
      float: left; /* shrinkwrap width and height */
    }
    .control {
       width: 10px;
       height: 100px;
       background: blue;
    }
    table, .control {
      font-size: 20px;
      float: left;
      border-spacing: 0;
    }
    td {
      background: blue;
      color: blue;
      padding: 0;
    }

    .middle td { vertical-align: middle; }
    .baseline td { vertical-align: baseline; }
    .one { height: 100px; }
    .two { font-size: 2em; }
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
