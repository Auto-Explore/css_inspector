# css/css-writing-modes/text-indent-vrl-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vrl-010.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

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
