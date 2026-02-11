# css/CSS2/backgrounds/background-position-applies-to-003c.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-applies-to-003c.xht"
}
```

## style[0]

```css
<![CDATA[
  table {border-spacing: 0px;}

  tfoot
  {
  background-image: url("/css/support/60x60-red.png");
  background-position: bottom right;
  background-repeat: no-repeat;
  }

  td
  {
  padding: 0px;
  width: 60px;
  }

  td#top-left {border-top: transparent solid 60px;}

  td#green-overlapping {border-bottom: green solid 60px;}
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
