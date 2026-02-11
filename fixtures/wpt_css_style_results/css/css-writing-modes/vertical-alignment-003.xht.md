# css/css-writing-modes/vertical-alignment-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-003.xht"
}
```

## style[0]

```css
<![CDATA[
    div#lr
    {
      writing-mode: vertical-lr;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      color: blue;
    }

    span#orange
    {
      font-size: 0.5em;
      color: orange;
      vertical-align: top;
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
