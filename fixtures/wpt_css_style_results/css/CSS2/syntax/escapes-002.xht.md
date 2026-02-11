# css/CSS2/syntax/escapes-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escapes-002.xht"
}
```

## style[0]

```css


  p.class#id { background: green; color: white; }

  p\.class#id { background: red; }

  p.class\#id { background: red; }

  p.class#id { background\: red; }

  p.class#id { background: red\; }

  p.class#id \{ background: red; \}
  p.class#id { background: red; }

  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
