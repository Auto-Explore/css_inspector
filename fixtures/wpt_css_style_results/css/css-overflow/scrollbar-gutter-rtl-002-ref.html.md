# css/css-overflow/scrollbar-gutter-rtl-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-rtl-002-ref.html"
}
```

## style[0]

```css

  .line {
    display: flex;
  }

  .container {
    writing-mode: horizontal-tb;
    direction: rtl;

    box-sizing: border-box;
    block-size: 200px;
    inline-size: 200px;
    margin: 10px;
    background: deepskyblue;
    resize: both;
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
