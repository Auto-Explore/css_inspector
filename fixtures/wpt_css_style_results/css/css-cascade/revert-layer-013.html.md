# css/css-cascade/revert-layer-013.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-013.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  background-color: revert-layer;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

@layer first {
    :host { background-color: green; }
}
@layer second {
    :host { background-color: revert-layer; }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
