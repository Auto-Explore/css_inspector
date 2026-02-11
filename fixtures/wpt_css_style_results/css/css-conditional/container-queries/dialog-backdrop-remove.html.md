# css/css-conditional/container-queries/dialog-backdrop-remove.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/dialog-backdrop-remove.html"
}
```

## style[0]

```css

  html {
    /* Prevent multiple layout passes due to scrollbars */
    overflow: hidden;
    background: green;
  }
  dialog {
    container-type: size;
    width: 100px;
    height: 100px;
    visibility: hidden;
  }
  dialog::backdrop {
    display: block;
    background-color: red;
  }
  @container (width > 1px) {
    dialog::backdrop {
      display: none;
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
