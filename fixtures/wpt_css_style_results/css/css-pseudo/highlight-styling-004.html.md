# css/css-pseudo/highlight-styling-004.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-styling-004.html"
}
```

## style[0]

```css

  :root {
    color-scheme: light dark;
  }

  .light {
    color-scheme: light;
  }

  .dark {
    color-scheme: dark;
  }

  p::selection {
    color: light-dark(green, blue);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color-scheme”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
