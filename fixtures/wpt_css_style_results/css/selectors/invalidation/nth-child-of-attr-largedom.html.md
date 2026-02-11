# css/selectors/invalidation/nth-child-of-attr-largedom.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/nth-child-of-attr-largedom.html"
}
```

## style[0]

```css

  tr:nth-child(even of :not([hidden])){
    background: lightgrey;
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
