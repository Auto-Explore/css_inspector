# css/CSS2/backgrounds/background-position-applies-to-013d.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-013d.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table
  {
  background-image: url("/css/support/60x60-red.png");
  background-position: bottom right;
  background-repeat: no-repeat;
  display: table;
  }

  div#tbody {display: table-row-group;}

  div.tr {display: table-row;}

  div#top-left
  {
  border-top: transparent solid 60px;
  display: table-cell;
  }

  div.td
  {
  display: table-cell;
  width: 60px;
  }

  div#green-overlapping
  {
  border-bottom: green solid 60px;
  display: table-cell;
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
