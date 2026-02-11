# css/css-anchor-position/anchor-getComputedStyle-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-getComputedStyle-002.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.rel {
  position: relative;
  background: lightgray;
}

.anchor {
  anchor-name: --a;
  background: orange;
}

.target {
  position: absolute;
  left: anchor(--a left);
  right: anchor(--a right);
  top: anchor(--a top);
  bottom: anchor(--a bottom);
  background: lime;
  opacity: 0.5;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

#test1.multicol {
  column-count: 3;
  column-width: 100px;
  column-gap: 10px;
  width: 320px;
  height: 100px;
}

#test1 .rel{
  width: 100px;
  height: 300px;
}

#test1 .spacer {
  height: 175px;
}

#test1 .anchor {
  margin-left: 25px;
  width: 50px;
  height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
