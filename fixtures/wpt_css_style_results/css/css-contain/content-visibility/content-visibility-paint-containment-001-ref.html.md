# css/css-contain/content-visibility/content-visibility-paint-containment-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-paint-containment-001-ref.html"
}
```

## style[0]

```css

  .container {
      width: 100px;
      height: 100px;
      background: green;
  }
  .hidden {
      contain: layout paint style;
  }
  .visible {
      contain: none;
  }
  #overflowing {
      width: 400px;
      height: 100px;
  }
  .square {
      display: inline-block;
      width: 50px;
      height: 50px;
      margin: 5px;
  }
  .red {
      background: red;
  }
  .green {
      background: green;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
