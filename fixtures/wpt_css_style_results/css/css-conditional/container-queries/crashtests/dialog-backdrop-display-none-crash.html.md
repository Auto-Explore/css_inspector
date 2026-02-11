# css/css-conditional/container-queries/crashtests/dialog-backdrop-display-none-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/dialog-backdrop-display-none-crash.html"
}
```

## style[0]

```css

  dialog {
    container-type: size;
    width: 20em;
    height: 3em;
  }

  dialog::backdrop {
    display: none;
  }

  @container (width > 1px) {
    dialog::backdrop {
      background: hotpink;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
