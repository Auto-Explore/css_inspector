# css/css-properties-values-api/registered-property-change-style-002.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-change-style-002.html"
}
```

## style[0]

```css

      .failure {
          background: pink;
      }
      #visibility {
          visibility: var(--my-visibility, visible);
      }
      #display {
          display: var(--my-display, inline-block);
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
