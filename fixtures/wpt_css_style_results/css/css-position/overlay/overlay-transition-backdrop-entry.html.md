# css/css-position/overlay/overlay-transition-backdrop-entry.html

```json
{
  "format_version": 3,
  "file": "css/css-position/overlay/overlay-transition-backdrop-entry.html"
}
```

## style[0]

```css

  body {
    background-color: green;
  }
  [popover] {
    display: block;
    visibility: hidden;
    transition-delay: 2s;
    transition-property: overlay;
    transition-duration: 2s;
    transition-behavior: allow-discrete;
  }
  [popover]::backdrop {
    background-color: blue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
