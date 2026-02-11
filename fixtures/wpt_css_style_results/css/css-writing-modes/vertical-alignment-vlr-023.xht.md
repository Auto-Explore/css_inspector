# css/css-writing-modes/vertical-alignment-vlr-023.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-vlr-023.xht"
}
```

## style[0]

```css
<![CDATA[
    div#lr-upright
    {
      writing-mode: vertical-lr;
      font: 100px/3 Ahem; /* computes to 100px/300px */
      color: blue;
      text-orientation: upright;
    }

    span
    {
      font-size: 0.2em;
      vertical-align: middle;
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
