# css/css-masking/clip-path/clip-path-inline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-inline-002.html"
}
```

## style[0]

```css

  body {
    overflow: hidden;
  }
  .container {
    writing-mode: vertical-rl;
    padding: 70px 30px;
    margin: -270px -130px;
    font: 100px/1 Ahem;
    line-height: 100px;
    color: transparent;
  }
  .container > span {
    clip-path: polygon(0% 0%, 100% 0%, 100% 50%, 0% 50%);
    color: red;
  }
  .container > span > span {
    color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
