# css/css-position/backdrop-inherit-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-position/backdrop-inherit-computed.html"
}
```

## style[0]

```css

  dialog {
    --foo: bar;
    --bg: green;
    left: 100px;
    color-scheme: dark;
  }
  dialog::backdrop {
    background-color: var(--bg);
    left: inherit;
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
