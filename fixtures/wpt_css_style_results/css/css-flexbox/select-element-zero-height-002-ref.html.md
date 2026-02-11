# css/css-flexbox/select-element-zero-height-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/select-element-zero-height-002-ref.html"
}
```

## style[0]

```css

  .container {
    height: 0px;
    border: 1px dotted black;
  }
  .with-background {
    width: 100%;
    background: lime;
  }
  select {
    /*
      WebKit applies intrinsic (default) margins to control elements in some circumstances,
      let's disable them to avoid mismatch errors caused by those margins.
    */
    margin: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
