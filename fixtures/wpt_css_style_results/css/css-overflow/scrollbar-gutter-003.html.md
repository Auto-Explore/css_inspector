# css/css-overflow/scrollbar-gutter-003.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-003.html"
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
    resize: both;
    /* This test is same as scrollbar-gutter-002, except for next line */
    will-change: transform;
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
