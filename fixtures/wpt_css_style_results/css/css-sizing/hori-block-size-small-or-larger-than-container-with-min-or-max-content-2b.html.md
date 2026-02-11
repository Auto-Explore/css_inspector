# css/css-sizing/hori-block-size-small-or-larger-than-container-with-min-or-max-content-2b.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/hori-block-size-small-or-larger-than-container-with-min-or-max-content-2b.html"
}
```

## style[0]

```css

    html,body {
      margin: 0;
    }

    .outer * {
      border: 2px solid lime;
    }

    .container {
      width: 80px;
      border-color: blue;
      display: inline-block;
      vertical-align: top;

      font-family: Ahem;
      font-size: 8px;
      line-height: 13px;
    }

    .too-small {
      block-size: 10px;
    }

    .too-big {
      block-size: 100px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
