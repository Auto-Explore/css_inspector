# css/css-flexbox/flexbox-align-self-baseline-horiz-007.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-baseline-horiz-007.xhtml"
}
```

## style[0]

```css

      .container {
        display: flex;
        border: 1px dashed blue;
        font: 14px sans-serif;
        height: 50px;
      }

      .base      { align-self: baseline; }
      .lastbase { align-self: last baseline; }

      .offset { margin-top: 10px;
                margin-bottom: 3px; }

      .lime   { background: lime;   }
      .yellow { background: yellow; }
      .orange { background: orange; }
      .pink   { background: pink;   }
    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “align-self”.",
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
