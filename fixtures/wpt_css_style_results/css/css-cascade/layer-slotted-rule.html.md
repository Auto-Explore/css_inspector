# css/css-cascade/layer-slotted-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/layer-slotted-rule.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
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

@layer {
  ::slotted(*) {
    background-color: green !important;
  }
}
::slotted(*) {
  background-color: red !important;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
