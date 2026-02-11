# css/css-writing-modes/inline-replaced-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/inline-replaced-vrl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      font: 100px/1 Ahem; /* computes to 100px/100px */
      height: 2em;
      writing-mode: vertical-rl;
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
