# css/css-position/backdrop-inherit-rendered.html

```json
{
  "format_version": 3,
  "file": "css/css-position/backdrop-inherit-rendered.html"
}
```

## style[0]

```css

  dialog {
    --bg: green;
    inset: 100px;
  }
  dialog::backdrop {
    background-color: var(--bg);
    inset: inherit;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
