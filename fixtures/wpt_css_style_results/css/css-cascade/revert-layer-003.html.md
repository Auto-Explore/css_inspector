# css/css-cascade/revert-layer-003.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-003.html"
}
```

## style[0]

```css

@layer {
  #target {
    width: 100px;
    height: 100px;
    background-color: green;
  }
}

@layer {
  #target {
    width: 200px;
    height: 200px;
    background-color: red;
  }

  #target {
    all: revert-layer;
  }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
