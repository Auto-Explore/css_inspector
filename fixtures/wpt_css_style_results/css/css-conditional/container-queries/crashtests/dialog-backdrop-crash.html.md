# css/css-conditional/container-queries/crashtests/dialog-backdrop-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/dialog-backdrop-crash.html"
}
```

## style[0]

```css

  html {
    overflow: hidden;
  }

  dialog {
    container-type: size;
    width: 100px;
    height: 100px;
  }

  @container (width > 1px) {
    dialog::backdrop {
      margin: 10px;
      background-color: green;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
