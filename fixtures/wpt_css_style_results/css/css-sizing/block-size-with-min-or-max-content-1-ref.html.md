# css/css-sizing/block-size-with-min-or-max-content-1-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/block-size-with-min-or-max-content-1-ref.html"
}
```

## style[0]

```css

    html,body {
      margin: 0;
    }
    .container {
      width: max-content;
      height: 50px;
      border: 1px solid black;

      font-family: Ahem;
      font-size: 15px;
      line-height: 21px;
    }
    .container > * {
      display: inline-block;
      border: 1px solid blue;
    }

    .container-vertical {
      width: 50px;
      border: 1px solid black;

      font-family: Ahem;
      font-size: 15px;
      line-height: 21px;
    }
    .container-vertical > * {
      writing-mode: vertical-lr;
      border: 1px solid blue;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
