# css/css-scrollbars/scrollbar-color-012.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/scrollbar-color-012.html"
}
```

## style[0]

```css

  html {scrollbar-color: transparent transparent;}
  body {height: 200vh}
  .container {
    overflow: auto;
    height: 200px;
    width: 200px;
    background-color: red;
  }

  .content {
    height: 300px;
    width: 300px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scrollbar-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
