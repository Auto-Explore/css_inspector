# css/css-flexbox/flexbox-writing-mode-009.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-writing-mode-009.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      width: 40px;
      height: 30px;
      border: 1px solid gray;
      margin-bottom: 5px;
      writing-mode: vertical-rl;
      flex-flow: row wrap;
    }
    .flexContainer > * {
      width: 20px;
      height: 15px;
    }
    .item1 {
      /* Note: flex items are ordered as "CMYK": cyan, magenta, yellow, black */
      background: cyan;
    }
    .item2 {
      background: magenta;
    }
    .item3 {
      background: yellow;
    }
    .item4 {
      background: black;
    }

    /* Classes applied to flex container, to customize its children
     * (which should not affect their sizing):
     */
    .kids_horizontal_tb > * {
      writing-mode: horizontal-tb;
    }
    .kids_vertical_lr > * {
      writing-mode: vertical-lr;
    }
    .kids_vertical_rl > * {
      writing-mode: vertical-rl;
    }

  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
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
