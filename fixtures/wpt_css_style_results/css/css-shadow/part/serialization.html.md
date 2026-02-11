# css/css-shadow/part/serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/serialization.html"
}
```

## style[0]

```css

      ::part(\(foo) {}
      ::part(   bar\    ) {}
      ::part( -foo  bar    ) {}
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
