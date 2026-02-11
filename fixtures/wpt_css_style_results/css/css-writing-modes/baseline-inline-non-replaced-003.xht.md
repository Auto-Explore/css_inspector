# css/css-writing-modes/baseline-inline-non-replaced-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/baseline-inline-non-replaced-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      background-color: red;
      color: green;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      height: 2em; /* computes to 100px */
      writing-mode: vertical-rl;
    }

  span
    {
      border-left: green solid 1em;
      display: inline-block;
      height: 2em;
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
