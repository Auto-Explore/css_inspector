# css/css-flexbox/flexbox-collapsed-item-baseline-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-collapsed-item-baseline-001-ref.html"
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
      align-items: flex-start;
    }
    .hiddenItemForSizing {
      width: 0;
      min-width: 0; /* disable default min-width:auto behavior */
      color: transparent;
      align-self: baseline;
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
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
