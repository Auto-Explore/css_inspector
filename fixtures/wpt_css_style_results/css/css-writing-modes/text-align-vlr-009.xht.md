# css/css-writing-modes/text-align-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-align-vlr-009.xht"
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
      background: red url("support/left-center-200x300.png");
      direction: ltr;
      text-align: center;
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
