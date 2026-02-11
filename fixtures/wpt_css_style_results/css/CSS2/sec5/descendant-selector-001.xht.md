# css/CSS2/sec5/descendant-selector-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/sec5/descendant-selector-001.xht"
}
```

## style[0]

```css

body * {border: thin solid orange}
body div {border: none}
div * {border: thin solid blue}
body h4 {border: none}
h4  * {border: thin solid fuchsia}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
