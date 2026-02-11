# css/css-writing-modes/text-baseline-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-baseline-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
    div#rl-mixed
    {
      color: orange;
      font: 60px/1.5 Ahem; /* computes to 60px/90px */
      writing-mode: vertical-rl;
      text-orientation: mixed;
    }

    span#blue120
    {
      color: blue;
      font-size: 2em; /* computes to 120px */
    }

    span#orange30
    {
      font-size: 0.5em; /* computes to 30px */
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
