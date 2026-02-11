# css/css-writing-modes/forms/input-range-zero-inline-size.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/forms/input-range-zero-inline-size.html"
}
```

## style[0]

```css

        div {
            display: flex;
            background-color: red;
            border-inline-start: 1px solid green;
        }
        .horizontal {
            background-color: transparent;
        }
        input[type=range] {
            visibility: hidden;
            inline-size: 0;
            margin: 0;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-inline-start”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
