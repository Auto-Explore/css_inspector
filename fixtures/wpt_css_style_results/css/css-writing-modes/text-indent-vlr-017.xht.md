# css/css-writing-modes/text-indent-vlr-017.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vlr-017.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  div#containing-block
    {
      color: green;
      font: 80px/1 Ahem; /* computes to 80px/80px */
      height: 320px;
      width: 160px;
    }

  div#containing-block
    {
      background: red url("support/bg-red-1col-3row-320x320.png");
      direction: rtl;
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
