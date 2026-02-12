# css/css-view-transitions/names-are-tree-scoped.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/names-are-tree-scoped.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: red;
}

#one {
  view-transition-name: light1;
}
#two {
  view-transition-name: light2;
}
#three {
  view-transition-name: light3;
}

:root { view-transition-name: none; }
html::view-transition-group(*) { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 0 }
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

    div {
      width: 100px;
      height: 100px;
      background: green;
      view-transition-name: shadow;
    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
