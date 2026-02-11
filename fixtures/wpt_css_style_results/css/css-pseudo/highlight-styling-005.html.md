# css/css-pseudo/highlight-styling-005.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-styling-005.html"
}
```

## style[0]

```css

    :root {
      --bg: red;
    }
    main::selection {
      background-color: var(--bg, red);
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
