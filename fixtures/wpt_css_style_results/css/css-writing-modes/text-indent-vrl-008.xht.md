# css/css-writing-modes/text-indent-vrl-008.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vrl-008.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      color: green;
      font: 80px/1 Ahem; /* computes to 80px/80px */
      height: 320px;
      width: 160px;
    }

  div#containing-block
    {
      background: red url("support/bg-red-4col-3row-320x320.png") top right;
      direction: rtl;
      writing-mode: vertical-rl;
    }

  div#child
    {
      text-indent: 25%;
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
