# css/css-writing-modes/vertical-alignment-vlr-015.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-vlr-015.xht"
}
```

## style[0]

```css
<![CDATA[
    div#lr-sideways
    {
      writing-mode: vertical-lr;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      color: orange;
      text-orientation: sideways;
    }

    span
    {
      vertical-align: super;
      color: blue;
      margin-top: -1em;
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
