# css/css-transitions/pseudo-elements-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/pseudo-elements-002.html"
}
```

## style[0]

```css

  #inner::before {
    content: "This text should transition from red to green.";
    height: 100px;
    transition: height steps(2, start) 1s;
  }
  .flex #inner::before {
    height: 300px;
  }
  .flex { display: flex }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
