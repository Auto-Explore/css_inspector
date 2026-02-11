# css/css-transforms/backface-visibility-with-sibling-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-with-sibling-001.html"
}
```

## style[0]

```css


div {
  width: 100px;
  height: 100px;
}

#outer {
  transform-style: preserve-3d;
  transform: rotateX(120deg);
  position: relative;
}

#outer > div {
  position: absolute;
  top: 0;
  left: 0;
}

#grandchild {
  transform: translateZ(1px);
}

#child2 {
  transform-style: preserve-3d;
  transform: translateX(0);
  backface-visibility: hidden;
  background: red;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
