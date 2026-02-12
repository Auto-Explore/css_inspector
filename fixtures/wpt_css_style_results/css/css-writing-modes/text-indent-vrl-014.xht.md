# css/css-writing-modes/text-indent-vrl-014.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vrl-014.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
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
      background: red url("support/bg-red-4col-2row-320x320.png") top right;
      direction: ltr;
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
