# css/CSS2/visuren/clear-applies-to-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visuren/clear-applies-to-016.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 8px;}

  p
  {
  float: left;
  line-height: 1.25;
  margin: 1em 0;
  width: 320px;
  }

  div#table
  {
  background-color: blue;
  clear: both;
  display: table;
  table-layout: fixed;
  width: 1in;
  }

  div#caption
  {
  background-color: blue;
  caption-side: top;
  color: blue;
  display: table-caption;
  height: 0.5in;
  width: 1in;
  }

  div.row {display: table-row;}

  div.cell
  {
  color: blue;
  display: table-cell;
  height: 0.25in;
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
