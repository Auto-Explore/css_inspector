# css/css-shadow/keyframes-002.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/keyframes-002.html"
}
```

## style[0]

```css

@keyframes myanim {
  from { background: red; }
  to { background: red; }
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      #in-shadow {
        width: 100px;
        height: 100px;
        background: blue;
        animation: myanim 10s infinite;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
