# css/css-writing-modes/text-indent-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      color: green;
      font: 80px/1 Ahem; /* computes to 80px/80px */
      height: 320px;
      width: 320px;
    }

  div
    {
      background: red url("support/bg-red-4col-2row-320x320.png");
      direction: ltr;
      text-indent: 80px;
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
