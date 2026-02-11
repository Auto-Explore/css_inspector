# css/css-transforms/perspective-children-only-fixpos.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-children-only-fixpos.html"
}
```

## style[0]

```css


div {
  width: 100px;
  height: 100px;
}

#outer {
  transform: scale(1);
  position: relative;
  background: red;
  perspective: 100px;
}

#middle {
}

#inner {
  transform: translateZ(-100px);
  position: fixed;
  top: 0;
  left: 0;
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
