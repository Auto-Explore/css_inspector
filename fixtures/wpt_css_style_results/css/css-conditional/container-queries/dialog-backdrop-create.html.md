# css/css-conditional/container-queries/dialog-backdrop-create.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/dialog-backdrop-create.html"
}
```

## style[0]

```css

  html {
    /* Prevent multiple layout passes due to scrollbars */
    overflow: hidden;
    background: red;
  }
  dialog {
    container-type: size;
    width: 100px;
    height: 100px;
    visibility: hidden;
  }
  dialog::backdrop {
    visibility: visible;
    display: none;
    background-color: green;
  }
  @container (width > 1px) {
    dialog::backdrop {
      display: block;
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
