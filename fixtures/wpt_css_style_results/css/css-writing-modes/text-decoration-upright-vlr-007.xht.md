# css/css-writing-modes/text-decoration-upright-vlr-007.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-decoration-upright-vlr-007.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      color: blue;
      font: 2.5em/1.5 serif; /* computes to 40px/60px */
      text-decoration: line-through;
      text-orientation: upright;
      writing-mode: vertical-lr;
    }

  span
    {
      color: orange;
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
