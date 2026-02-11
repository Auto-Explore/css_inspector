# css/css-values/rlh-unit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/rlh-unit-001.html"
}
```

## style[0]

```css

  :root {
    line-height: 50px;
  }

  p {
    line-height: normal;
  }

  .container {
    position: relative;
    width: 100px;
    height: 100px;
  }

  .contents {
    position: absolute;
    top: 0;
    left: 0;
  }

  .div1 {
    background-color: green;
    width: 100px;
    height: 100px;
  }

  .div2 {
    line-height: 10px;
    block-size: 100px;
    background-color: red;
    inline-size: calc(1rlh - 1rlh);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
