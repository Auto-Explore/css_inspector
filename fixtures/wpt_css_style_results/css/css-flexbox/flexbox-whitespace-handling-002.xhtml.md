# css/css-flexbox/flexbox-whitespace-handling-002.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-whitespace-handling-002.xhtml"
}
```

## style[0]

```css

      div { height: 100px; }
      div.flexbox {
        white-space: pre;
        border: 1px dashed blue;
        width: 200px;
        display: flex;
        justify-content: space-around;
      }
      div.a {
        width: 30px;
        background: lightgreen;
      }
      div.b {
        width: 20px;
        background: lightblue;
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
