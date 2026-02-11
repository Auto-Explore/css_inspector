# css/css-pseudo/highlight-custom-properties-dynamic-001.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-custom-properties-dynamic-001.html"
}
```

## style[0]

```css

    div {
      --background-color: red;
    }
    ::selection {
      background-color: var(--background-color, red);
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
