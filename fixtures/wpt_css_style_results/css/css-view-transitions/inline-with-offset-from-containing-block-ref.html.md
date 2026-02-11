# css/css-view-transitions/inline-with-offset-from-containing-block-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/inline-with-offset-from-containing-block-ref.html"
}
```

## style[0]

```css

  :root {
    scrollbar-width: none;
    font: 20px/1 Ahem;
  }
  .outer {
    transform: scale3d(2, 2, 1);
    width: 100vw;
    text-align: center;
  }
  .inner {
    transform: translate(10px);
    padding: 10px;
    border: 10px solid black;
  }
  body {
    background: pink;
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
