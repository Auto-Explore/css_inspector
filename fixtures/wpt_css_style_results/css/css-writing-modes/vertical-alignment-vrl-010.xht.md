# css/css-writing-modes/vertical-alignment-vrl-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-vrl-010.xht"
}
```

## style[0]

```css
<![CDATA[
    div#rl-mixed
    {
      writing-mode: vertical-rl;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      color: orange;
      text-orientation: mixed;
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
