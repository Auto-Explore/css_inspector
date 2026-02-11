# css/CSS2/backgrounds/background-position-applies-to-001d.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-001d.xht"
}
```

## style[0]

```css
<![CDATA[
  div#table {display: table;}

  div#tbody
  {
  background-image: url("/css/support/60x60-red.png");
  background-position: bottom right;
  background-repeat: no-repeat;
  display: table-row-group;
  }

  div.tr {display: table-row;}

  div.td
  {
  display: table-cell;
  width: 60px;
  }

  div#top-left
  {
  border-top: transparent solid 60px;
  display: table-cell;
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
