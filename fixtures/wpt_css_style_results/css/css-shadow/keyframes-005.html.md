# css/css-shadow/keyframes-005.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/keyframes-005.html"
}
```

## style[0]

```css

  #host {
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

## style[1]

```css

        @keyframes myanim {
          from { background: red; }
          to { background: green; }
        }
      
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
