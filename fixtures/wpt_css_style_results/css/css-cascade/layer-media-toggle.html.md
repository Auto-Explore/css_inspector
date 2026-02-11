# css/css-cascade/layer-media-toggle.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/layer-media-toggle.html"
}
```

## style[0]

```css

@layer foo, bar;
@layer bar {
  #target { background-color: green; }
}
@layer foo {
  #target { background-color: red; }
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
