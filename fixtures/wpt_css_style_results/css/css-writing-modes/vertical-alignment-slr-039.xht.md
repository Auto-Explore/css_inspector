# css/css-writing-modes/vertical-alignment-slr-039.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-slr-039.xht"
}
```

## style[0]

```css
<![CDATA[
    div#slr
    {
      writing-mode: sideways-lr;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      color: orange;
    }

    span
    {
      vertical-align: sub;
      color: blue;
      margin-bottom: -1em;
    }
    ]]>
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
