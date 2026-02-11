# css/css-flexbox/flexbox-align-self-baseline-horiz-008.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-baseline-horiz-008.xhtml"
}
```

## style[0]

```css

      .container {
        float: left;
        display: flex;
        writing-mode: sideways-rl;
        border: 1px dashed blue;
        font: 14px sans-serif;
        width: 80px;
      }

      .reverse { flex-flow: row wrap-reverse; }

      .base     { align-self: baseline; }
      .lastbase { align-self: last baseline; }

      .offset { margin-right: 10px;
                margin-left: 3px; }

      .lime   { background: lime;   }
      .yellow { background: yellow; }
      .orange { background: orange; }
      .pink   { background: pink;   }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
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
