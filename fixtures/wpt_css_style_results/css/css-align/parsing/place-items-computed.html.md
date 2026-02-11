# css/css-align/parsing/place-items-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-align/parsing/place-items-computed.html"
}
```

## style[0]

```css

  #grandparent {
    justify-items: legacy center;
  }
  #parent {
    justify-items: legacy;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “justify-items”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
