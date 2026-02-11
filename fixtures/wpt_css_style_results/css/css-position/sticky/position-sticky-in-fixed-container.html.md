# css/css-position/sticky/position-sticky-in-fixed-container.html

```json
{
  "format_version": 3,
  "file": "css/css-position/sticky/position-sticky-in-fixed-container.html"
}
```

## style[0]

```css

  .modal {
    bottom: 0;
    left: 0;
    right: 0;
    position: fixed;
  }

  .modal-dialog {
    position: relative;
    overflow: auto;
    max-height: 400px;
    transform: translateY(0);
  }

  .modal-content {
    background-color: purple;
    height: 400px;
  }

  .modal-footer {
    height: 100px;
    background-color: blue;

    position:sticky;
    bottom: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
