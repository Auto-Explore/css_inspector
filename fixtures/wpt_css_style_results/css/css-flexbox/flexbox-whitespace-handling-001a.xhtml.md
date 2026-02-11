# css/css-flexbox/flexbox-whitespace-handling-001a.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-whitespace-handling-001a.xhtml"
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
        flex: none;
        width: 30px;
        background: lightgreen;
      }
      div.b {
        flex: none;
        width: 20px;
        background: lightblue;
      }
      img { width: 40px; }
    
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
