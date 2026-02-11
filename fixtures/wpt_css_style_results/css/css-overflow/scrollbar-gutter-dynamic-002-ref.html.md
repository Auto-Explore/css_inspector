# css/css-overflow/scrollbar-gutter-dynamic-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-dynamic-002-ref.html"
}
```

## style[0]

```css

  .line {
    display: flex;
  }

  .container {
    writing-mode: horizontal-tb;
    direction: ltr;

    block-size: 200px;
    inline-size: 200px;
    margin: 10px;
    background: deepskyblue;
  }

  .content {
    inline-size: 100%;
    block-size: 200%;
    background: lightsalmon;
  }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
