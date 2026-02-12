# css/css-writing-modes/text-indent-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vlr-009.xht"
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
      background: red url("support/bg-red-1col-3row-320x320.png");
      direction: rtl;
      writing-mode: vertical-lr;
    }

  div#child
    {
      text-indent: 25%;
    }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
