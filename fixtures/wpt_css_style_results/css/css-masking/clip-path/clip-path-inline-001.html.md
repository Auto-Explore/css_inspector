# css/css-masking/clip-path/clip-path-inline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-inline-001.html"
}
```

## style[0]

```css

  body {
    overflow: hidden;
  }
  .container {
    writing-mode: horizontal-tb;
    padding: 70px 30px;
    margin: -170px -230px;
    font: 100px/1 Ahem;
    line-height: 100px;
    color: transparent;
  }
  .container > span {
    clip-path: polygon(0% 0%, 50% 0%, 50% 100%, 0% 100%);
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
