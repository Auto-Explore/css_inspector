# css/css-ui/cursor-x-y-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/cursor-x-y-001-manual.html"
}
```

## style[0]

```css

body, html {
  cursor: url("support/cursors/arrows.png") 31 31, url("support/cursors/arrows.ico") 31 31, help;
}
div {
  margin: 30px;
  width: 15px;
  height: 15px;
  background: green;
}
#fail, #pass { display: none }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
