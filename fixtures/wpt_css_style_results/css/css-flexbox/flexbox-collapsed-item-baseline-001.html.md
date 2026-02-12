# css/css-flexbox/flexbox-collapsed-item-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-collapsed-item-baseline-001.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      width: 50px;
      background: yellow;
      border: 1px dotted black;
      margin: 5px;
      align-items: baseline;
    }
    .collapse {
      visibility: collapse;
    }
    .largeFont {
      font-size: 20px;
      background: lightblue;
      /* Our flex items get padding on opposite sides (top/bottom) so that they
         produce a large combined height when baseline-aligned: */
      padding-top: 5px;
    }
    .smallFont {
      font-size: 10px;
      background: pink;
      /* Our flex items get padding on opposite sides (top/bottom) so that they
         produce a large combined height when baseline-aligned: */
      padding-bottom: 20px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
