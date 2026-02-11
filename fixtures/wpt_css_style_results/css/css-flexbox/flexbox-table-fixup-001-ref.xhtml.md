# css/css-flexbox/flexbox-table-fixup-001-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-table-fixup-001-ref.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        border: 1px dashed blue;
        width: 200px;
        display: flex;
        justify-content: space-around;
      }

      .a {
        background: lightgreen;
        width: 48px;
      }

      .b {
        background: yellow;
        width: 48px;
      }

      .c {
        background: pink;
        width: 48px;
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
