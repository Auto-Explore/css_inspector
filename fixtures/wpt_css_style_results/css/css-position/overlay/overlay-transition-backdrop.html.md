# css/css-position/overlay/overlay-transition-backdrop.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition-backdrop.html"
}
```

## style[0]

```css

  [popover] {
    display: block;
    visibility: hidden;
    transition: overlay 60s step-end;
    transition-behavior: allow-discrete;
  }
  [popover]::backdrop {
    visibility: visible;
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
