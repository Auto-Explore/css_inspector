# css/css-writing-modes/direction-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/direction-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      color: green;
      font: 100px/1 Ahem; /* computes to 100px/100px */
      height: 300px;
      width: 200px;
    }

  div
    {
      background: red url("support/left-top-200x300.png");
      direction: ltr;
      writing-mode: vertical-lr;
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
