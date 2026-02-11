# css/css-writing-modes/vertical-alignment-srl-040.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/vertical-alignment-srl-040.xht"
}
```

## style[0]

```css
<![CDATA[
    div#srl
    {
      writing-mode: sideways-rl;
      font: 100px/3 Ahem; /* computes to 100px/300px */
      color: blue;
    }

    span
    {
      font-size: 0.8em;
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
