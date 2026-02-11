# css/css-backgrounds/border-image-repeat-1.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-repeat-1.html"
}
```

## style[0]

```css

      .outer {
        height: 16px;
        width: 32px;
        border-width: 16px;
        border-style: solid;
        border-image: url("data:image/svg+xml;charset=utf8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='48' height='48'%3E%3Cg fill='blue' stroke-width='0'%3E%3Cpath d='M2 2h4v44H2z'/%3E%3Cpath d='M2 2h44v4H2z'/%3E%3Cpath d='M42 2h4v44h-4z'/%3E%3Cpath d='M2 42h44v4H2zM8'/%3E%3C/g%3E%3C/svg%3E") 16 repeat;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
