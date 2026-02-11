# css/css-sizing/vert-block-size-small-or-larger-than-container-with-min-or-max-content-2b.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/vert-block-size-small-or-larger-than-container-with-min-or-max-content-2b.html"
}
```

## style[0]

```css

    html,body {
      margin: 0;
    }

    .outer {
      writing-mode: vertical-lr;
      display: inline-block;
    }

    .outer * {
      border: 2px solid lime;
      display: inline-block;
    }

    .container {
      height: 80px;
      border-color: blue;
      vertical-align: bottom;

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
