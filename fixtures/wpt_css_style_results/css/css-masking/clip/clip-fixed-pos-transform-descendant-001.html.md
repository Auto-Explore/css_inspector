# css/css-masking/clip/clip-fixed-pos-transform-descendant-001.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip/clip-fixed-pos-transform-descendant-001.html"
}
```

## style[0]

```css

html, body { margin: 0; }

#clip {
  height: 100px;
  width: 100px;
  background: lime;
  clip: rect(0, auto, auto, 0);
  position: absolute;
}

#fixed {
  position: fixed;
  top: 0;
  left: 0;
  width: 100px;
  height: 100px;
}

#clipped {
  height: 100px;
  width: 100px;
  background: red;
  transform: translateY(100px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
