# css/css-shadow/part/animation-part.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/animation-part.html"
}
```

## style[0]

```css

        @keyframes green {
          from { background: green; }
          to { background: green; }
        }

        display-el::part(icon) {
          animation: green 1s linear infinite;
        }
      
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

        div {
          width: 16px; height: 16px; background: red;
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
