# css/css-writing-modes/vertical-alignment-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-006.xht"
}
```

## style[0]

```css
<![CDATA[
    div#rl
    {
      color: orange;
      font: 60px/3 Ahem; /* computes to 60px/180px */
      writing-mode: vertical-rl;
    }

    span#orange30
    {
      font-size: 0.5em; /* computes to 30px */
      vertical-align: text-bottom;
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
