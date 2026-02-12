# css/css-values/tree-counting/sibling-index-keyframe-value-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-value-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      z-index: sibling-index();
    }
    to {
      z-index: 1;
    }
  }
  #target {
    animation: --anim 1000s step-end;
    position: relative;
    width: 100px;
    height: 100px;
    background: red;
  }
  #abs {
    position: absolute;
    width: 100px;
    height: 100px;
    z-index: 3;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
